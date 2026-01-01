/**
 * @file Implements the Lempel-Ziv-like compression logic.
 */

import * as config from "./config";
import type { BufferType } from "./util";
import * as util from "./util";

/**
 * Options for the Compressor constructor.
 */
interface CompressorOptions {
	/**
	 * A callback function that receives compressed data chunks.
	 * If provided, the final result will not be returned as a single string.
	 */
	onData?: (chunk: string) => void;
	/**
	 * A callback function that is called when compression is complete.
	 */
	onEnd?: () => void;
}

/**
 * A class for compressing string data.
 */
export default class Compressor {
	private _data: string | null = null;
	private _table: BufferType | null = null;
	private _result: string | null = null;
	private _onDataCallback?: (chunk: string) => void;
	private _onEndCallback?: () => void;
	private _offset: number = 0;
	private _dataLen: number = 0;
	private _index: number = 0;
	private _length: number = 0;

	/**
	 * @param {CompressorOptions} [options] - Compression options.
	 */
	constructor(options?: CompressorOptions) {
		this._init(options);
	}

	/**
	 * Initializes or re-initializes the compressor's state.
	 * @private
	 * @param {CompressorOptions} [options] - Compression options.
	 */
	private _init(options?: CompressorOptions): void {
		options = options || {};
		this._data = null;
		this._table = null;
		this._result = null;
		this._onDataCallback = options.onData;
		this._onEndCallback = options.onEnd;
	}

	/**
	 * Creates the base62 lookup table for compression.
	 * @private
	 * @returns {BufferType} The character code table.
	 */
	private _createTable(): BufferType {
		const table = util.createBuffer(8, config.TABLE_LENGTH);
		for (let i = 0; i < config.TABLE_LENGTH; i++) {
			table[i] = config.BASE62TABLE.charCodeAt(i);
		}
		return table;
	}

	/**
	 * Handles a chunk of compressed data.
	 * Either calls the onData callback or appends to the internal result string.
	 * @private
	 * @param {BufferType} buffer - The buffer containing the data chunk.
	 * @param {number} length - The length of the data in the buffer.
	 */
	private _onData(buffer: BufferType, length: number): void {
		const chunk = util.bufferToString_fast(buffer, length);

		if (this._onDataCallback) {
			this._onDataCallback(chunk);
		} else if (this._result !== null) {
			this._result += chunk;
		}
	}

	/**
	 * Finalizes the compression process.
	 * @private
	 */
	private _onEnd(): void {
		if (this._onEndCallback) {
			this._onEndCallback();
		}
		this._data = this._table = null;
	}

	/**
	 * Searches for the longest matching string in the sliding window.
	 * @private
	 * @returns {boolean} `true` if a match was found, otherwise `false`.
	 */
	private _search(): boolean {
		let i = 2;
		const data = this._data!;
		const offset = this._offset;
		let len = config.BUFFER_MAX;
		if (this._dataLen - offset < len) {
			len = this._dataLen - offset;
		}
		if (i > len) {
			return false;
		}

		const pos = offset - config.WINDOW_BUFFER_MAX;
		const win = data.substring(pos, offset + len);
		const limit = offset + i - 3 - pos;
		let j, s, index, lastIndex, bestIndex, winPart;

		do {
			if (i === 2) {
				s = data.charAt(offset) + data.charAt(offset + 1);
				index = win.indexOf(s);
				if (index === -1 || index > limit) {
					break;
				}
			} else if (i === 3) {
				s = s + data.charAt(offset + 2);
			} else {
				s = data.substr(offset, i);
			}

			if (config.STRING_LASTINDEXOF_BUG) {
				winPart = data.substring(pos, offset + i - 1);
				lastIndex = winPart.lastIndexOf(s);
			} else {
				lastIndex = win.lastIndexOf(s, limit);
			}

			if (lastIndex === -1) {
				break;
			}

			bestIndex = lastIndex;
			j = pos + lastIndex;
			do {
				if (data.charCodeAt(offset + i) !== data.charCodeAt(j + i)) {
					break;
				}
			} while (++i < len);

			if (index === lastIndex) {
				i++;
				break;
			}
		} while (++i < len);

		if (i === 2) {
			return false;
		}

		this._index = config.WINDOW_BUFFER_MAX - bestIndex!;
		this._length = i - 1;
		return true;
	}

	/**
	 * Compresses the input data string.
	 * @param {string | null} data - The string data to compress.
	 * @returns {string} The compressed data as a base62 encoded string.
	 */
	public compress(data: string | null): string {
		if (data == null || data.length === 0) {
			return "";
		}

		let result = "";
		const table = this._createTable();
		let win = util.createWindow();
		const buffer = util.createBuffer(8, config.COMPRESS_CHUNK_SIZE);
		let i = 0;

		this._result = "";
		this._offset = win.length;
		this._data = win + data;
		this._dataLen = this._data.length;

		let index = -1;
		let lastIndex = -1;
		let c: number, c1: number, c2: number, c3: number, c4: number;

		while (this._offset < this._dataLen) {
			if (!this._search()) {
				c = this._data.charCodeAt(this._offset++);
				if (c < config.LATIN_BUFFER_MAX) {
					if (c < config.UNICODE_CHAR_MAX) {
						c1 = c;
						index = config.LATIN_INDEX;
					} else {
						c1 = c % config.UNICODE_CHAR_MAX;
						c2 = (c - c1) / config.UNICODE_CHAR_MAX;
						index = c2 + config.LATIN_INDEX;
					}

					if (lastIndex === index) {
						buffer[i++] = table[c1];
					} else {
						buffer[i++] = table[index - config.LATIN_INDEX_START];
						buffer[i++] = table[c1];
						lastIndex = index;
					}
				} else {
					if (c < config.UNICODE_BUFFER_MAX) {
						index = config.UNICODE_INDEX;
						c1 = c;
					} else {
						c1 = c % config.UNICODE_BUFFER_MAX;
						c2 = (c - c1) / config.UNICODE_BUFFER_MAX;
						index = c2 + config.UNICODE_INDEX;
					}

					if (c1 < config.UNICODE_CHAR_MAX) {
						c3 = c1;
						c4 = 0;
					} else {
						c3 = c1 % config.UNICODE_CHAR_MAX;
						c4 = (c1 - c3) / config.UNICODE_CHAR_MAX;
					}

					if (lastIndex === index) {
						buffer[i++] = table[c3];
						buffer[i++] = table[c4];
					} else {
						buffer[i++] = table[config.CHAR_START];
						buffer[i++] = table[index - config.TABLE_LENGTH];
						buffer[i++] = table[c3];
						buffer[i++] = table[c4];
						lastIndex = index;
					}
				}
			} else {
				if (this._index < config.BUFFER_MAX) {
					c1 = this._index;
					c2 = 0;
				} else {
					c1 = this._index % config.BUFFER_MAX;
					c2 = (this._index - c1) / config.BUFFER_MAX;
				}

				if (this._length === 2) {
					buffer[i++] = table[c2 + config.COMPRESS_FIXED_START];
					buffer[i++] = table[c1];
				} else {
					buffer[i++] = table[c2 + config.COMPRESS_START];
					buffer[i++] = table[c1];
					buffer[i++] = table[this._length];
				}

				this._offset += this._length;
				if (~lastIndex) {
					lastIndex = -1;
				}
			}

			if (i >= config.COMPRESS_CHUNK_MAX) {
				this._onData(buffer, i);
				i = 0;
			}
		}

		if (i > 0) {
			this._onData(buffer, i);
		}

		this._onEnd();
		result = this._result;
		this._result = null;
		return result === null ? "" : result;
	}
}
