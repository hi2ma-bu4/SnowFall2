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
    private wasmModule;
    /**
     * Wasmモジュールを非同期で初期化します。
     * @param wasmUrl WasmファイルのURL
     */
    init(wasmUrl: string): Promise<void>;
    /**
     * Wasmモジュールをインスタンス化します。
     * TS側から呼び出される関数などをインポートオブジェクトとして渡します。
     */
    private instantiate;
    /**
     * ホスト(TS)側の関数を呼び出すためのハンドラ。
     * Rustからの `HostRequest` に応じて、適切なTS関数を実行します。
     * @param request HostRequestオブジェクト
     * @returns HostResponseオブジェクト
     */
    private handleHostRequest;
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