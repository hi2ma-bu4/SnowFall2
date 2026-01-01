use crate::common::{Span, Token};
use serde::{Deserialize, Serialize};

/// 位置情報付きノード
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

/// 抽象構文木のノード
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AstNode {
    /// 抽象構文木のルート(全体)
    Program(Vec<Statement>),
    /// 単一の文のラッパー
    Statement(Statement),
    /// 単一の式のラッパー
    Expression(Expression),
}

/// 抽象構文木の文ノード
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// 変数定義 (Int a = 10;)
    Let { binding: Binding, value: Expression },
    /// ループやswitchからの脱出 (break;)
    Break,
    /// 続行文 (continue;)
    Continue,
    /// 返却文 (return x + 1;)
    Return(Expression),
    /// 式 (なんでも)
    Expression(Expression),
    /// 関数定義 (function Int add(Int a, Int b) /* ... */)
    Function {
        name: String,
        params: Vec<Binding>,
        body: Box<Statement>,
        return_type: Option<String>, // None => Subtitle
    },
    /// 条件分岐 (if (condition) /* ... */ else /* ... */)
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    /// ループ文 (for (init; condition; update) /* ... */)
    For {
        init: Option<Box<Statement>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Statement>>,
        body: Box<Statement>,
    },
    /// イテレートループ文 (for (identifier in iterable) /* ... */)
    ForEach {
        binding: Binding,
        iterable: Box<Expression>,
        kind: ForEachKind,
        body: Box<Statement>,
    },
    /// 条件付きループ文 (while (condition) /* ... */)
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    /// switch文 (switch (expression) { case /* ... */ })
    Switch {
        expression: Box<Expression>,
        cases: Vec<SwitchCase>,
        default: Option<Box<Statement>>,
    },

    /// ブロック文 ({ /* ... */ })
    Block(Vec<Statement>),
}

/// 変数のバインディング情報
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub name: String,
    pub type_name: Option<String>,
}

/// switch文のケース
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub values: Vec<Expression>,
    pub body: Box<Statement>,
}

/// for...in / for...of の種類
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForEachKind {
    In,
    Of,
}

/// 抽象構文木の式ノード
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// 変数
    Identifier(String),
    // 型
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Boolean(bool),
    /// 前置演算子 (-x, !flag)
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    /// 中置演算子 (x + y, a == b)
    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    /// 関数またはサブルーチンの呼び出し (add(1, 2))
    Call {
        function: Box<Expression>, // 識別子または別の呼び出し
        arguments: Vec<Expression>,
    },
    /// 型変換 ( (Int) x; )
    Cast {
        target_type: String,
        expression: Box<Expression>,
    },
}

// ASTノードを訪問するためのVisitorトレイト
pub trait Visitor {
    type Output;

    /// ASTノードの入口
    fn visit_node(&mut self, node: &AstNode) -> Self::Output;
    /// 文単位の処理
    fn visit_statement(&mut self, stmt: &Statement) -> Self::Output;
    /// 式単位の処理
    fn visit_expression(&mut self, expr: &Expression) -> Self::Output;
}
