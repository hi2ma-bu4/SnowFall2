import init, { compile_source, execute_bytecode } from "../pkg/snowfall_core";

export interface SnowFallConfig {
	debug?: boolean;
	maxRecursion?: number;
}

interface CompilationResult {
	success: boolean;
	bytecode: string;
	error_msg?: string;
	debug_info?: string;
}

export class SnowFallCompiler {
	private config: { debug_mode: boolean };

	constructor(options: SnowFallConfig = {}) {
		this.config = {
			debug_mode: options.debug || false,
		};
	}

	public compile(source: string): CompilationResult {
		const resultJson = compile_source(source, JSON.stringify(this.config));
		return JSON.parse(resultJson);
	}
}

export class SnowFallExecutor {
	private config: { max_recursion: number; debug_mode: boolean };

	constructor(options: SnowFallConfig = {}) {
		this.config = {
			max_recursion: options.maxRecursion || 10000,
			debug_mode: options.debug || false,
		};
	}

	public execute(bytecode: string): string {
		return execute_bytecode(bytecode, JSON.stringify(this.config));
	}
}

/**
 * SnowFall言語の統合システム
 */
export class SnowFallSystem {
	private compiler: SnowFallCompiler;
	private executor: SnowFallExecutor;
	private initialized = false;

	constructor(options?: SnowFallConfig) {
		this.compiler = new SnowFallCompiler(options);
		this.executor = new SnowFallExecutor(options);
	}

	public async init() {
		if (!this.initialized) {
			await init();
			this.initialized = true;
		}
	}

	public async run(sourceCode: string): Promise<string> {
		if (!this.initialized) await this.init();

		// コンパイル
		const compileRes = this.compiler.compile(sourceCode);
		if (!compileRes.success) {
			return `[Error] Compilation Failed: ${compileRes.error_msg}`;
		}

		if (compileRes.debug_info) {
			console.log("[DEBUG] Bytecode:", compileRes.bytecode);
		}

		// 実行
		return this.executor.execute(compileRes.bytecode);
	}
}

if (typeof window !== "undefined") {
	(window as any).SnowFallSystem = SnowFallSystem;
}
