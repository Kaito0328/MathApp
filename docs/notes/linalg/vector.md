# ベクトル

## 背景知識
ベクトルは 1 次元配列として表現され，内積・ノルム・正規化・統計量などが定義される。本クレートの `Vector<T>` はスカラ抽象 `Scalar` を満たす型で使用できる。

## 入力例と出力例
- 入力: データ列，次元 dim。
- 出力: `Vector` の演算結果（スカラーやベクトル）。

## アルゴリズム
- 基本: zeros/ones、加減・スカラー演算・Hadamard・畳み込み conv（simple/FFT/auto）。
	- conv_simple: O(NM) の直接法。conv_fft: FFT/IFFT で O(N log N)。auto は閾値で切替える。
- 幾何/統計: norm/normalize、cosine_similarity、mean/std、argmax/min、max/min。
- Householder 反射ベクトル v の構成:
	- 与えられた x に対し α=−sign(x_1)·||x||、u=x−αe_1、v=u/||u||。
	- 反射 H=I−2vv^T を適用して先頭以降の成分を消去。数値安定性のため α の符号選択と縮尺に注意する。

### 計算量と制約
- いずれも O(n)。conv_fft は O(N log N)。スカラ型が実数のときは丸め誤差に注意。
