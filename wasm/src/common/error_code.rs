use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Lexer errors
    UnexpectedCharacter,
    InvalidNumberFormat,
    UnterminatedString,

    // Parser errors
    UnexpectedToken,
    ExpectedExpression,
    ExpectedTypeName,
    ExpectedReturnType,
    ExpectedParameterType,
    UnexpectedTokenForExpression,
    ExpectedIdentifierInForEach,
    ExpectedInOrOfInForEach,
    ExpectedMemberForClass,
}

impl ErrorCode {
    pub fn to_str(&self) -> &'static str {
        match self {
            // Lexer
            ErrorCode::UnexpectedCharacter => "SF0001",
            ErrorCode::InvalidNumberFormat => "SF0002",
            ErrorCode::UnterminatedString => "SF0003",
            // Parser
            ErrorCode::UnexpectedToken => "SF0010",
            ErrorCode::ExpectedMemberForClass => "SF0011",
            ErrorCode::ExpectedTypeName => "SF0012",
            ErrorCode::ExpectedReturnType => "SF0013",
            ErrorCode::ExpectedParameterType => "SF0014",
            ErrorCode::UnexpectedTokenForExpression => "SF0015",
            ErrorCode::ExpectedIdentifierInForEach => "SF0016",
            ErrorCode::ExpectedInOrOfInForEach => "SF0017",
            ErrorCode::ExpectedExpression => "SF0018",
        }
    }

    pub fn get_default_message(&self) -> &'static str {
        match self {
            // Lexer
            ErrorCode::UnexpectedCharacter => "Unexpected character",
            ErrorCode::InvalidNumberFormat => "Invalid number format",
            ErrorCode::UnterminatedString => "Unterminated string",
            // Parser
            ErrorCode::UnexpectedToken => "Unexpected token",
            ErrorCode::ExpectedMemberForClass => "Expected 'function' or 'sub' for class member",
            ErrorCode::ExpectedTypeName => "Expected type name",
            ErrorCode::ExpectedReturnType => "Expected return type",
            ErrorCode::ExpectedParameterType => "Expected parameter type",
            ErrorCode::UnexpectedTokenForExpression => "Unexpected token for expression",
            ErrorCode::ExpectedIdentifierInForEach => "Expected identifier in for-each loop",
            ErrorCode::ExpectedInOrOfInForEach => "Expected 'in' or 'of' in for-each loop",
            ErrorCode::ExpectedExpression => "Expected expression",
        }
    }
}
