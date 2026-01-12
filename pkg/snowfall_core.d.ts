/* tslint:disable */
/* eslint-disable */

export class WasmCompileResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * バイナリデータ (Uint8Array | undefined)
   */
  readonly binary: Uint8Array | undefined;
  /**
   * エラーリスト (ISnowFallError[] | undefined)
   */
  readonly errors: any;
}

/**
 * ソースコードをコンパイルし、バイナリへのポインタとサイズを返す
 * debug: true の場合、ソースマップ（Debug Section）を含めます
 */
export function compile(source: string, debug: boolean): WasmCompileResult;

/**
 * 実行
 */
export function execute(binary: Uint8Array): any;

/**
 * Rust 側で確保されたヒープメモリを解放（解放専用）
 *
 * この関数はメモリ解放のみを目的としたAPIです。
 * 長さ情報を必要とせず、`capacity`分のメモリを解放します。
 *
 * この関数は`length`を0として扱うため、
 * データの内容には一切アクセスしません。
 */
export function free_memory(ptr: number, capacity: number): void;

/**
 * Rust の`Vec::into_raw_parts`によって取得したポインタを解放
 *
 * この関数は、`Vec::into_raw_parts`で分解された
 * `(ptr, length, capacity)`の完全な対となる解放関数です。
 *
 * 上記条件を満たさない場合、未定義動作(UB)になります。
 */
export function free_memory_with_len(ptr: number, length: number, capacity: number): void;

/**
 * ソースコードを受け取り、トークンのリストを返す
 * @deprecated 本番環境での使用は非推奨
 */export function lexer(source: string): any;

/**
 * ライブラリの初期化時に一度だけ呼び出されるべき関数
 */
export function main_init(): void;

/**
 * ソースコードを受け取り、正規化したASTを返す
 * @deprecated 本番環境での使用は非推奨
 */export function normalize(source: string): any;

/**
 * ソースコードを受け取り、解析したASTを返す
 * @deprecated 本番環境での使用は非推奨
 */export function parser(source: string): any;

/**
 * バージョン情報
 */
export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_init: () => void;
  readonly free_memory_with_len: (a: number, b: number, c: number) => void;
  readonly free_memory: (a: number, b: number) => void;
  readonly version: () => [number, number];
  readonly lexer: (a: number, b: number) => [number, number, number];
  readonly parser: (a: number, b: number) => [number, number, number];
  readonly normalize: (a: number, b: number) => [number, number, number];
  readonly __wbg_wasmcompileresult_free: (a: number, b: number) => void;
  readonly wasmcompileresult_binary: (a: number) => [number, number];
  readonly wasmcompileresult_errors: (a: number) => any;
  readonly compile: (a: number, b: number, c: number) => number;
  readonly execute: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
