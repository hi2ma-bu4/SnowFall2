/// `Token`構造体を簡単に作成するためのマクロ。
///
/// # 使用例
///
/// ```rust
/// use snowfall_core::common::TokenKind;
/// use snowfall_core::create_token;
/// let token = create_token!(TokenKind::Identifier("example".to_string()), 0, 7);
/// assert!(matches!(token.kind, TokenKind::Identifier(s) if s == "example"));
/// assert_eq!(token.span.start, 0);
/// assert_eq!(token.span.end, 7);
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
