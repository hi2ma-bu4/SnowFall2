import init, * as wasm from "../pkg/snowfall_core";
import { SnowFallError } from "./common/SnowFallError";
import type { CompileBinResult, CompileStrResult, ISnowFallError, ParserResult, Token } from "./common/types";
import { strFromU8Latin1 } from "./libs/compress";
import { Logger } from "./libs/Logger";
import * as lzbase62 from "./libs/lzbase62/src";
import { compareVersion, parseSemVer } from "./libs/version_check";
import { VERSION } from "./version";

// `wasm`名前空間に`memory`が存在することをTypeScriptに伝えるための型拡張
export type WasmModule = typeof wasm;

export class SnowFall {
	private _wasm: WasmModule | null = null;
	private _isInitialized: boolean = false;

	constructor(isDebug: boolean = false) {
		Logger.isDebug = isDebug;
	}

	public async init(wasmPath: string | ArrayBuffer | NonSharedBuffer): Promise<void> {
		if (this._isInitialized) return;

		try {
			await init(wasmPath);
		} catch (error) {
			Logger.error("Failed to initialize SnowFall Wasm module:", error);
			throw error;
		}

		this._wasm = wasm as WasmModule;
		this._versionCheck();

		this._isInitialized = true;

		Logger.info("SnowFall Wasm module initialized successfully.");
	}

	public ensureInitialized(): WasmModule {
		if (!this._wasm || !this._isInitialized) {
			throw new Error("SnowFall has not been initialized. Please call init() first.");
		}
		return this._wasm;
	}

	/* ================================================== */
	/* 管理情報 */
	/* ================================================== */

	/**
	 * ts(js)ライブラリのバージョン取得
	 */
	public version(): string {
		return VERSION;
	}
	/**
	 * rust(wasm)ライブラリのバージョン取得
	 */
	public version_wasm(): string {
		const wasm = this.ensureInitialized();
		return wasm.version();
	}

	/* ================================================== */
	/* 公開機能 */
	/* ================================================== */

	/**
	 * sfソースコードをコンパイルする
	 * @param input ソースコードの文字列
	 * @param debug ソースマップを追加するか
	 * @returns バイナリデータなど
	 */
	public compile_bin(input: string, debug: boolean): CompileBinResult {
		const wasm = this.ensureInitialized();
		const result = wasm.compile(input, debug);
		try {
			// エラーのチェック
			const errorsVal = result.errors;
			if (errorsVal) {
				const errors = errorsVal as ISnowFallError[];
				// 配列で、かつ中身がある場合
				if (Array.isArray(errors) && errors.length > 0) {
					return {
						errors: errors.map((err) => new SnowFallError(err)),
					};
				}
			}

			// バイナリの取得
			const binary = result.binary;
			if (binary) {
				return { binary };
			}

			// ここに来ることは通常ない
			return {};
		} finally {
			// Wasm側のオブジェクトを解放する
			result.free();
		}
	}

	/**
	 * sfソースコードをコンパイルする
	 * @param input ソースコードの文字列
	 * @param debug ソースマップを追加するか
	 * @returns テキストデータなど
	 */
	public compile(input: string, debug: boolean): CompileStrResult {
		const result = this.compile_bin(input, debug);
		if (result.errors) {
			return {
				errors: result.errors,
			};
		}
		if (result.binary) {
			return {
				data: lzbase62.compress(strFromU8Latin1(result.binary)),
			};
		}
		return {};
	}

	/* ================================================== */
	/* デバッグ用機能 */
	/* ================================================== */

	/**
	 * デバッグ用のLexer関数
	 * @param input ソースコードの文字列
	 * @returns トークンの配列
	 * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
	 */
	public dev_lexer(input: string): Array<Token> {
		const wasm = this.ensureInitialized();
		return wasm.lexer(input);
	}

	/**
	 * デバッグ用のParser(Lexer含む)関数
	 * @param input ソースコードの文字列
	 * @returns トークンの配列
	 * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
	 */
	public dev_parser(input: string): ParserResult {
		const wasm = this.ensureInitialized();
		const result = wasm.parser(input) as ParserResult;
		if (result.errors) {
			return {
				...result,
				errors: result.errors.map((err: ISnowFallError) => new SnowFallError(err)),
			};
		}
		return result;
	}

	/**
	 * デバッグ用のParser(normalize済)関数
	 * @param input ソースコードの文字列
	 * @returns トークンの配列
	 * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
	 */
	public dev_normalize(input: string): ParserResult {
		const wasm = this.ensureInitialized();
		const result = wasm.normalize(input) as ParserResult;
		if (result.errors) {
			return {
				...result,
				errors: result.errors.map((err: ISnowFallError) => new SnowFallError(err)),
			};
		}
		return result;
	}

	/* ================================================== */
	/* 共通利用 */
	/* ================================================== */

	/**
	 * バージョンチェック
	 * @throws {Error}
	 */
	private _versionCheck(): void {
		if (!this._wasm) return;
		const tsVer = parseSemVer(this.version());
		const rustVer = parseSemVer(this._wasm.version());

		if (!tsVer || !rustVer) {
			throw new Error("Invalid version format (expected x.y.z)");
		}

		const result = compareVersion(tsVer, rustVer);

		switch (result.kind) {
			case "ok":
				return;
			case "warn":
				Logger.warn("[Version]", result.message);
				return;
			case "err":
				throw new Error(`[Version] ${result.message}`);
		}
	}
}
