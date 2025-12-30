import * as wasm from "../pkg/snowfall_core";
export type WasmModule = typeof wasm & {
    memory: WebAssembly.Memory;
};
export declare class SnowFall {
    private _isDebug;
    private _wasm;
    private _isInitialized;
    constructor(isDebug?: boolean);
    init(wasmPath: string | ArrayBuffer | NonSharedBuffer): Promise<void>;
    ensureInitialized(): WasmModule;
    /**
     * デバッグ用のLexer関数
     * @param input ソースコードの文字列
     * @returns トークンの配列
     * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください。
     */
    dev_lexer(input: string): Array<{
        type: string;
        value?: string;
    }>;
    private _logInfo;
}
//# sourceMappingURL=snowfall.d.ts.map