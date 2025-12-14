mod compiler;
mod runtime;
mod types;

use compiler::parser::Parser;
use runtime::Runtime;
use types::{CompilerConfig, RuntimeConfig};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_source(source: &str, config_json: &str) -> String {
    let config: CompilerConfig =
        serde_json::from_str(config_json).unwrap_or(CompilerConfig { debug_mode: false });

    let mut parser = Parser::new(source, config);
    let result = parser.compile();

    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn execute_bytecode(bytecode: &str, config_json: &str) -> String {
    let config: RuntimeConfig = serde_json::from_str(config_json).unwrap_or(RuntimeConfig {
        max_recursion: 1000,
        debug_mode: false,
    });

    let mut runtime = Runtime::new(bytecode.to_string(), config);
    match runtime.run() {
        Ok(res) => res,
        Err(e) => format!("RuntimeError: {}", e),
    }
}
