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

/** Rust (wasm-pack) プロジェクトのディレクトリ */
const WASM_DIR = path.resolve(ROOT_DIR, "wasm");
const WASM_FILE = "snowfall_core_bg.wasm";

/** wasm-pack の出力先 */
const PKG_DIR = path.resolve(ROOT_DIR, "pkg");

/** esbuild の出力先 */
const DIST_DIR = path.resolve(ROOT_DIR, "dist");

/** エントリーポイント */
const ENTRY_FILE = path.resolve(ROOT_DIR, "src/snowfall.ts");

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

	runCommand("npx", ["wasm-pack", "build", "--target", "web", "--no-pack", "-d", PKG_DIR], WASM_DIR);

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
		entryNames: "snowfall",
	});

	console.log("┗✅ esbuild 完了");
}

async function buildJsMin() {
	console.log("📦 esbuild (min) 開始...");

	await build({
		...ESBUILD_COMMON,
		entryNames: "snowfall.min",
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

	runCommand("npx", ["tsc", "--emitDeclarationOnly", "--declaration", "--declarationMap", "--outDir", DIST_DIR, "--project", "tsconfig.json"], ROOT_DIR, "❌ 型定義の生成に失敗しました");

	console.log("┗✅ 型定義生成完了");
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
		}

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
