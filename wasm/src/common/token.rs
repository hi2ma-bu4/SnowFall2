use serde::{Deserialize, Serialize};
use std::fmt;

use crate::common::Span;

/// 字句解析結果として扱われるトークンの種類
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TokenKind {
    // 特殊トークン (Special Tokens)
    /// 入力の終端を表すトークン
    Eof,
    /// 不正なトークン
    ///
    /// 内容にはエラーメッセージや元文字列が入る。
    Illegal(String),
    /// 識別子 (変数名・関数名・型名など)
    Identifier(String),
    /// リテラル値
    Literal(LiteralToken),
    /// 演算子
    Operator(OperatorToken),
    /// 区切り記号
    Delimiter(DelimiterToken),
    /// 予約語
    Keyword(KeywordToken),
}

/// リテラル (Literals)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LiteralToken {
    /// 整数リテラル
    Int(i64),
    /// 浮動小数点数リテラル
    Float(f64),
    /// 文字列リテラル
    String(String),
    /// 真偽値リテラル
    Boolean(bool),
}

/// 演算子 (Operators)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OperatorToken {
    // 基本的な演算子 (Basic Operators)
    /// 代入演算子 (`=`)
    Assign,
    /// 等価比較 (`==`)
    Equal,
    /// 厳密等価比較 (`===`)
    StrictEqual,
    /// 加算/プラス (`+`)
    Plus,
    /// 減算/マイナス (`-`)
    Minus,
    /// 乗算 (`*`)
    Asterisk,
    /// べき乗 (`**`)
    Power,
    /// 除算 (`/`)
    Slash,
    /// 剰余 (`%`)
    Percent,
    /// 論理否定 (`!`)
    Bang,
    /// 非等価比較 (`!=`)
    NotEqual,
    /// 厳密非等価比較 (`!==`)
    StrictNotEqual,
    /// 小なり (`<`)
    LessThan,
    /// 以下 (`<=`)
    LessThanOrEqual,
    /// 大なり (`>`)
    GreaterThan,
    /// 以上 (`>=`)
    GreaterThanOrEqual,

    // 論理演算子 (Logical Operators)
    /// 論理AND (`&&`)
    LogicalAnd,
    /// 論理OR (`||`)
    LogicalOr,

    // ビット演算子 (Bitwise Operators)
    /// ビットAND (`&`)
    BitwiseAnd,
    /// ビットOR (`|`)
    BitwiseOr,
    /// ビットXOR (`^`)
    BitwiseXor,
    /// ビットNOT (`~`)
    BitwiseNot,
    /// 左シフト (`<<`)
    BitwiseLeftShift,
    /// 符号なし左シフト (`<<<`)
    BitwiseUnsignedLeftShift,
    /// 右シフト (`>>`)
    BitwiseRightShift,
    /// 符号なし右シフト (`>>>`)
    BitwiseUnsignedRightShift,
}

/// 境界記号 (Delimiters)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DelimiterToken {
    /// ドット (`.`)
    Dot,
    /// カンマ (`,`)
    Comma,
    /// コロン (`:`)
    Colon,
    /// セミコロン (`;`)
    Semicolon,
    /// 左丸括弧 (`(`)
    LParen,
    /// 右丸括弧 (`)`)
    RParen,
    /// 左波括弧 (`{`)
    LBrace,
    /// 右波括弧 (`}`)
    RBrace,
    /// 左角括弧 (`[`)
    LBracket,
    /// 右角括弧 (`]`)
    RBracket,
}

/// キーワード (Keywords)
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum KeywordToken {
    /// 関数定義 (`function`)
    Function,
    /// 戻り値なし関数定義 (`sub`)
    Sub,
    /// クラス定義 (`class`)
    Class,
    /// 継承指定 (`extends`)
    Extends,
    /// コンストラクタ (`constructor`)
    Constructor,
    /// インスタンス生成 (`new`)
    New,
    /// 条件分岐 (`if`)
    If,
    /// else節
    Else,
    /// forループ
    For,
    /// whileループ
    While,
    /// in演算子
    In,
    /// of演算子
    Of,
    /// switch文
    Switch,
    /// case節
    Case,
    /// default節
    Default,
    /// break文
    Break,
    /// continue文
    Continue,
    /// return文
    Return,
    /// 真 (`true`)
    True,
    /// 偽 (`false`)
    False,
    /// null値
    Null,
    /// 論理AND (`and`)
    And,
    /// 論理OR (`or`)
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    /// EOF（入力終端）トークンを生成する
    pub fn eof(pos: usize) -> Self {
        Token {
            kind: TokenKind::Eof,
            span: Span {
                start: pos,
                end: pos,
            },
        }
    }
}
