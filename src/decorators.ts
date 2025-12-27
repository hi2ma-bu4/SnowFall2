/**
 * @fileoverview
 * このファイルは、SnowFall言語のデコレーター（`@foo`）の
 * コンパイル時メタプログラミングをサポートするためのTypeScript側の
 * 抽象化構造を定義します。
 *
 * ## 設計思想と拡張性
 * Rustコンパイラは、ソースコードのAST（抽象構文木）を解析する際に
 * デコレーターを検出します。検出されたデコレーターの情報
 * （名前、引数、デコレート対象のノード情報など）は、TS側に送られます。
 *
 * TS側では、デコレーター名に対応する`ICompilerDecorator`の実装クラスを
 * 探し、その`process`メソッドを実行します。`process`メソッドは、
 * ASTノードを修正・ラップし、新しいASTノードをRust側に返却します。
 *
 * この設計により、新しいデコレーター（例: `@async`, `@deprecated`）の
 * ロジックをTypeScript側で完結して追加できます。Rustコンパイラ本体は
 * デコレーターの存在を検出し、TSの処理を呼び出すだけでよく、
 * 個々のデコレーターの詳細なロジックを知る必要がありません。
 * これにより、言語機能の拡張が容易になります。
 */

/**
 * デコレートされるASTノードの基本的な情報。
 * Rustから渡されることを想定しています。
 */
export interface AstNodeInfo {
	nodeType: "Class" | "Method" | "Property";
	name: string;
	// その他、ASTノードに関する詳細情報
}

/**
 * すべてのコンパイラデコレーターが実装すべきインターフェース。
 */
export interface ICompilerDecorator {
	/**
	 * デコレーターの名前 (例: "override", "logging")。
	 */
	readonly name: string;

	/**
	 * デコレーターのロジックを実行し、ASTノードを変換します。
	 * @param targetNode デコレートされる対象のASTノード
	 * @param args デコレーターに渡された引数
	 * @returns 変換後の新しいASTノード (または元のノード)
	 */
	process(targetNode: AstNodeInfo, ...args: any[]): AstNodeInfo;
}

/**
 * メソッドデコレーターのロジックをカプセル化するための抽象クラス。
 * 共通の処理を提供し、具体的なデコレーターは`hook`メソッドを実装します。
 */
export abstract class MethodDecoratorProcessor implements ICompilerDecorator {
	abstract readonly name: string;

	public process(targetNode: AstNodeInfo, ...args: any[]): AstNodeInfo {
		if (targetNode.nodeType !== "Method") {
			// 本来はコンパイルエラーとしてRust側に返すべき
			throw new Error(`Decorator @${this.name} can only be applied to methods.`);
		}

		// メソッドの実行前、実行後、または全体をラップするロジックを注入
		return this.hook(targetNode, ...args);
	}

	/**
	 * 具体的なメソッドデコレーターが実装すべき抽象メソッド。
	 * ここでASTノードを操作し、新しいロジックを追加します。
	 *
	 * @param methodNode メソッドのASTノード
	 * @param args デコレーターの引数
	 * @returns 変換後のメソッドASTノード
	 */
	protected abstract hook(methodNode: AstNodeInfo, ...args: any[]): AstNodeInfo;
}

// --- 具体的なデコレーターの実装例 ---

/**
 * `@override` デコレーターのプロセッサ。
 * このデコレーターは、Rust側での静的検証のマーカーとしてのみ機能するため、
 * TS側ではASTを何も変更せずにそのまま返します。
 */
export class OverrideDecorator extends MethodDecoratorProcessor {
	public readonly name = "override";

	protected hook(methodNode: AstNodeInfo): AstNodeInfo {
		// `@override` はRustコンパイラが親クラスのメソッド存在を
		// チェックするためのマーカー。TS側でのAST変換は不要。
		console.log(`[Compiler] Processing @override for method: ${methodNode.name}`);
		return methodNode;
	}
}
