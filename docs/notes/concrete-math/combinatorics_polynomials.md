# 組合せ多項式（falling/rising, shift, binom(x+k,k)）

## 背景知識
落ちる階乗 (x)_m と上がる階乗 x^{\overline{m}} は多項式基底として便利である。シフト p(x+h) はテイラー展開により係数の線形変換で表せる。

## 入力例と出力例
- 入力: m, または多項式 p とシフト量 h。
- 出力: 多項式の係数配列。

## アルゴリズム
- falling_factorial_poly(m):
	- (x)_0=1。for t=1..m: (x)_t=(x−t+1)·(x)_{t−1} を多項式乗算で更新。
	- 計算量: 朴素多項式乗算で O(m^2)。FFT 乗算を用いれば準線形化可能。
- rising_factorial_poly(m):
	- x^{\overline{0}}=1。for t=1..m: x^{\overline{t}}=(x+t−1)·x^{\overline{t−1}}。
	- 対応する係数の関係から falling と相互に変換可能である。
- shift_poly_x_plus_h(p,h):
	- p(x)=∑ a_i x^i → p(x+h)=∑ a_i ∑_{j=0..i} C(i,j) h^{i−j} x^j。
	- 実装: 各次数 i で内側ループ j を回し、出力係数 b_j に加算する（数値は f64/Complex）。
	- 計算量: O(n^2)。Newton 形式や分割統治で高速化余地あり。
- binom_x_plus_k_choose_k_poly(k):
	- 恒等式 C(x+k,k)=∑_{j=0..k} C(k,j) C(x,j)。右辺の基底 C(x,j) を falling 基底で構成し合成する。
	- 直接再帰: C(x+k,k)=((x+k)/k)·C(x+k−1,k−1) から前進に係数を更新する実装も可能。

境界条件と注意:
- m=0 で恒等的に 1。h=0 のとき shift は恒等写像である。
