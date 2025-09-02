# Frontend integration for `crates/wasm`

このディレクトリには、frontendでWASMを初期化・利用するための最小コード雛形があります。これらを`frontend`配下にコピーし、依存関係をリンクすれば、すぐに`wasm`クレートの関数を呼び出せます。

## 1) 依存のリンク（モノレポ想定）
- `frontend/package.json`にローカル依存を追加します。

```jsonc
{
  "dependencies": {
    // 既存の依存...
    "wasm": "file:../backend/crates/wasm/pkg"
  }
}
```

- その後、フロントエンド側でインストールを実行します。

```bash
# frontend ディレクトリで実行
npm install
# または
pnpm install
# または
yarn install
```

Vite/Next.js (webpack5)/Create React App のいずれでも `--target web` でビルドした`pkg`は動作します。

補足: すでに`wasm-pack build crates/wasm --target web`で`pkg`が生成済みであることが前提です。

## 2) 初期化ラッパの配置
- `docs/frontend-integration/src/wasm/bridge.ts` を `frontend/src/wasm/bridge.ts` へコピーします。
- React等のフレームワークを使っていない場合でも、ESMとして利用可能です。

```
frontend/
  src/
    wasm/
      bridge.ts  // これを配置
```

## 3) 使い方（例）
- 任意のコンポーネントやモジュールでWASMを初期化し、エクスポート関数を呼び出します。

```ts
// frontend/src/main.ts or any component
import { getWasm } from './wasm/bridge';

async function run() {
  const wasm = await getWasm();

  // 例: 線形方程式を解く（rows x cols 行列Aとベクトルb）
  const rows = 2, cols = 2;
  const A = new Float64Array([
    3, 2,
    1, 2,
  ]);
  const b = new Float64Array([5, 5]);
  const x = wasm.solveLinearSystem(rows, cols, A, b); // Float64Array
  console.log('x =', Array.from(x));

  // 例: 正規分布のPDFをSVGで取得
  const normal = new wasm.WasmNormal(0, 1);
  const svg = normal.pdf_svg(480, 240, 200);
  document.querySelector('#app')!.innerHTML = svg;
}
run();
```

- Viteなどで開発サーバを起動すれば、`pkg/wasm_bg.wasm`はESMの`wasm.js`から相対解決されます（`import.meta.url`で解決）。

## 4) トラブルシューティング
- 404で`wasm_bg.wasm`が見つからない:
  - 依存を`file:../backend/crates/wasm/pkg`として正しくlinkしているか確認
  - lockfile削除→再インストールを試す
- TypeScriptで型が見えない:
  - `wasm/pkg/wasm.d.ts`が型として認識されるため、`node_modules/wasm`配下に正しく配置されているか確認
- 二重初期化を避けたい:
  - `bridge.ts`側でシングルトン化しているので、そのまま使えばOK

## 5) Nodeで実行したい場合（参考）
- `--target nodejs`で別ビルドを作るか、Bundler（Vite/webpack）が`web`ターゲットWASMを解決する設定を用いる
- 典型例: `import init, * as wasm from 'wasm'; await init();` でOK

---

この雛形は、当面の「WASMを受け取って初期化する」最小構成です。要件に応じて、API層のラップ（例: 配列の組み立て、GMMパラメータのオブジェクト化など）を`bridge.ts`に追加してください。
