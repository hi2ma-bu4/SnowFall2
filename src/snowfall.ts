import { SnowFallCompiler } from "./compiler";
import { SnowFallErrorData } from "./errors";

// wasm-packが生成したJSと型定義をインポートします。
// `init`はデフォルトエクスポート、その他は名前空間`wasm`にインポートされます。
// @ts-ignore - a build step will create this file
import init, * as wasm from "../pkg/snowfall_core.js";

// `wasm`名前空間に`memory`が存在することをTypeScriptに伝えるための型拡張
export type WasmModule = typeof wasm & { memory: WebAssembly.Memory };

/**
 * Wasmのメモリ内にある大規模なオブジェクトへの参照を表すインターフェース。
 */
export interface SnowFallHandle {
	__type: "SnowFallHandle";
	id: number;
	dataType: "Array" | "Dictionary";
	size: number;
}

/**
 * Host/TS側からWasmへ渡される要求データ。
 */
export interface HostRequest {
	operation: string;
	args: any[];
	requires_return: boolean;
}

/**
 * Wasm側からHost/TSへ返される応答データ。
 */
export interface HostResponse {
	status: "OK" | "ERROR";
	result?: any;
	error_info?: SnowFallErrorData;
}

/**
 * Wasm内のSnowFall Arrayへのプロキシ。
 */
class SnowFallArrayProxy {
	private handle: SnowFallHandle;
	private wasm: WasmModule;

	constructor(handle: SnowFallHandle, wasm: WasmModule) {
		this.handle = handle;
		this.wasm = wasm;

		return new Proxy(this, {
			get: (target, prop) => {
				const index = Number(prop);
				if (!isNaN(index)) {
					return target.wasm.get_element_by_handle(target.handle.id, index);
				}
				if (prop === "length") {
					return target.handle.size;
				}
				return Reflect.get(target, prop);
			},
			set: (target, prop, value) => {
				const index = Number(prop);
				if (!isNaN(index)) {
					target.wasm.set_element_by_handle(target.handle.id, index, value);
					return true;
				}
				return Reflect.set(target, prop, value);
			},
		});
	}
}

/**
 * Wasm内のSnowFall Dictionaryへのプロキシ。
 */
class SnowFallDictionaryProxy {
	private handle: SnowFallHandle;
	private wasm: WasmModule;

	constructor(handle: SnowFallHandle, wasm: WasmModule) {
		this.handle = handle;
		this.wasm = wasm;

		return new Proxy(this, {
			get: (target, prop) => {
				if (typeof prop === "string") {
					return target.wasm.get_element_by_handle(target.handle.id, prop);
				}
				return Reflect.get(target, prop);
			},
			set: (target, prop, value) => {
				if (typeof prop === "string") {
					target.wasm.set_element_by_handle(target.handle.id, prop, value);
					return true;
				}
				return Reflect.set(target, prop, value);
			},
		});
	}
}

/**
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export { SnowFallCompiler, SnowFallCompilerError } from "./compiler";

export class SnowFall {
	private wasm: WasmModule | null = null;
	private isInitialized = false;
	private handleRegistry: FinalizationRegistry<number>;
	private hostFunctions: Map<string, Function>;

	constructor() {
		this.handleRegistry = new FinalizationRegistry((handleId) => {
			this.wasm?.release_handle(handleId);
			console.log(`Released handle: ${handleId}`);
		});
		this.hostFunctions = new Map();

		// 標準的なホスト関数を登録
		this.registerHostFunction("console.log", (...args: any[]) => console.log(...args));
	}

	public async init(wasmUrl: string | ArrayBuffer): Promise<void> {
		if (this.isInitialized) {
			return;
		}
		try {
			await init(wasmUrl);
			this.wasm = wasm as WasmModule;
			this.isInitialized = true;
			console.log("SnowFall Wasm module initialized successfully.");
		} catch (error) {
			console.error("Failed to initialize SnowFall Wasm module:", error);
			throw error;
		}
	}

	public ensureInitialized(): WasmModule {
		if (!this.wasm || !this.isInitialized) {
			throw new Error("SnowFall has not been initialized. Please call init() first.");
		}
		return this.wasm;
	}

	/**
	 * Rustから呼び出すためのTypeScript関数を登録します。
	 * @param name 登録名 (例: "my_library.my_function")
	 * @param func 実行する関数
	 */
	public registerHostFunction(name: string, func: Function): void {
		this.hostFunctions.set(name, func);
	}

	/**
	 * Wasmからの要求に応じてホスト関数を呼び出します。
	 * @param request Wasmから渡されるHostRequestオブジェクト
	 * @returns 実行結果を含むHostResponseオブジェクト
	 */
	public async invokeHostFunction(request: HostRequest): Promise<HostResponse> {
		const func = this.hostFunctions.get(request.operation);
		if (!func) {
			return {
				status: "ERROR",
				error_info: {
					type: "RuntimeError",
					message: `Host function '${request.operation}' not found.`,
					code: "SF501",
					line: 0, // Wasmからの呼び出しのため、行情報は別途渡す必要がある
					column: 0,
					trace: [],
				},
			};
		}
		try {
			// Wasmからの引数をTSネイティブ型に変換
			const nativeArgs = request.args.map((arg) => this.deserializeSnowValue(arg));
			const result = await func(...nativeArgs);

			if (request.requires_return) {
				return {
					status: "OK",
					result: this.serializeTsValue(result),
				};
			} else {
				return { status: "OK" };
			}
		} catch (e: any) {
			return {
				status: "ERROR",
				error_info: {
					type: "RuntimeError",
					message: e.message || "An unexpected error occurred in host function.",
					code: "SF502",
					line: 0,
					column: 0,
					trace: e.stack ? e.stack.split("\n") : [],
				},
			};
		}
	}

	/**
	 * Wasmから渡された `SnowObject` (のJSON表現) をTypeScriptのネイティブ型に
	 * 再帰的に変換します。`SnowFallHandle`はプロキシオブジェクトに変換されます。
	 *
	 * @param value 変換対象の `SnowObject`
	 * @returns TypeScriptのネイティブ値
	 */
	private deserializeSnowValue(value: any): any {
		if (value === null || typeof value !== "object") {
			return value;
		}

		if (value.__type === "SnowFallHandle") {
			return this._test_create_proxy_from_handle(value as SnowFallHandle);
		}

		const data = value.data;
		if (data === "Void") {
			return undefined;
		}
		if (typeof data !== "object" || data === null) {
			return data;
		}

		const key = Object.keys(data)[0];
		const val = data[key];

		switch (key) {
			case "Int":
			case "Long":
			case "Float":
			case "Char":
			case "String":
			case "Boolean":
				return val;
			case "Array":
				return val.map((item: any) => this.deserializeSnowValue(item));
			case "Dictionary": {
				const map = new Map<string, any>();
				for (const [k, v] of Object.entries(val)) {
					map.set(k, this.deserializeSnowValue(v));
				}
				return map;
			}
		}
		return value;
	}

	/**
	 * TypeScriptのネイティブ値を、Wasmが期待する `SnowObject` (のJSON表現) に
	 * 再帰的にシリアライズします。
	 * @param value シリアライズ対象のTypeScript値
	 * @returns `SnowObject` のJSON表現
	 */
	private serializeTsValue(value: any): any {
		let type_id: number;
		let data: any;

		if (value === null || value === undefined) {
			type_id = 1; // Void
			data = "Void";
		} else {
			switch (typeof value) {
				case "string":
					type_id = 4;
					data = { String: value };
					break;
				case "number":
					if (Number.isInteger(value)) {
						type_id = 2; // Int
						data = { Int: value };
					} else {
						type_id = 3; // Float
						data = { Float: value };
					}
					break;
				case "boolean":
					type_id = 5;
					data = { Boolean: value };
					break;
				case "object":
					if (Array.isArray(value)) {
						type_id = 6;
						data = { Array: value.map((v) => this.serializeTsValue(v)) };
					} else {
						type_id = 7;
						const dict: { [key: string]: any } = {};
						const entries = value instanceof Map ? value.entries() : Object.entries(value);
						for (const [k, v] of entries) {
							dict[String(k)] = this.serializeTsValue(v);
						}
						data = { Dictionary: dict };
					}
					break;
				default:
					type_id = 1; // Void
					data = "Void";
					break;
			}
		}

		return { type_id, data, properties: {}, version: 0 };
	}

	/**
	 * テスト用にハンドルからプロキシを作成する。
	 */
	public _test_create_proxy_from_handle(handle: SnowFallHandle): any {
		const wasm = this.ensureInitialized();
		let proxy: any;

		if (handle.dataType === "Array") {
			proxy = new SnowFallArrayProxy(handle, wasm);
		} else if (handle.dataType === "Dictionary") {
			proxy = new SnowFallDictionaryProxy(handle, wasm);
		} else {
			throw new Error(`Unsupported handle dataType: ${handle.dataType}`);
		}

		this.handleRegistry.register(proxy, handle.id);
		return proxy;
	}

	/**
	 * Returns a compiler instance.
	 * @returns A new SnowFallCompiler instance.
	 */
	public getCompiler(): SnowFallCompiler {
		const wasm = this.ensureInitialized();
		return new SnowFallCompiler(wasm);
	}
}
