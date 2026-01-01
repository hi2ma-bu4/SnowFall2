//! 共通利用モジュール
//!
pub mod constants;
pub mod error;
pub mod macros;
pub mod span;
pub mod token;

pub use span::Span;
pub use token::DelimiterToken;
pub use token::KeywordToken;
pub use token::LiteralToken;
pub use token::OperatorToken;
pub use token::Token;
pub use token::TokenKind;
