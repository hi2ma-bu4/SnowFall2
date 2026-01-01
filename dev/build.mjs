import { build } from "esbuild";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";

/* -------------------------------------------------------------------------- */
/* 設定値 */
/* -------------------------------------------------------------------------- */

/** プロジェクトルート */
const ROOT_DIR = process.cwd();

const FILE_NAME = "snowfall";

/** Rust (wasm-pack) プロジェクトのディレクトリ */
const WASM_DIR = path.resolve(ROOT_DIR, "wasm");
const WASM_FILE = "snowfall_core_bg.wasm";

/** wasm-pack の出力先 */
const PKG_DIR = path.resolve(ROOT_DIR, "pkg");

/** esbuild の出力先 */
const DIST_DIR = path.resolve(ROOT_DIR, "dist");

/** エントリーポイント */
const ENTRY_FILE = path.resolve(ROOT_DIR, `src/${FILE_NAME}.ts`);

/** wasmファイルコピー用 */
const WASM_SRC = path.join(PKG_DIR, WASM_FILE);
const WASM_DIST = path.join(DIST_DIR, WASM_FILE);

/* -------------------------------------------------------------------------- */
/* ユーティリティ */
/* -------------------------------------------------------------------------- */

/**
 * ディレクトリを安全に削除して再作成する
 * @param {string} dirPath
 */
function cleanDir(dirPath) {
	if (fs.existsSync(dirPath)) {
		fs.rmSync(dirPath, { recursive: true, force: true });
	}
	fs.mkdirSync(dirPath, { recursive: true });
}

/**
 * コマンドを同期実行する（失敗時は即終了）
 * @param {string} command
 * @param {string[]} args
 * @param {string} cwd
 * @param {string} [errMes] - エラーメッセージ
 */
function runCommand(command, args, cwd, errMes) {
	const result = spawnSync(command, args, {
		cwd,
		stdio: "inherit",
		shell: process.platform === "win32", // Windows 対策
	});

	if (result.status !== 0) {
		if (errMes) console.error(errMes);
		process.exit(result.status ?? 1);
	}
}

/* -------------------------------------------------------------------------- */
/* wasm-pack ビルド */
/* -------------------------------------------------------------------------- */

/**
 * wasm-pack build を実行する
 *
 * --target web:
 *   - ブラウザ ESM 前提
 *   - import.meta.url を使用した wasm ローダー生成
 */
function buildWasm() {
	console.log("🦀 wasm-pack build 開始...");

	runCommand("npx", ["wasm-pack", "--version"], WASM_DIR, "❌ wasm-pack が利用できません");

	// runCommand("cargo", ["test"], WASM_DIR, "❌ wasmのテストに失敗しました");

	runCommand("npx", ["wasm-pack", "build", "--target", "web", "--no-pack", "--release", "-d", PKG_DIR], WASM_DIR, "❌ wasm-pack build に失敗しました");

	fs.rmSync("pkg/.gitignore", { force: true });

	fs.copyFileSync(WASM_SRC, WASM_DIST);

	console.log("┗✅ wasm-pack build 完了");
}

/**
 * wasm の再ビルドが必要か判定する
 * @returns {boolean}
 */
function shouldBuildWasm() {
	if (!fs.existsSync(PKG_DIR)) return true;

	// 必要なら mtime 比較などに拡張可能
	const files = fs.readdirSync(PKG_DIR);
	return files.length === 0;
}

/* -------------------------------------------------------------------------- */
/* ts バージョン注入 */
/* -------------------------------------------------------------------------- */
function getVersion() {
	const pkg = JSON.parse(fs.readFileSync(path.relative(ROOT_DIR, "package.json"), "utf8"));
	const out = `/** 自動生成・編集禁止 */
export const VERSION = ${JSON.stringify(pkg.version)} as const;
`;
	fs.writeFileSync("src/version.ts", out);
}

/* -------------------------------------------------------------------------- */
/* esbuild */
/* -------------------------------------------------------------------------- */

const ESBUILD_COMMON = {
	entryPoints: [ENTRY_FILE],
	outdir: DIST_DIR,
	bundle: true,

	/* ESM / browser 前提 */
	format: "esm",
	platform: "browser",
	target: "es2024",

	sourcemap: true,
	minify: false,

	loader: {
		".wasm": "file",
	},

	supported: {
		"import-meta": true,
	},
};

/**
 * esbuild を実行する
 *
 * - ESM 出力
 * - import.meta を保持
 * - wasm は file loader
 */
async function buildJs() {
	console.log("📦 esbuild 開始...");

	await build({
		...ESBUILD_COMMON,
		entryNames: FILE_NAME,
	});

	console.log("┗✅ esbuild 完了");
}

async function buildJsMin() {
	console.log("📦 esbuild (min) 開始...");

	await build({
		...ESBUILD_COMMON,
		entryNames: `${FILE_NAME}.min`,
		minify: true,
	});

	console.log("┗✅ esbuild (min) 完了");
}

/* -------------------------------------------------------------------------- */
/* .d.ts */
/* -------------------------------------------------------------------------- */

/**
 * .d.ts を dist に生成する
 */
function buildTypes() {
	console.log("📐 型定義(.d.ts)生成開始...");

	runCommand("npx", ["dts-bundle-generator", "-o", `${DIST_DIR}/${FILE_NAME}.d.ts`, ENTRY_FILE], ROOT_DIR, "❌ 型定義のバンドルに失敗しました");

	console.log("┗✅ 型定義生成完了");
}

/* -------------------------------------------------------------------------- */
/* .d.ts deprecated コメント自動追加 */
/* -------------------------------------------------------------------------- */

/**
 * Rust の #[deprecated(note = "...")] を解析して
 * 対応する .d.ts に \/\** \@deprecated ... *\/ を追加する
 */
function addDeprecatedToDts() {
	console.log("📝 .d.ts に deprecated コメント追加開始...");

	const DTS_FILE = path.join(PKG_DIR, "snowfall_core.d.ts");
	const LIB_RS = path.join(WASM_DIR, "src/lib.rs");

	if (!fs.existsSync(DTS_FILE)) {
		console.warn("⚠ .d.ts ファイルが見つかりません:", DTS_FILE);
		return;
	}
	if (!fs.existsSync(LIB_RS)) {
		console.warn("⚠ lib.rs が見つかりません:", LIB_RS);
		return;
	}

	const rustCode = fs.readFileSync(LIB_RS, "utf-8");
	let dts = fs.readFileSync(DTS_FILE, "utf-8");

	// #[deprecated(note = "...")] の関数だけ抽出
	const deprecatedMap = {};
	const depRegex = /#\[deprecated\((?:[^)]*note\s*=\s*"([^"]+)")[^)]*\)\][\s\S]*?pub fn (\w+)/g;
	let match;
	while ((match = depRegex.exec(rustCode)) !== null) {
		const [, note, fnName] = match;
		deprecatedMap[fnName] = note;
	}

	const detectedCount = Object.keys(deprecatedMap).length;
	let replacedCount = 0;

	// d.ts に @deprecated コメントを追加
	for (const [fnName, note] of Object.entries(deprecatedMap)) {
		console.log(`┃ [deprecated] lib::${fnName}`);
		// JSDoc がある場合は末尾に追記
		const jsdocRegex = new RegExp(`(\/\\*\\*(?:[^*]|\\*(?!\\/))*\\*\\/)\\s*(export function ${fnName}\\s*\\()`, "g");
		if (jsdocRegex.test(dts)) {
			dts = dts.replace(jsdocRegex, (all, a, b) => {
				// コメント内の末尾に追記
				return (
					a.replace(/\*\/$/, () => {
						replacedCount++;
						return `* @deprecated ${note}\n */`;
					}) + b
				);
			});
		} else {
			const fnRegex = new RegExp(`(export function ${fnName}\\()`, "g");
			dts = dts.replace(fnRegex, (all) => {
				// コメントがない場合は新規追加
				replacedCount++;
				return `/** @deprecated ${note} */\n${all}`;
			});
		}
	}

	fs.writeFileSync(DTS_FILE, dts, "utf-8");
	console.log(`┗✅ .d.ts deprecated コメント追加完了: ${replacedCount}/${detectedCount}`);
}

/* -------------------------------------------------------------------------- */
/* メイン処理 */
/* -------------------------------------------------------------------------- */

(async () => {
	try {
		console.log("🧹 dist / pkg クリーン中...");
		cleanDir(DIST_DIR);
		cleanDir(PKG_DIR);

		if (shouldBuildWasm()) {
			buildWasm();
			addDeprecatedToDts();
		}

		getVersion();

		await Promise.all([
			//
			buildJs(),
			buildJsMin(),
		]);

		buildTypes();

		console.log("🎉 build 完了");
	} catch (err) {
		console.error("❌ build 失敗:", err);
		process.exit(1);
	}
})();
