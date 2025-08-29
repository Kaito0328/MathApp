# MathApp

数理・情報通信分野の基礎から応用までをRustで実装し, 手を動かして学べる土台を提供するモノレポです. バックエンドはRustのワークスペースとして各分野をクレートで分割し, 将来的にフロントエンドと接続して可視化や操作性を高めることを想定しています.

## 全体像

- backend: Rustワークスペース. 分野ごとに独立したクレートを配置.
- frontend: 将来追加予定のWeb UI. TypeScriptベースを想定.

## クレート一覧と概要

以下は`backend/crates/`配下の主なクレートの概要です. 実装の詳細は各クレートの`src/`と`examples/`, `tests/`を参照してください.

- coding: 線形符号の実装. ハミング符号, BCH符号, リードソロモン符号など, 生成行列, 検査行列, シンドローム計算, 復号アルゴリズムを含みます.
- finite-field: 有限体の実装. GFp, GF256, 拡大体GF(p^m)など, 符号や多項式演算の土台となる演算を提供します.
- polynomial: 多項式のコア演算. 加減乗除, GCD, 評価, 補間などを汎用型で扱います.
- linalg: 線形代数. ベクトル, 行列の基本演算, LU/QR, 固有値分解, 連立方程式解法などを実装します.
- fft-core: DFT/FFTのコア実装. 周波数解析や畳み込みの基盤を提供します.
- convolution: 畳み込み/相関の実装. FFTベースや直接法を含みます.
- signal_processing: 信号処理. ウィンドウ関数, サンプリング, FIR/IIR, DFT, 画像処理ユーティリティなど.
- lti-systems: 線形時不変システム. 伝達関数, 応答解析, 安定性評価など.
- statistics: 基礎統計. 離散/連続分布, 記述統計, 検定など.
- statsmodels: 統計モデル. ベイズ推定, 最尤推定, カルマンフィルタなど.
- number-theory: 整数論. 素数判定, 拡張ユークリッド, 中国剰余定理など.
- special-functions: 特殊関数. ガンマ関数など, 数値解析で利用する関数群.
- concrete-math: 離散数学と初等的な和の計算など. 組合せ, 和の公式, 離散和等.
- utils: 共有ユーティリティ.

## ディレクトリ構成

```
MathApp/
├── README.md
├── backend/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── crates/
│       ├── coding/
│       ├── concrete-math/
│       ├── convolution/
│       ├── fft-core/
│       ├── finite-field/
│       ├── linalg/
│       ├── lti-systems/
│       ├── number-theory/
│       ├── polynomial/
│       ├── signal_processing/
│       ├── special-functions/
│       ├── statistics/
│       ├── statsmodels/
│       └── utils/
└── frontend/                 # 将来追加予定
```

## 使い方

ワークスペース全体のビルドとテスト.

```bash
cd backend
cargo build
cargo test
```

特定クレートのテストだけを実行.

```bash
cargo test -p linalg
cargo test -p coding
```

examplesの実行例. 例えば符号理論クレートのRS符号サンプルを実行します.

```bash
cargo run -p coding --example rs_examples
cargo run -p coding --example bch_examples
```

ベンチマークや可視化は今後追加予定です.

## 実装の方針と設計メモ

- モジュール分割と再エクスポート. 各クレートは`pub use`や`prelude`モジュールで外部APIを整理し, 学習用途でも使いやすいインターフェースを目指します.
- エラー型の一貫性. `error`モジュールを各クレートに設け, `Result`型エイリアスを提供します.
- 数値安定性と正確さ. 線形代数や信号処理では, ピボット選択, 直交化, 反復改良などの定石を取り入れます.
- ジェネリック設計. 体や環のような代数的制約をトレイトで表現し, 数値型を差し替え可能にします.

## 今後のドキュメント計画

各実装ファイルごとにMarkdownノートを用意し, 概要と具体的なアルゴリズムを簡潔にまとめます. 読点は","を, 句点は"."を用います. 粒度は次の想定です.

- 例: `linalg/src/eig.rs` → `docs/notes/linalg/eig.md`.
- 例: `coding/src/rs.rs` → `docs/notes/coding/reed_solomon.md`.
- 四則演算など基本機能がまとまった巨大ファイルは要相談で章立てします.

テンプレート案.

1. 概要: 何をするモジュールか, 使用場面.
2. 背景: 数式と基本概念の要点.
3. 実装要旨: データ構造, アルゴリズムの要点, 複雑度と数値安定性.
4. 使い方: 主要APIの短いサンプル.
5. 参考: 書籍や論文, Web資料.

## 貢献

IssueやPRを歓迎します. 命名とAPIは一貫性を重視し, 既存の`prelude`, `error`設計に沿ってください.

## ライセンス

後日追記予定です.