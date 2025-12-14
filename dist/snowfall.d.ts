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
export declare class SnowFallCompiler {
    private config;
    constructor(options?: SnowFallConfig);
    compile(source: string): CompilationResult;
}
export declare class SnowFallExecutor {
    private config;
    constructor(options?: SnowFallConfig);
    execute(bytecode: string): string;
}
/**
 * SnowFall言語の統合システム
 */
export declare class SnowFallSystem {
    private compiler;
    private executor;
    private initialized;
    constructor(options?: SnowFallConfig);
    init(): Promise<void>;
    run(sourceCode: string): Promise<string>;
}
export {};
//# sourceMappingURL=snowfall.d.ts.map