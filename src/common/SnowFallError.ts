import type { ISnowFallError, SnowFallErrorContext } from "../common/types";

export class SnowFallError extends Error implements ISnowFallError {
	public readonly type: string;
	public readonly code: string;
	public readonly line: number;
	public readonly column: number;
	public readonly trace: string[];
	public readonly context?: SnowFallErrorContext;

	constructor(error: ISnowFallError) {
		super(error.message);
		this.name = this.constructor.name;
		this.type = error.type;
		this.code = error.code;
		this.line = error.line;
		this.column = error.column;
		this.trace = error.trace;
		this.context = error.context;

		// V8（Node.js、Chrome）でスタックトレースを正しくキャプチャするための設定
		if (Error.captureStackTrace) {
			Error.captureStackTrace(this, this.constructor);
		}
	}
}
