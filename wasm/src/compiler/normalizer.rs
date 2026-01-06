use crate::compiler::ast::*;
use std::cmp::Ordering;

/// オペランドをソートするためのキー
#[derive(Debug, Eq)]
enum SortKey {
    Identifier(String),
    Literal(String),
    Other,
}

/// `SortKey` の全順序を定義
impl Ord for SortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (SortKey::Identifier(a), SortKey::Identifier(b)) => a.cmp(b),
            (SortKey::Literal(a), SortKey::Literal(b)) => a.cmp(b),
            (SortKey::Identifier(_), SortKey::Literal(_)) => Ordering::Less,
            (SortKey::Literal(_), SortKey::Identifier(_)) => Ordering::Greater,
            (SortKey::Other, SortKey::Other) => Ordering::Equal,
            (_, SortKey::Other) => Ordering::Less,
            (SortKey::Other, _) => Ordering::Greater,
        }
    }
}

/// `Ord` に基づく完全順序を常に返す
impl PartialOrd for SortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `Ord` の結果が `Equal` かどうかで等価性を判定する
impl PartialEq for SortKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

/// 式からソート用キーを生成する
fn get_sort_key(expr: &Expression) -> SortKey {
    match &expr.kind {
        ExpressionKind::Identifier(name) => SortKey::Identifier(name.clone()),
        ExpressionKind::IntLiteral(val) => SortKey::Literal(val.to_string()),
        ExpressionKind::FloatLiteral(val) => SortKey::Literal(val.to_string()),
        ExpressionKind::StringLiteral(val) => SortKey::Literal(val.clone()),
        ExpressionKind::Boolean(val) => SortKey::Literal(val.to_string()),
        ExpressionKind::NullLiteral => SortKey::Literal("null".to_string()),
        _ => SortKey::Other,
    }
}

/// 演算が可換であるかを判定する
fn is_commutative(op: &InfixOperator) -> bool {
    matches!(op, InfixOperator::Add | InfixOperator::Multiply)
}

/// 指定した演算子に対して、
/// ネストされた同一演算子の式を再帰的にフラット化しオペランドを収集する
///
/// `(a + (b + c))` → `[a, b, c]`
fn collect_operands(expr: Expression, op: &InfixOperator, operands: &mut Vec<Expression>) {
    match expr {
        Expression {
            kind:
                ExpressionKind::Infix {
                    left,
                    operator,
                    right,
                },
            ..
        } if &operator == op => {
            collect_operands(*left, op, operands);
            collect_operands(*right, op, operands);
        }
        _ => {
            operands.push(expr);
        }
    }
}

/// ソート済みオペランド列から左結合の AST を再構築する。
///
/// `a, b, c` → `((a op b) op c)`
fn rebuild_tree(mut operands: Vec<Expression>, op: InfixOperator) -> Expression {
    let mut left = operands.remove(0);
    while !operands.is_empty() {
        let right = operands.remove(0);
        let span = left.span.clone(); // TODO: Spanをコピーしているため正しくない
        left = Expression {
            span,
            kind: ExpressionKind::Infix {
                left: Box::new(left),
                operator: op.clone(),
                right: Box::new(right),
            },
        };
    }
    left
}

/// 正規化処理のエントリポイント
///
/// - 文・式を再帰的に正規化
/// - 不要になった文を削除
pub fn normalize(mut program: ProgramAst) -> ProgramAst {
    let mut new_statements = Vec::new();
    for stmt in program.statements {
        if let Some(normalized_stmt) = normalize_statement(stmt) {
            new_statements.push(normalized_stmt);
        }
    }
    program.statements = new_statements;
    program
}

/// 単一の文を正規化する。
///
/// - 定数条件の if を畳み込み
/// - 不要な文は `None` を返す
fn normalize_statement(stmt: Statement) -> Option<Statement> {
    let new_kind = match stmt.kind {
        StatementKind::VariableDeclaration {
            type_name,
            declarators,
        } => StatementKind::VariableDeclaration {
            type_name,
            declarators: declarators
                .into_iter()
                .map(|decl| VariableDeclarator {
                    name: decl.name,
                    value: decl.value.map(normalize_expression),
                })
                .collect(),
        },
        StatementKind::Expression(expr) => StatementKind::Expression(normalize_expression(expr)),
        StatementKind::Block(stmts) => {
            let new_stmts = stmts.into_iter().filter_map(normalize_statement).collect();
            StatementKind::Block(new_stmts)
        }
        StatementKind::If {
            condition,
            consequence,
            alternative,
        } => {
            let normalized_condition = normalize_expression(condition);
            if let ExpressionKind::Boolean(val) = normalized_condition.kind {
                if val {
                    // if(true)なので、consequenceを返す
                    return normalize_statement(*consequence);
                } else {
                    // if(false)なので、alternativeを返す
                    if let Some(alt) = alternative {
                        return normalize_statement(*alt);
                    } else {
                        // if(false)でelseがない場合は、文自体を削除
                        return None;
                    }
                }
            }
            // 条件が定数でない場合は、各ブロックを正規化
            let consequence_span = consequence.span;
            let normalized_consequence = Box::new(
                normalize_statement(*consequence).unwrap_or_else(|| Statement {
                    kind: StatementKind::Block(vec![]),
                    span: consequence_span,
                }),
            );
            let normalized_alternative =
                alternative.and_then(|alt| normalize_statement(*alt).map(Box::new));

            StatementKind::If {
                condition: normalized_condition,
                consequence: normalized_consequence,
                alternative: normalized_alternative,
            }
        }
        StatementKind::Return(Some(expr)) => {
            StatementKind::Return(Some(normalize_expression(expr)))
        }
        // 他の文は今のところそのまま
        _ => stmt.kind,
    };
    Some(Statement {
        kind: new_kind,
        span: stmt.span,
    })
}

/// 式を正規化する
fn normalize_expression(expr: Expression) -> Expression {
    // 式の子を再帰的に正規化
    let kind = match expr.kind {
        ExpressionKind::Infix {
            left,
            operator,
            right,
        } => ExpressionKind::Infix {
            left: Box::new(normalize_expression(*left)),
            operator,
            right: Box::new(normalize_expression(*right)),
        },
        ExpressionKind::Prefix { operator, right } => ExpressionKind::Prefix {
            operator,
            right: Box::new(normalize_expression(*right)),
        },
        ExpressionKind::Call {
            function,
            arguments,
        } => ExpressionKind::Call {
            function: Box::new(normalize_expression(*function)),
            arguments: arguments.into_iter().map(normalize_expression).collect(),
        },
        // リテラルと識別子には正規化する子がありません
        kind => kind,
    };

    match kind {
        ExpressionKind::Prefix { operator, right } => {
            let folded_kind = match (operator, right.kind) {
                (PrefixOperator::Minus, ExpressionKind::IntLiteral(val)) => {
                    Some(ExpressionKind::IntLiteral(-val))
                }
                (PrefixOperator::Plus, ExpressionKind::IntLiteral(val)) => {
                    Some(ExpressionKind::IntLiteral(val))
                }
                (PrefixOperator::Minus, ExpressionKind::FloatLiteral(val)) => {
                    Some(ExpressionKind::FloatLiteral(-val))
                }
                (PrefixOperator::Plus, ExpressionKind::FloatLiteral(val)) => {
                    Some(ExpressionKind::FloatLiteral(val))
                }
                (operator, kind) => Some(ExpressionKind::Prefix {
                    operator,
                    right: Box::new(Expression {
                        kind,
                        span: right.span,
                    }),
                }),
            };
            Expression {
                kind: folded_kind.unwrap(),
                span: expr.span,
            }
        }
        ExpressionKind::Infix {
            left,
            operator,
            right,
        } => {
            let folded_kind = match (&left.kind, &operator, &right.kind) {
                // Int and Int
                (ExpressionKind::IntLiteral(l), op, ExpressionKind::IntLiteral(r)) => match op {
                    InfixOperator::Add => Some(ExpressionKind::IntLiteral(l + r)),
                    InfixOperator::Subtract => Some(ExpressionKind::IntLiteral(l - r)),
                    InfixOperator::Multiply => Some(ExpressionKind::IntLiteral(l * r)),
                    InfixOperator::Divide => Some(ExpressionKind::IntLiteral(l / r)),
                    _ => None,
                },
                // Float and Float
                (ExpressionKind::FloatLiteral(l), op, ExpressionKind::FloatLiteral(r)) => {
                    match op {
                        InfixOperator::Add => Some(ExpressionKind::FloatLiteral(l + r)),
                        InfixOperator::Subtract => Some(ExpressionKind::FloatLiteral(l - r)),
                        InfixOperator::Multiply => Some(ExpressionKind::FloatLiteral(l * r)),
                        InfixOperator::Divide => Some(ExpressionKind::FloatLiteral(l / r)),
                        _ => None,
                    }
                }
                // Int and Float
                (ExpressionKind::IntLiteral(l), op, ExpressionKind::FloatLiteral(r)) => {
                    let l_float = *l as f64;
                    match op {
                        InfixOperator::Add => Some(ExpressionKind::FloatLiteral(l_float + r)),
                        InfixOperator::Subtract => Some(ExpressionKind::FloatLiteral(l_float - r)),
                        InfixOperator::Multiply => Some(ExpressionKind::FloatLiteral(l_float * r)),
                        InfixOperator::Divide => Some(ExpressionKind::FloatLiteral(l_float / r)),
                        _ => None,
                    }
                }
                // Float and Int
                (ExpressionKind::FloatLiteral(l), op, ExpressionKind::IntLiteral(r)) => {
                    let r_float = *r as f64;
                    match op {
                        InfixOperator::Add => Some(ExpressionKind::FloatLiteral(l + r_float)),
                        InfixOperator::Subtract => Some(ExpressionKind::FloatLiteral(l - r_float)),
                        InfixOperator::Multiply => Some(ExpressionKind::FloatLiteral(l * r_float)),
                        InfixOperator::Divide => Some(ExpressionKind::FloatLiteral(l / r_float)),
                        _ => None,
                    }
                }
                _ => None,
            };

            if let Some(kind) = folded_kind {
                return Expression {
                    kind,
                    span: expr.span,
                };
            }

            if is_commutative(&operator) {
                let temp_expr = Expression {
                    span: expr.span,
                    kind: ExpressionKind::Infix {
                        left,
                        operator: operator.clone(),
                        right,
                    },
                };
                let mut operands = Vec::new();
                collect_operands(temp_expr, &operator, &mut operands);
                operands.sort_by(|a, b| get_sort_key(a).cmp(&get_sort_key(b)));
                return rebuild_tree(operands, operator.clone());
            }
            Expression {
                kind: ExpressionKind::Infix {
                    left,
                    operator,
                    right,
                },
                span: expr.span,
            }
        }
        _ => Expression {
            kind,
            span: expr.span,
        },
    }
}
