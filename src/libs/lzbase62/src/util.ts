/**
 * @file Utility functions for buffer and string manipulation.
 */

import * as config from "./config";

const fromCharCode = String.fromCharCode;

/**
 * A type representing a buffer that can be created.
 */
export type BufferType = Uint8Array | Uint16Array | number[];

/**
 * Creates a buffer of a specified size and bit type.
 * Prefers TypedArrays if available, otherwise falls back to a regular Array.
 * @param {8 | 16} bits - The number of bits per element (8 or 16).
 * @param {number} size - The size of the buffer.
 * @returns {BufferType} The created buffer.
 */
export function createBuffer(bits: 8 | 16, size: number): BufferType {
	if (config.HAS_TYPED) {
		switch (bits) {
			case 8:
				return new Uint8Array(size);
			case 16:
				return new Uint16Array(size);
		}
	}
	return new Array(size);
}

/**
 * A type that can be truncated.
 */
type TruncatableBuffer = {
	length: number;
	subarray?: (begin: number, end?: number) => TruncatableBuffer;
};

/**
 * Truncates a buffer to a specified length.
 * Uses `subarray` for performance if available.
 * @template T
 * @param {T} buffer - The buffer to truncate.
 * @param {number} length - The desired new length.
 * @returns {T} The truncated buffer.
 */
export function truncateBuffer<T extends TruncatableBuffer>(buffer: T, length: number): T {
	if (buffer.length === length) {
		return buffer;
	}

	if (buffer.subarray) {
		return buffer.subarray(0, length) as T;
	}

	buffer.length = length;
	return buffer;
}

/**
 * Converts a buffer to a string quickly, using `String.fromCharCode.apply`.
 * This method is faster but can fail with a `RangeError` on large buffers.
 * It includes a fallback mechanism and runtime feature detection.
 * @param {BufferType} buffer - The buffer to convert.
 * @param {number} [length] - The number of elements in the buffer to use.
 * @returns {string} The resulting string.
 */
export function bufferToString_fast(buffer: BufferType, length?: number): string {
	if (length == null) {
		length = buffer.length;
	} else {
		buffer = truncateBuffer(buffer, length);
	}

	if (config.CAN_CHARCODE_APPLY && config.CAN_CHARCODE_APPLY_TYPED) {
		const len = buffer.length;
		if (len < config.APPLY_BUFFER_SIZE && config.APPLY_BUFFER_SIZE_OK) {
			// The `any` cast is needed because apply expects `number[]` but we might have a TypedArray.
			return fromCharCode.apply(null, buffer as any);
		}

		if (config.APPLY_BUFFER_SIZE_OK === null) {
			try {
				const s = fromCharCode.apply(null, buffer as any);
				if (len > config.APPLY_BUFFER_SIZE) {
					// It works with large buffers, so we can skip chunking in the future.
					config.setApplyBufferSizeOk(true);
				}
				return s;
			} catch (e) {
				// RangeError: arguments too large
				config.setApplyBufferSizeOk(false);
			}
		}
	}

	return bufferToString_chunked(buffer);
}

/**
 * Converts a buffer to a string by processing it in chunks.
 * This is a fallback for when `bufferToString_fast` might fail.
 * @param {BufferType} buffer - The buffer to convert.
 * @returns {string} The resulting string.
 */
export function bufferToString_chunked(buffer: BufferType): string {
	let string = "";
	const length = buffer.length;
	let i = 0;
	let sub: BufferType;

	while (i < length) {
		if (!Array.isArray(buffer) && buffer.subarray) {
			sub = buffer.subarray(i, i + config.APPLY_BUFFER_SIZE);
		} else {
			// The `any` cast is for Array.slice
			sub = (buffer as any).slice(i, i + config.APPLY_BUFFER_SIZE);
		}
		i += config.APPLY_BUFFER_SIZE;

		if (config.APPLY_BUFFER_SIZE_OK) {
			string += fromCharCode.apply(null, sub as any);
			continue;
		}

		if (config.APPLY_BUFFER_SIZE_OK === null) {
			try {
				string += fromCharCode.apply(null, sub as any);
				if (sub.length > config.APPLY_BUFFER_SIZE) {
					config.setApplyBufferSizeOk(true);
				}
				continue;
			} catch (e) {
				config.setApplyBufferSizeOk(false);
			}
		}

		// If apply fails even with chunks, fall back to the slowest method.
		return bufferToString_slow(buffer);
	}

	return string;
}

/**
 * Converts a buffer to a string using a simple loop.
 * This is the slowest but most reliable method.
 * @param {BufferType} buffer - The buffer to convert.
 * @returns {string} The resulting string.
 */
export function bufferToString_slow(buffer: BufferType): string {
	let string = "";
	const length = buffer.length;

	for (let i = 0; i < length; i++) {
		string += fromCharCode(buffer[i]);
	}

	return string;
}

/**
 * Converts a string to an array of character codes.
 * @param {string | null | undefined} string - The input string.
 * @returns {number[]} An array of character codes.
 */
export function stringToArray(string: string | null | undefined): number[] {
	if (!string) {
		return [];
	}
	const array: number[] = [];
	const len = string ? string.length : 0;

	for (let i = 0; i < len; i++) {
		array[i] = string.charCodeAt(i);
	}

	return array;
}

/**
 * Creates a sliding window buffer initialized with spaces.
 * This is used to provide a "history" for the compression algorithm.
 * @returns {string} A string of spaces with a length of `WINDOW_MAX`.
 */
export function createWindow(): string {
	let i = config.WINDOW_MAX >> 7;
	let win = "        "; // 8 spaces
	while (!(i & config.WINDOW_MAX)) {
		win += win;
		i <<= 1;
	}
	return win;
}
