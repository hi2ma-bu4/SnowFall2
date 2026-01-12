const CHUNK = 16384;

/**
 * Uint8Arrayをstringに変換
 */
export function strFromU8Latin1(dat: Uint8Array): string {
	let out = "";
	for (let i = 0; i < dat.length; i += CHUNK) {
		out += String.fromCharCode(...dat.subarray(i, i + CHUNK));
	}
	return out;
}

/**
 * stringをUint8Arrayに変換
 */
export function strToU8Latin1(str: string): Uint8Array {
	const len = str.length;
	const out = new Uint8Array(len);
	for (let i = 0; i < len; i++) {
		out[i] = str.charCodeAt(i);
	}
	return out;
}
