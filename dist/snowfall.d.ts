import { SnowFallErrorData } from './errors';
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
    status: 'OK' | 'ERROR';
    result?: any;
    error_info?: SnowFallErrorData;
}
/**
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export declare class SnowFall {
    private wasm;
    private isInitialized;
    /**
     * Wasmモジュールを非同期で初期化します。
     */
    init(wasmUrl: string): Promise<void>;
    /**
     * 初期化が完了していることを保証するヘルパーメソッド。
     */
    private ensureInitialized;
    /**
     * Wasmメモリからポインタを介してオブジェクトをデコード(デシリアライズ)します。
     */
    private decodeObjectFromWasm;
    /**
     * オブジェクトをエンコード(シリアライズ)してWasmメモリに書き込み、ポインタを返します。
     */
    private encodeObjectToWasm;
}
//# sourceMappingURL=snowfall.d.ts.map