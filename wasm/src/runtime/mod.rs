pub mod ops;
use crate::types::{RuntimeConfig, SFValue};
use ops::*;
use std::collections::HashMap;

pub struct Runtime {
    stack: Vec<SFValue>,
    env: HashMap<String, SFValue>, // 変数領域
    instructions: Vec<String>,
    ip: usize,
    config: RuntimeConfig,
}

impl Runtime {
    pub fn new(bytecode: String, config: RuntimeConfig) -> Self {
        Runtime {
            stack: Vec::new(),
            env: HashMap::new(),
            instructions: bytecode.lines().map(|s| s.to_string()).collect(),
            ip: 0,
            config,
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        // 命令実行ループ
        while self.ip < self.instructions.len() {
            let line = self.instructions[self.ip].clone();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                self.ip += 1;
                continue;
            }

            let opcode = parts[0];
            match opcode {
                "PUSH_INT" => {
                    let v: i64 = parts[1].parse().map_err(|_| "Invalid Int")?;
                    self.stack.push(SFValue::new_int(v));
                }
                "PUSH_FLOAT" => {
                    let v: f64 = parts[1].parse().map_err(|_| "Invalid Float")?;
                    self.stack.push(SFValue::new_float(v));
                }
                "PUSH_STR" => {
                    // 空白を含む文字列の復元は簡易的に結合
                    let val = parts[1..].join(" ");
                    self.stack.push(SFValue::new_string(val));
                }
                "PUSH_NULL" => self.stack.push(SFValue::null()),

                "STORE" => {
                    let name = parts[1].to_string();
                    let val = self.stack.pop().ok_or("Stack Underflow")?;
                    self.env.insert(name, val);
                }
                "LOAD" => {
                    let name = parts[1].to_string();
                    if let Some(val) = self.env.get(&name) {
                        self.stack.push(val.clone());
                    } else {
                        return Err(format!("Undefined variable: {}", name));
                    }
                }

                "ADD" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(op_add(a, b)?);
                }
                "SUB" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(op_sub(a, b)?);
                }
                "MUL" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(op_mul(a, b)?);
                }

                "EQ_STRICT" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(op_eq_strict(a, b));
                }
                "EQ_LOOSE" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(op_eq_loose(a, b));
                }
                "AND" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(SFValue::new_bool(a.raw_bool && b.raw_bool));
                }
                "OR" => {
                    let b = self.stack.pop().ok_or("Stack Underflow")?;
                    let a = self.stack.pop().ok_or("Stack Underflow")?;
                    self.stack.push(SFValue::new_bool(a.raw_bool || b.raw_bool));
                }

                "JUMP" => {
                    let target: usize = parts[1].parse().map_err(|_| "Invalid Jump Target")?;
                    self.ip = target;
                    continue;
                }
                "JUMP_IF_FALSE" => {
                    let target: usize = parts[1].parse().map_err(|_| "Invalid Jump Target")?;
                    let val = self.stack.pop().ok_or("Stack Underflow")?;
                    if !val.raw_bool {
                        self.ip = target;
                        continue;
                    }
                }

                _ => return Err(format!("Unknown OpCode: {}", opcode)),
            }
            self.ip += 1;
        }

        // 最終結果返却
        if let Some(last) = self.stack.last() {
            Ok(last.raw_string.clone())
        } else {
            Ok("No Result".to_string())
        }
    }
}
