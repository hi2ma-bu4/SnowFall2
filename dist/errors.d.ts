/**
 * SnowFallエラーに関する追加情報。
 */
export interface SnowFallErrorContext {
    [key: string]: any;
}
/**
 * Rust(Wasm)から渡されるエラーオブジェクトのJSON構造に対応するインターフェース。
 */
export interface SnowFallErrorData {
    type: "CompilationError" | "RuntimeError" | "SyntaxError" | string;
    message: string;
    code: string;
    line: number;
    column: number;
    trace: string[];
    context?: SnowFallErrorContext;
}
/**
 * SnowFall言語のすべてのエラーの基底クラス。
 * Rustから渡されたエラーデータを元に、TypeScriptのネイティブErrorオブジェクトを拡張します。
 */
export declare class SnowFallError extends Error {
    readonly type: string;
    readonly code: string;
    readonly line: number;
    readonly column: number;
    readonly trace: string[];
    readonly context?: SnowFallErrorContext;
    constructor(data: SnowFallErrorData);
    /**
     * エラー情報を文字列として整形します。
     * @returns 整形されたエラー文字列
     */
    toString(): string;
}
export declare class CompilationError extends SnowFallError {
}
export declare class RuntimeError extends SnowFallError {
}
export declare class SyntaxError extends SnowFallError {
}
/**
 * エラータイプに応じて適切なエラークラスのインスタンスを生成するファクトリ関数。
 * @param data Rustから受け取ったエラーデータ
 * @returns SnowFallErrorのインスタンス
 */
export declare function createError(data: SnowFallErrorData): SnowFallError;
//# sourceMappingURL=errors.d.ts.map