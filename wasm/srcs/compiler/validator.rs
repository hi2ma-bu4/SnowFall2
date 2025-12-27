use crate::compiler::ast::{AstNode, Expression, Statement, Visitor};
use crate::common::error::SnowFallError;
use crate::common::object::TypeId;
use std::collections::HashMap;

/// A simplified representation of types for the verifier.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Void,
    Any, // For unknown types or dynamic behavior
    Function { param_types: Vec<Type>, return_type: Box<Type> },
    Class { name: String, methods: HashMap<String, Type> },
}

/// Represents the type definition of an object at compile time.
pub struct TypeDefinition {
    pub properties: HashMap<String, TypeId>,
    pub methods: HashMap<String, TypeId>, // Using TypeId for function signatures
    pub __proto__: Option<TypeId>,
}

/// Represents the entire type system known to the compiler.
pub type TypeSystem = HashMap<TypeId, TypeDefinition>;

/// The main struct for static type analysis.
pub struct TypeChecker {
    pub type_system: TypeSystem,
    pub errors: Vec<SnowFallError>,
    // A simple symbol table for variable types in the current scope.
    symbol_table: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut type_system = TypeSystem::new();
        // TODO: Populate with built-in types like Int, String, etc.
        TypeChecker {
            type_system,
            errors: Vec::new(),
            symbol_table: HashMap::new(),
        }
    }

    pub fn check(&mut self, node: &AstNode) -> Result<(), Vec<SnowFallError>> {
        self.visit_node(node);
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Type-checks an infix expression, paying special attention to `+` with strings.
    fn check_infix_expression(&mut self, left: &Expression, op: &str, right: &Expression) -> Type {
        let left_type = self.visit_expression(left);
        let right_type = self.visit_expression(right);

        if op == "+" {
            if left_type == Type::String || right_type == Type::String {
                return Type::String; // String concatenation rule
            }
        }

        // For other operators, if types are the same, return that type.
        // This is a simplification. A real implementation would have more complex rules.
        if left_type == right_type {
            return left_type;
        }

        // Default/error case
        self.errors.push(SnowFallError::new(
            "CompilationError".to_string(),
            format!("Type mismatch between {:?} and {:?}", left_type, right_type),
            "SF021".to_string(), 0, 0,
        ));
        Type::Any
    }
}

impl Visitor for TypeChecker {
    type Output = Type;

    fn visit_node(&mut self, node: &AstNode) -> Self::Output {
        match node {
            AstNode::Program(statements) => {
                for stmt in statements {
                    self.visit_statement(stmt);
                }
                Type::Void
            }
            AstNode::Statement(stmt) => self.visit_statement(stmt),
            AstNode::Expression(expr) => self.visit_expression(expr),
        }
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Output {
        match stmt {
            Statement::Expression(expr) => self.visit_expression(expr),
            Statement::Let { name, type_name, value } => {
                let val_type = self.visit_expression(value);
                // A simplified type mapping
                let declared_type = match type_name.as_str() {
                    "Int" => Type::Int,
                    "String" => Type::String,
                    _ => Type::Any,
                };

                if declared_type != val_type {
                    // This check is too simple for a real language, but works for now.
                    // For example, it doesn't handle subtypes.
                     self.errors.push(SnowFallError::new(
                        "CompilationError".to_string(),
                        format!("Type mismatch for variable '{}'. Expected {:?}, got {:?}.", name, declared_type, val_type),
                        "SF021".to_string(), 0, 0,
                    ));
                }

                self.symbol_table.insert(name.clone(), declared_type);
                Type::Void
            }
            // TODO: Implement for other statement types
            _ => Type::Void,
        }
    }

    fn visit_expression(&mut self, expr: &Expression) -> Self::Output {
        match expr {
            Expression::IntLiteral(_) => Type::Int,
            Expression::FloatLiteral(_) => Type::Float,
            Expression::StringLiteral(_) => Type::String,
            Expression::Boolean(_) => Type::Boolean,
            Expression::Identifier(name) => self.symbol_table.get(name).cloned().unwrap_or(Type::Any),
            Expression::Infix { left, operator, right } => {
                 if let crate::compiler::Token::Dot = operator {
                    // This is a member access, for now, we don't type it
                    return Type::Any;
                }
                let op_str = match operator {
                    crate::compiler::Token::Plus => "+",
                    _ => "",
                };
                self.check_infix_expression(left, op_str, right)
            }
            Expression::Call { function, arguments: _ } => {
                if let Expression::Infix { left, operator: _, right } = &**function {
                     if let Expression::Identifier(obj_name) = &**left {
                        if let Expression::Identifier(method_name) = &**right {
                             let obj_type = self.symbol_table.get(obj_name);
                             if let Some(Type::String) = obj_type {
                                if method_name != "toUpperCase" {
                                    self.errors.push(SnowFallError::new(
                                        "CompilationError".to_string(),
                                        format!("Method '{}' not found on type String.", method_name),
                                        "SF020".to_string(), 0, 0,
                                    ));
                                }
                            }
                        }
                    }
                }
                Type::Any // Return type should be properly looked up
            }
            _ => Type::Any,
        }
    }
}
