export interface Span {
	start: number;
	end: number;
}

export type LiteralToken =
	| { type: "Int" | "Float"; value: number } //
	| { type: "String"; value: String }
	| { type: "Boolean"; value: Boolean };
export type OperatorToken = {
	type:
		| "Assign" //
		| "Equal"
		| "StrictEqual"
		| "Plus"
		| "Minus"
		| "Asterisk"
		| "Power"
		| "Slash"
		| "Percent"
		| "Bang"
		| "NotEqual"
		| "StrictNotEqual"
		| "LessThan"
		| "LessThanOrEqual"
		| "GreaterThan"
		| "GreaterThanOrEqual"
		| "LogicalAnd"
		| "LogicalOr"
		| "BitwiseAnd"
		| "BitwiseOr"
		| "BitwiseXor"
		| "BitwiseNot"
		| "BitwiseLeftShift"
		| "BitwiseUnsignedLeftShift"
		| "BitwiseRightShift"
		| "BitwiseUnsignedRightShift";
};
export type DelimiterToken = {
	type:
		| "Dot" //
		| "Comma"
		| "Colon"
		| "Semicolon"
		| "LParen"
		| "RParen"
		| "LBrace"
		| "RBrace"
		| "LBracket"
		| "RBracket";
};
export type KeywordToken = {
	type:
		| "Function" //
		| "Sub"
		| "Class"
		| "Extends"
		| "Constructor"
		| "New"
		| "If"
		| "Else"
		| "For"
		| "While"
		| "In"
		| "Of"
		| "Switch"
		| "Case"
		| "Default"
		| "Break"
		| "Continue"
		| "Return"
		| "True"
		| "False"
		| "Null"
		| "And"
		| "Or";
};

export type TokenKind =
	| { type: "Eof" } //
	| { type: "Illegal" | "Identifier"; value: string }
	| { type: "Literal"; value: LiteralToken }
	| { type: "Operator"; value: OperatorToken }
	| { type: "Delimiter"; value: DelimiterToken }
	| { type: "Keyword"; value: KeywordToken };

export interface Token {
	kind: TokenKind;
	span: Span;
}
