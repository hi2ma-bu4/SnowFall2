pub mod error;
/**
 * @fileoverview
 * このモジュールは、SnowFall言語のコンパイラとランタイム全体で共有される
 * コアなデータ構造を定義します。これには、SnowFallのオブジェクトモデル、
 * エラー表現、および中間コード（SIR）の形式などが含まれます。
 *
 * This module defines the core data structures shared across the SnowFall
 * language's compiler and runtime. This includes the SnowFall object model,
 * error representations, and the intermediate representation (SIR) format.
 */
// 新しいモジュールを公開します (Expose the new modules).
pub mod object;
pub mod operator;
pub mod sir;
