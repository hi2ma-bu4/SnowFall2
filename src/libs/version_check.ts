/**
 * セマンティックなバージョン表現
 * @property major - 互換性破壊を伴う変更
 * @property minor - 後方互換な機能追加
 * @property patch - 後方互換なバグ修正
 */
type SemVer = {
	major: number;
	minor: number;
	patch: number;
};

/**
 * SemVer 文字列をパースする
 *
 * 対応形式:
 * - "1.2.3"
 * - "v1.2.3"
 *
 * 非対応:
 * - prerelease / build metadata
 * - 欠損・不正フォーマット
 *
 * @param v - バージョン文字列
 * @returns パース結果。失敗時は null
 */
export function parseSemVer(v: string): SemVer | null {
	const m = /^v?(\d+)\.(\d+)\.(\d+)$/.exec(v);
	if (!m) return null;

	return {
		major: Number(m[1]),
		minor: Number(m[2]),
		patch: Number(m[3]),
	};
}

/**
 * バージョン比較結果
 *
 * - ok   : 完全一致
 * - warn : PATCH 差異（継続可能）
 * - err  : MINOR / MAJOR 差異（即停止）
 */
type VersionCheckResult = { kind: "ok" } | { kind: "warn"; message: string } | { kind: "err"; message: string };

/**
 * TypeScript と Rust(wasm) の SemVer を比較する
 *
 * 判定ルール:
 * - MAJOR 不一致 → err（停止）
 * - MINOR 不一致 → err（停止）
 * - PATCH 不一致 → warn（継続）
 *
 * @param tsV - TypeScript 側バージョン
 * @param rustV - Rust(wasm) 側バージョン
 * @returns 判定結果
 */
export function compareVersion(tsV: SemVer, rustV: SemVer): VersionCheckResult {
	if (tsV.major !== rustV.major) {
		return {
			kind: "err",
			message: `MAJOR mismatch: ts=${tsV.major}, rust=${rustV.major}`,
		};
	}

	if (tsV.minor !== rustV.minor) {
		return {
			kind: "err",
			message: `MINOR mismatch: ts=${tsV.minor}, rust=${rustV.minor}`,
		};
	}

	if (tsV.patch !== rustV.patch) {
		return {
			kind: "warn",
			message: `PATCH mismatch: ts=${tsV.patch}, rust=${rustV.patch}`,
		};
	}

	return { kind: "ok" };
}
