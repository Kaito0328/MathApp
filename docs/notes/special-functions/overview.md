# special-functions クレート概要

## 背景知識
特殊関数は確率・統計・物理・工学で頻出する。本クレートはガンマ関数/正規誤差関数/ベータ関数とその正則化，逆関数近似などを提供する。

## 入力例と出力例
- 例: log_gamma，erf/erfc/erf_inv，beta/log_beta/regularized_beta。

## アルゴリズム
- Gamma 系: Lanczos 近似や級数/不完全ガンマ（上側/下側）の再帰で実装。
- Erf 系: 多項式/有理近似で erf/erfc，Acklam 近似で正規分布の分位点の補助を提供。
- Beta 系: ガンマ関数を介した定義と対数形，正則化ベータの連分数展開や級数近似。
