/// `Token`構造体を簡単に作成するためのマクロ。
///
/// # 使用例
///
/// ```rust
/// let token = create_token!(TokenKind::Identifier("example".to_string()), 0, 7);
/// ```
#[macro_export]
macro_rules! create_token {
    ($kind:expr, $start:expr, $end:expr) => {
        $crate::common::Token {
            kind: $kind,
            span: $crate::common::Span {
                start: $start,
                end: $end,
            },
        }
    };
}
