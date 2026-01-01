/**
 * @file Configuration constants for the compression/decompression library.
 */

/**
 * Indicates if TypedArrays (Uint8Array, Uint16Array) are available.
 * @type {boolean}
 */
export const HAS_TYPED: boolean = typeof Uint8Array !== "undefined" && typeof Uint16Array !== "undefined";

/**
 * Checks if `String.fromCharCode.apply` can be used with a regular array.
 * @type {boolean}
 */
let canCharCodeApply = false;
try {
	if (String.fromCharCode.apply(null, [0x61]) === "a") {
		canCharCodeApply = true;
	}
} catch (e) {}
export const CAN_CHARCODE_APPLY = canCharCodeApply;

/**
 * Checks if `String.fromCharCode.apply` can be used with a TypedArray.
 * @type {boolean}
 */
let canCharCodeApplyTyped = false;
if (HAS_TYPED) {
	try {
		if (String.fromCharCode.apply(null, new Uint8Array([0x61]) as any) === "a") {
			canCharCodeApplyTyped = true;
		}
	} catch (e) {}
}
export const CAN_CHARCODE_APPLY_TYPED = canCharCodeApplyTyped;

/**
 * The maximum number of arguments for `Function.prototype.apply`.
 * @type {number}
 */
export const APPLY_BUFFER_SIZE = 65533;

/**
 * A flag to check if the `APPLY_BUFFER_SIZE` is safe to use without causing a RangeError.
 * `null` indicates it hasn't been checked yet.
 * @type {boolean | null}
 */
export let APPLY_BUFFER_SIZE_OK: boolean | null = null;

/**
 * Sets the value of APPLY_BUFFER_SIZE_OK.
 * This is used because imported module variables are read-only.
 * @param {boolean} value - The new value.
 */
export function setApplyBufferSizeOk(value: boolean): void {
	APPLY_BUFFER_SIZE_OK = value;
}

/**
 * A flag for a bug in some IE versions with `String.prototype.lastIndexOf`.
 * @type {boolean}
 */
let stringLastIndexOfBug = false;
if ("abc\u307b\u3052".lastIndexOf("\u307b\u3052", 1) !== -1) {
	stringLastIndexOfBug = true;
}
export const STRING_LASTINDEXOF_BUG = stringLastIndexOfBug;

/**
 * The base62 character set used for encoding. (A-Z, a-z, 0-9)
 * @type {string}
 */
export const BASE62TABLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

// Buffers
/**
 * The length of the base62 table.
 * @type {number}
 */
export const TABLE_LENGTH = BASE62TABLE.length;

/**
 * The difference between `TABLE_LENGTH` and 62.
 * @type {number}
 */
export const TABLE_DIFF = Math.max(TABLE_LENGTH, 62) - Math.min(TABLE_LENGTH, 62);

/**
 * Maximum buffer value, derived from `TABLE_LENGTH`.
 * @type {number}
 */
export const BUFFER_MAX = TABLE_LENGTH - 1;

// Sliding Window
/**
 * Maximum size of the sliding window.
 * @type {number}
 */
export const WINDOW_MAX = 1024;

/**
 * Maximum buffer size for the sliding window.
 * @type {number}
 */
export const WINDOW_BUFFER_MAX = 304; // maximum 304

// Chunk buffer length
/**
 * The size of chunks for compression, tied to `APPLY_BUFFER_SIZE`.
 * @type {number}
 */
export const COMPRESS_CHUNK_SIZE = APPLY_BUFFER_SIZE;

/**
 * The maximum size for a compression chunk.
 * @type {number}
 */
export const COMPRESS_CHUNK_MAX = COMPRESS_CHUNK_SIZE - TABLE_LENGTH;

/**
 * The size of chunks for decompression.
 * @type {number}
 */
export const DECOMPRESS_CHUNK_SIZE = APPLY_BUFFER_SIZE;

/**
 * The maximum size for a decompression chunk.
 * @type {number}
 */
export const DECOMPRESS_CHUNK_MAX = DECOMPRESS_CHUNK_SIZE + WINDOW_MAX * 2;

// Unicode table : U+0000 - U+0084
const LATIN_CHAR_MAX = 11;
/**
 * Maximum buffer size for latin characters.
 * @type {number}
 */
export const LATIN_BUFFER_MAX = LATIN_CHAR_MAX * (LATIN_CHAR_MAX + 1);

// Unicode table : U+0000 - U+FFFF
/**
 * Maximum number of characters in the Unicode character set for this algorithm.
 * @type {number}
 */
export const UNICODE_CHAR_MAX = 40;

/**
 * Maximum buffer size for Unicode characters.
 * @type {number}
 */
export const UNICODE_BUFFER_MAX = UNICODE_CHAR_MAX * (UNICODE_CHAR_MAX + 1);

// Index positions
/**
 * Index for Latin characters.
 * @type {number}
 */
export const LATIN_INDEX = TABLE_LENGTH + 1;

/**
 * Starting index for Latin characters.
 * @type {number}
 */
export const LATIN_INDEX_START = TABLE_DIFF + 20;

/**
 * Index for Unicode characters.
 * @type {number}
 */
export const UNICODE_INDEX = TABLE_LENGTH + 5;

// Decode/Start positions
/**
 * Maximum value for decoding.
 * @type {number}
 */
export const DECODE_MAX = TABLE_LENGTH - TABLE_DIFF - 19;

/**
 * Maximum decode value for Latin characters.
 * @type {number}
 */
export const LATIN_DECODE_MAX = UNICODE_CHAR_MAX + 7;

/**
 * Start position for character encoding.
 * @type {number}
 */
export const CHAR_START = LATIN_DECODE_MAX + 1;

/**
 * Start position for compression.
 * @type {number}
 */
export const COMPRESS_START = CHAR_START + 1;

/**
 * Start position for fixed-length compression.
 * @type {number}
 */
export const COMPRESS_FIXED_START = COMPRESS_START + 5;

/**
 * Index for compression markers.
 * @type {number}
 */
export const COMPRESS_INDEX = COMPRESS_FIXED_START + 5; // 59
// Currently, 60 and 61 of the position is not used yet
