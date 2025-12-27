use crate::common::object::SnowValue;
use crate::common::sir::{ConstantEntry, DebugMapEntry, Header, Instruction, Sir};
use crate::compiler::ast::{AstNode, Expression, Statement, Visitor};
use std::collections::HashMap;

/// # SnowFall Intermediate Representation (SIR) Example
///
/// This example demonstrates how a simple `If` statement in SnowFall
/// is compiled into a sequence of SIR instructions.
///
/// ## SnowFall Source Code
///
/// ```snowfall
/// if (a > b) {
///     return 1;
/// } else {
///     return 0;
/// }
/// ```
///
/// ## Generated SIR Instructions
///
/// ```text
/// PUSH_CONST 0  // Push constant 'a' from the constant table
/// PUSH_CONST 1  // Push constant 'b'
/// GREATER_THAN  // Pop 'a' and 'b', push boolean result of a > b
/// JUMP_IF_FALSE 8 // Pop boolean, if false, jump to instruction at index 8
/// PUSH_CONST 2  // Push constant '1'
/// RETURN        // Return from function
/// JUMP 9        // Unconditionally jump to the end
/// PUSH_CONST 3  // Push constant '0' (at index 8)
/// RETURN        // Return from function (at index 9)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Stack operations
    PushConst(u32),
    Pop,

    // Arithmetic & Logic
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    Equals,

    // Control Flow
    Jump(usize),
    JumpIfFalse(usize),

    // Variables & Scope
    SetVar(u32),
    GetVar(u32),

    // Functions
    Call(u8),
    Return,
}

pub struct CodeGenerator {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<SnowValue>,
    // Other state: debug info, etc.
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn generate(&mut self, node: &AstNode) -> Sir {
        self.visit_node(node);

        // Finalize SIR structure
        Sir {
            header: Header {
                sir_version: "1.0".to_string(),
                debug_enabled: false,
                source_hash: None,
            },
            constants: self
                .constants
                .iter()
                .enumerate()
                .map(|(i, v)| ConstantEntry {
                    index: i as u32,
                    value: v.clone(),
                })
                .collect(),
            code: self.instructions.clone(),
            debug_map: None,
        }
    }

    fn add_constant(&mut self, value: SnowValue) -> u32 {
        self.constants.push(value);
        (self.constants.len() - 1) as u32
    }

    fn emit(&mut self, opcode: OpCode) -> usize {
        let (op_str, operands) = match opcode {
            OpCode::PushConst(idx) => ("PUSH_CONST".to_string(), vec![idx]),
            OpCode::Pop => ("POP".to_string(), vec![]),
            OpCode::Add => ("ADD".to_string(), vec![]),
            OpCode::SetVar(idx) => ("SET_VAR".to_string(), vec![idx]),
            OpCode::Return => ("RETURN".to_string(), vec![]),
            OpCode::Jump(addr) => ("JUMP".to_string(), vec![addr as u32]),
            OpCode::JumpIfFalse(addr) => ("JUMP_IF_FALSE".to_string(), vec![addr as u32]),
            _ => ("UNKNOWN".to_string(), vec![]),
        };

        let instruction = Instruction {
            index: self.instructions.len() as u32,
            opcode: op_str,
            operands,
        };
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }
}

impl Visitor for CodeGenerator {
    type Output = ();

    fn visit_node(&mut self, node: &AstNode) {
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.visit_statement(stmt);
                }
            }
            AstNode::Statement(stmt) => self.visit_statement(stmt),
            AstNode::Expression(expr) => self.visit_expression(expr),
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { value, .. } => {
                self.visit_expression(value);
                // For now, assume we're setting a local variable at index 0
                self.emit(OpCode::SetVar(0));
            }
            Statement::Expression(expr) => {
                self.visit_expression(expr);
                self.emit(OpCode::Pop);
            }
            Statement::If {
                condition,
                consequence,
                ..
            } => {
                self.visit_expression(condition);
                let jump_if_false_pos = self.emit(OpCode::JumpIfFalse(999)); // Placeholder
                self.visit_statement(consequence);

                let after_consequence_pos = self.instructions.len();
                self.instructions[jump_if_false_pos].operands[0] = after_consequence_pos as u32;
            }
            Statement::Block(statements) => {
                for s in statements {
                    self.visit_statement(s);
                }
            }
            Statement::Function { body, .. } => {
                self.visit_statement(body);
                // Implicitly return if the last instruction isn't already a return.
                if self.instructions.last().map(|i| i.opcode.as_str()) != Some("RETURN") {
                    self.emit(OpCode::PushConst(u32::MAX)); // Placeholder for void/null
                    self.emit(OpCode::Return);
                }
            }
            Statement::Sub { body, .. } => {
                self.visit_statement(body);
                if self.instructions.last().map(|i| i.opcode.as_str()) != Some("RETURN") {
                    self.emit(OpCode::PushConst(u32::MAX)); // Placeholder for void/null
                    self.emit(OpCode::Return);
                }
            }
            Statement::Return(expr) => {
                self.visit_expression(expr);
                self.emit(OpCode::Return);
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::IntLiteral(val) => {
                let const_idx = self.add_constant(SnowValue::Int(*val as i32));
                self.emit(OpCode::PushConst(const_idx));
            }
            Expression::FloatLiteral(val) => {
                let const_idx = self.add_constant(SnowValue::Float(*val));
                self.emit(OpCode::PushConst(const_idx));
            }
            Expression::StringLiteral(val) => {
                let const_idx = self.add_constant(SnowValue::String(val.clone()));
                self.emit(OpCode::PushConst(const_idx));
            }
            Expression::Boolean(val) => {
                let const_idx = self.add_constant(SnowValue::Boolean(*val));
                self.emit(OpCode::PushConst(const_idx));
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                self.visit_expression(left);
                self.visit_expression(right);
                match operator {
                    crate::compiler::Token::Plus => {
                        self.emit(OpCode::Add);
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}
