/**
 * @file Implements the decompression logic.
 */

import * as config from "./config";
import * as util from "./util";

/**
 * Options for the Decompressor constructor.
 */
interface DecompressorOptions {
	/**
	 * A callback function that receives decompressed data chunks.
	 */
	onData?: (chunk: string) => void;
	/**
	 * A callback function that is called when decompression is complete.
	 */
	onEnd?: () => void;
}

/**
 * A class for decompressing string data.
 */
export default class Decompressor {
	private _result: number[] | null = null;
	private _onDataCallback?: (chunk: string) => void;
	private _onEndCallback?: () => void;

	/**
	 * @param {DecompressorOptions} [options] - Decompression options.
	 */
	constructor(options?: DecompressorOptions) {
		this._init(options);
	}

	/**
	 * Initializes or re-initializes the decompressor's state.
	 * @private
	 * @param {DecompressorOptions} [options] - Decompression options.
	 */
	private _init(options?: DecompressorOptions): void {
		options = options || {};

		this._result = null;
		this._onDataCallback = options.onData;
		this._onEndCallback = options.onEnd;
	}

	/**
	 * Creates a reverse lookup table from the base62 character set.
	 * @private
	 * @returns {{ [char: string]: number }} The reverse lookup table.
	 */
	private _createTable(): { [char: string]: number } {
		const table: { [char: string]: number } = {};
		for (let i = 0; i < config.TABLE_LENGTH; i++) {
			table[config.BASE62TABLE.charAt(i)] = i;
		}
		return table;
	}

	/**
	 * Handles a chunk of decompressed data.
	 * @private
	 * @param {boolean} [ended=false] - `true` if this is the final chunk.
	 */
	private _onData(ended: boolean = false): void {
		if (!this._onDataCallback || !this._result) {
			return;
		}

		let chunk: number[];
		if (ended) {
			chunk = this._result;
			this._result = [];
		} else {
			const len = config.DECOMPRESS_CHUNK_SIZE - config.WINDOW_MAX;
			chunk = this._result.slice(config.WINDOW_MAX, config.WINDOW_MAX + len);
			this._result = this._result.slice(0, config.WINDOW_MAX).concat(this._result.slice(config.WINDOW_MAX + len));
		}

		if (chunk.length > 0) {
			this._onDataCallback(util.bufferToString_fast(chunk));
		}
	}

	/**
	 * Finalizes the decompression process.
	 * @private
	 */
	private _onEnd(): void {
		if (this._onEndCallback) {
			this._onEndCallback();
		}
	}

	/**
	 * Decompresses a base62 encoded string.
	 * @param {string | null} data - The compressed data string.
	 * @returns {string} The original, decompressed string.
	 */
	public decompress(data: string | null): string {
		if (data == null || data.length === 0) {
			return "";
		}

		this._result = util.stringToArray(util.createWindow());
		let result = "";
		const table = this._createTable();

		let out = false;
		let index: number | null = null;
		const len = data.length;
		let offset = 0;

		let i: number, c: number, c2: number, c3: number;
		let code: number, pos: number, length: number, sub: number[], subLen: number, expandLen: number;

		for (; offset < len; offset++) {
			c = table[data.charAt(offset)];
			if (c === undefined) {
				continue;
			}

			if (c < config.DECODE_MAX) {
				if (!out) {
					// Latin index
					code = index! * config.UNICODE_CHAR_MAX + c;
				} else {
					// Unicode index
					c3 = table[data.charAt(++offset)];
					code = c3 * config.UNICODE_CHAR_MAX + c + config.UNICODE_BUFFER_MAX * index!;
				}
				this._result[this._result.length] = code;
			} else if (c < config.LATIN_DECODE_MAX) {
				// Latin starting point
				index = c - config.DECODE_MAX;
				out = false;
			} else if (c === config.CHAR_START) {
				// Unicode starting point
				c2 = table[data.charAt(++offset)];
				index = c2 - 5;
				out = true;
			} else if (c < config.COMPRESS_INDEX) {
				c2 = table[data.charAt(++offset)];

				if (c < config.COMPRESS_FIXED_START) {
					pos = (c - config.COMPRESS_START) * config.BUFFER_MAX + c2;
					length = table[data.charAt(++offset)];
				} else {
					pos = (c - config.COMPRESS_FIXED_START) * config.BUFFER_MAX + c2;
					length = 2;
				}

				sub = this._result.slice(-pos);
				if (sub.length > length) {
					sub.length = length;
				}
				subLen = sub.length;

				if (sub.length > 0) {
					expandLen = 0;
					while (expandLen < length) {
						for (i = 0; i < subLen; i++) {
							this._result[this._result.length] = sub[i];
							if (++expandLen >= length) {
								break;
							}
						}
					}
				}
				index = null;
			}

			if (this._result.length >= config.DECOMPRESS_CHUNK_MAX) {
				this._onData();
			}
		}

		this._result = this._result.slice(config.WINDOW_MAX);
		this._onData(true);
		this._onEnd();

		result = util.bufferToString_fast(this._result);
		this._result = null;
		return result;
	}
}
