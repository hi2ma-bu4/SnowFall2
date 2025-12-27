import { SnowFallErrorData } from "./errors";
import * as wasm from "../pkg/snowfall_core.js";
type WasmModule = typeof wasm & {
    memory: WebAssembly.Memory;
};
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
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export declare class SnowFall {
    private wasm;
    private isInitialized;
    private handleRegistry;
    private hostFunctions;
    constructor();
    init(wasmUrl: string | ArrayBuffer): Promise<void>;
    ensureInitialized(): WasmModule;
    /**
     * Rustから呼び出すためのTypeScript関数を登録します。
     * @param name 登録名 (例: "my_library.my_function")
     * @param func 実行する関数
     */
    registerHostFunction(name: string, func: Function): void;
    /**
     * Wasmからの要求に応じてホスト関数を呼び出します。
     * @param request Wasmから渡されるHostRequestオブジェクト
     * @returns 実行結果を含むHostResponseオブジェクト
     */
    invokeHostFunction(request: HostRequest): Promise<HostResponse>;
    /**
     * Wasmから渡された `SnowObject` (のJSON表現) をTypeScriptのネイティブ型に
     * 再帰的に変換します。`SnowFallHandle`はプロキシオブジェクトに変換されます。
     *
     * @param value 変換対象の `SnowObject`
     * @returns TypeScriptのネイティブ値
     */
    private deserializeSnowValue;
    /**
     * TypeScriptのネイティブ値を、Wasmが期待する `SnowObject` (のJSON表現) に
     * 再帰的にシリアライズします。
     * @param value シリアライズ対象のTypeScript値
     * @returns `SnowObject` のJSON表現
     */
    private serializeTsValue;
    /**
     * テスト用にハンドルからプロキシを作成する。
     */
    _test_create_proxy_from_handle(handle: SnowFallHandle): any;
}
export {};
//# sourceMappingURL=snowfall.d.ts.map