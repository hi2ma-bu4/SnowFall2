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
    constructor();
    init(wasmUrl: string | ArrayBuffer): Promise<void>;
    ensureInitialized(): WasmModule;
    /**
     * テスト用にハンドルからプロキシを作成する。
     */
    _test_create_proxy_from_handle(handle: SnowFallHandle): any;
}
export {};
//# sourceMappingURL=snowfall.d.ts.map