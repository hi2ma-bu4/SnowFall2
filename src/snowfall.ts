import init, * as wasm from "../pkg/snowfall_core";
import type { Token } from "./const/types";
import { Logger } from "./libs/Logger";
import { compareVersion, parseSemVer } from "./libs/version_check";
import { VERSION } from "./version";

// `wasm`名前空間に`memory`が存在することをTypeScriptに伝えるための型拡張
export type WasmModule = typeof wasm & { memory: WebAssembly.Memory };

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
	/* デバッグ用機能 */
	/* ================================================== */

	/**
	 * デバッグ用のLexer関数
	 * @param input ソースコードの文字列
	 * @returns トークンの配列
	 * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください。
	 */
	public dev_lexer(input: string): Array<Token> {
		const wasm = this.ensureInitialized();
		return wasm.lexer(input);
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
