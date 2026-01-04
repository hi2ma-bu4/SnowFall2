/* ================================================== */
/* 共通利用 */
/* ================================================== */

export interface Span {
	start: Number;
	end: Number;
}

export interface SnowFallErrorWasm {
	type: String;
	message: String;
	code: String;
	line: Number;
	Column: Number;
	trace: any[];
	context?: Map<String, String>;
}

/* ================================================== */
/* Lexer使用 */
/* ================================================== */

export type LiteralToken =
	| { type: "Int" | "Float"; value: Number } //
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
	| { type: "Identifier"; value: string }
	| { type: "Literal"; value: LiteralToken }
	| { type: "Operator"; value: OperatorToken }
	| { type: "Delimiter"; value: DelimiterToken }
	| { type: "Keyword"; value: KeywordToken };

export interface Token {
	kind: TokenKind;
	span: Span;
}

/* ================================================== */
/* Parser使用 */
/* ================================================== */

export type PrefixOperator =
	| "Plus" //
	| "Minus"
	| "Bang"
	| "BitwiseNot";
export type InfixOperator =
	| "Add" //
	| "Subtract"
	| "Multiply"
	| "Divide"
	| "Modulo"
	| "Power"
	| "Equals"
	| "NotEquals"
	| "StrictEquals"
	| "StrictNotEquals"
	| "LessThan"
	| "GreaterThan"
	| "LessThanOrEqual"
	| "GreaterThanOrEqual"
	| "LogicalAnd"
	| "LogicalOr"
	| "LogicalAndAlso"
	| "LogicalOrElse"
	| "BitwiseAnd"
	| "BitwiseOr"
	| "BitwiseXor"
	| "BitwiseLeftShift"
	| "BitwiseRightShift"
	| "BitwiseUnsignedLeftShift"
	| "BitwiseUnsignedRightShift";

export interface Prefix {
	operator: PrefixOperator;
	right: Expression;
}
export interface Infix {
	left: Expression;
	operator: InfixOperator;
	right: Expression;
}
export interface Call {
	function: Expression;
	arguments: Expression[];
}
export interface Cast {
	target_type: String;
	expression: Expression;
}
export interface Index {
	left: Expression;
	index: Expression;
}
export interface Member {
	left: Expression;
	property: String;
}
export interface Assignment {
	left: Expression;
	right: Expression;
}
export interface MemberAccess {
	object: Expression;
	property: Expression;
	computed: Boolean;
}
export interface New {
	class: Expression;
	arguments: Expression[];
}

export type ExpressionKind =
	| { type: "IntLiteral"; value: number } //
	| { type: "FloatLiteral"; value: number }
	| { type: "StringLiteral"; value: String }
	| { type: "Boolean"; value: Boolean }
	| { type: "Identifier"; value: String }
	| { type: "Prefix"; value: Prefix }
	| { type: "Infix"; value: Infix }
	| { type: "Call"; value: Call }
	| { type: "Cast"; value: Cast }
	| { type: "NullLiteral" }
	| { type: "ArrayLiteral"; value: Expression[] }
	| { type: "ObjectLiteral"; value: [Expression, Expression][] }
	| { type: "Index"; value: Index }
	| { type: "Assignment"; value: Assignment }
	| { type: "MemberAccess"; value: MemberAccess }
	| { type: "New"; value: New };

export interface Expression {
	kind: ExpressionKind;
	span: Span;
}

export interface VariableDeclarator {
	name: String;
	value?: Expression;
}
export interface Parameter {
	name: String;
	type_name: String;
	value?: Expression;
}
export interface Binding {
	name: String;
	type_name: String;
}
export type FunctionKind = "Function" | "Sub";
export type ForEachKind = "In" | "Of";
export interface SwitchCase {
	values: Expression[];
	body: Statement;
}

export interface VariableDeclaration {
	type_name: String;
	declarators: VariableDeclarator[];
}
export interface FunctionDeclaration {
	kind: FunctionKind;
	name: String;
	return_type?: String;
	params: Parameter[];
	body: Statement;
}
export interface ClassDeclaration {
	name: String;
	superclass?: String;
	members: Statement[];
}
export interface If {
	condition: Expression;
	consequence: Statement;
	alternative?: Statement;
}
export interface For {
	init?: Statement;
	condition?: Expression;
	update?: Statement;
	body: Statement;
}
export interface ForEach {
	binding: Binding;
	iterable: Expression;
	kind: ForEachKind;
	body: Statement;
}
export interface Switch {
	expression: Expression;
	cases: SwitchCase[];
	default?: Statement;
}

export type StatementKind =
	| { type: "VariableDeclaration"; value: VariableDeclaration } //
	| { type: "FunctionDeclaration"; value: FunctionDeclaration }
	| { type: "ClassDeclaration"; value: ClassDeclaration }
	| { type: "If"; value: If }
	| { type: "For"; value: For }
	| { type: "ForEach"; value: ForEach }
	| { type: "Switch"; value: Switch }
	| { type: "Return"; value?: Expression }
	| { type: "Break" }
	| { type: "Continue" }
	| { type: "Block"; value: Statement[] }
	| { type: "Expression"; value: Expression };

export interface Statement {
	kind: StatementKind;
	span: Span;
}

export interface ProgramAst {
	statements: Statement[];
	span: Span;
}

export interface ParserResult {
	ast?: ProgramAst;
	errors?: SnowFallErrorWasm[];
}
