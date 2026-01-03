use crate::common::Span;
use serde::{Deserialize, Serialize};

/// プログラム全体を表すノード
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// 文 (Statement)
/// 位置情報(span)と種類(kind)を保持します。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

/// 抽象構文木の文ノード
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StatementKind {
    /// 変数宣言: `Int a = 1, b = 2;`
    VariableDeclaration {
        type_name: String,
        declarators: Vec<VariableDeclarator>,
    },
    /// 関数定義 `function Int add(Int a, Int b) /* ... */`
    FunctionDeclaration {
        kind: FunctionKind, // Function or Sub
        name: String,
        return_type: Option<String>, // Subの場合はNone
        params: Vec<Parameter>,
        body: Box<Statement>, // Block
    },
    /// クラス定義 `class MyClass extends Base { /* ... */ }`
    ClassDeclaration {
        name: String,
        superclass: Option<String>,
        members: Vec<Statement>,
    },

    /// 条件分岐 `if (condition) /* ... */ else /* ... */`
    If {
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    /// ループ文 `for (init; condition; update) /* ... */`
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Box<Statement>>,
        body: Box<Statement>,
    },
    /// イテレートループ文 `for (identifier in iterable) /* ... */`
    ForEach {
        binding: Binding,
        iterable: Expression,
        kind: ForEachKind,
        body: Box<Statement>,
    },
    /// 条件付きループ文 `while (condition) /* ... */`
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    /// switch文 `switch (expression) { case /* ... */ }`
    Switch {
        expression: Expression,
        cases: Vec<SwitchCase>,
        default: Option<Box<Statement>>,
    },

    /// 返却文 `return x + 1;`
    Return(Option<Expression>),
    /// ループやswitchからの脱出 `break;`
    Break,
    /// 続行文 `continue;`
    Continue,

    /// ブロック文 `{ /* ... */ }`
    Block(Vec<Statement>),

    /// 式 (なんでも)
    Expression(Expression),
}

/// 変数宣言の1要素 (例: `a = 1`)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub name: String,
    pub value: Option<Expression>, // 初期化式がない場合 (`Int a;`) も考慮
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_name: String, // 引数は型必須と仮定 (`Int a`)
    pub value: Option<Box<Statement>>,
}

/// 変数のバインディング情報
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub name: String,
    pub type_name: Option<String>,
}

/// function / sub の種類
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FunctionKind {
    Function,
    Sub,
}

/// for...in / for...of の種類
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForEachKind {
    In,
    Of,
}

/// switch文のケース
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub values: Vec<Expression>,
    pub body: Box<Statement>,
}

/// 式 (Expression)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

/// 抽象構文木の式ノード
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExpressionKind {
    // 型
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Boolean(bool),

    /// 変数
    Identifier(String),

    /// 前置演算子 `-x`, `!flag`
    Prefix {
        operator: PrefixOperator,
        right: Box<Expression>,
    },
    /// 中置演算子 `x + y`, `a == b`
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    /// 関数またはサブルーチンの呼び出し `add(1, 2)`
    Call {
        function: Box<Expression>, // 識別子または別の呼び出し
        arguments: Vec<Expression>,
    },
    /// 型変換 `(Int) x;`
    Cast {
        target_type: String,
        expression: Box<Expression>,
    },

    /// `null` リテラル
    NullLiteral,
    /// 配列リテラル `[1, "two", true]`
    ArrayLiteral(Vec<Expression>),
    /// オブジェクトリテラル `{"key": value}`
    ObjectLiteral {
        pairs: Vec<(Expression, Expression)>,
    },
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    /// メンバーアクセス: `obj.prop`
    Member {
        left: Box<Expression>,
        property: String,
    },
    /// 代入式 `a = 10`, `obj.prop = 20`
    Assignment {
        left: Box<Expression>, // Identifier or MemberAccess
        right: Box<Expression>,
    },
    /// メンバーアクセス `obj.prop` or `arr[0]`
    MemberAccess {
        object: Box<Expression>,
        property: Box<Expression>, // Identifier or Literal
        computed: bool,            // true for `[]`, false for `.`
    },
    /// `new` 式 `new MyClass()`
    New {
        class: Box<Expression>, // Should resolve to a class identifier
        arguments: Vec<Expression>,
    },
}

/// 前置演算子一覧
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Bang,
    BitwiseNot,
}

/// 中置演算子一覧
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InfixOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equals,
    NotEquals,
    StrictEquals,
    StrictNotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalAndAlso,
    LogicalOrElse,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseUnsignedLeftShift,
    BitwiseUnsignedRightShift,
}
