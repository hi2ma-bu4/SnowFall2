// src/errors.ts

/**
 * SnowFallエラーに関する追加情報。
 */
export interface SnowFallErrorContext {
  // 例: 期待された型、見つかった型など
  [key: string]: any;
}

/**
 * Rust(Wasm)から渡されるエラーオブジェクトのJSON構造に対応するインターフェース。
 */
export interface SnowFallErrorData {
  type: 'CompilationError' | 'RuntimeError' | 'SyntaxError' | string;
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
export class SnowFallError extends Error {
  public readonly type: string;
  public readonly code: string;
  public readonly line: number;
  public readonly column: number;
  public readonly trace: string[];
  public readonly context?: SnowFallErrorContext;

  constructor(data: SnowFallErrorData) {
    super(data.message);
    this.name = data.type || 'SnowFallError';

    this.type = data.type;
    this.code = data.code;
    this.line = data.line;
    this.column = data.column;
    this.trace = data.trace || [];
    this.context = data.context;

    // スタックトレースをよりリッチな形式に整形
    this.stack = `${this.name} (${this.code}): ${this.message}\n` +
                 `    at <source>:${this.line}:${this.column}\n` +
                 this.trace.map(t => `    at ${t}`).join('\n');
  }

  /**
   * エラー情報を文字列として整形します。
   * @returns 整形されたエラー文字列
   */
  public toString(): string {
    return this.stack || `${this.name}: ${this.message}`;
  }
}

// 具体的なエラークラスの例
export class CompilationError extends SnowFallError {}
export class RuntimeError extends SnowFallError {}
export class SyntaxError extends SnowFallError {}

/**
 * エラータイプに応じて適切なエラークラスのインスタンスを生成するファクトリ関数。
 * @param data Rustから受け取ったエラーデータ
 * @returns SnowFallErrorのインスタンス
 */
export function createError(data: SnowFallErrorData): SnowFallError {
  switch (data.type) {
    case 'CompilationError':
      return new CompilationError(data);
    case 'RuntimeError':
      return new RuntimeError(data);
    case 'SyntaxError':
      return new SyntaxError(data);
    default:
      return new SnowFallError(data);
  }
}
