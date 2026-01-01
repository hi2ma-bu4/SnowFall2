export class Logger {
	public static isDebug = false;
	private static readonly prefix = "SnowFall";

	public static info(...args: any[]): void {
		if (this.isDebug) console.log(`[${this.prefix}]`, ...args);
	}

	public static warn(...args: any[]): void {
		console.warn(`[${this.prefix}]`, ...args);
	}

	public static error(...args: any[]): void {
		console.error(`[${this.prefix}]`, ...args);
	}
}
