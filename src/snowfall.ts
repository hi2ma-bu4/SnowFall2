import { SnowFallErrorData } from "./errors";

// wasm-packが生成したJSと型定義をインポートします。
// `init`はデフォルトエクスポート、その他は名前空間`wasm`にインポートされます。
// @ts-ignore - a build step will create this file
import init, * as wasm from "../pkg/snowfall_core.js";

// `wasm`名前空間に`memory`が存在することをTypeScriptに伝えるための型拡張
type WasmModule = typeof wasm & { memory: WebAssembly.Memory };

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
			}
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
                if (typeof prop === 'string') {
                    return target.wasm.get_element_by_handle(target.handle.id, prop);
                }
                return Reflect.get(target, prop);
            },
            set: (target, prop, value) => {
                if (typeof prop === 'string') {
                    target.wasm.set_element_by_handle(target.handle.id, prop, value);
                    return true;
                }
                return Reflect.set(target, prop, value);
            }
        });
    }
}

/**
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export class SnowFall {
	private wasm: WasmModule | null = null;
	private isInitialized = false;
	private handleRegistry: FinalizationRegistry<number>;

	constructor() {
		this.handleRegistry = new FinalizationRegistry((handleId) => {
			this.wasm?.release_handle(handleId);
			console.log(`Released handle: ${handleId}`);
		});
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
}
