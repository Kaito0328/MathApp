# 行列（数値分解と派生機能）索引

## 背景知識
数値線形代数では安定・効率的な分解（QR, SVD, 固有分解）や派生演算（擬似逆，行列指数）が重要である。本章は分解ごとにファイルを分割した。

## 章立て
- QR 分解: `matrix_qr.md`
- 特異値分解（SVD）: `matrix_svd.md`
- 固有分解（対称/一般，Schur）: `matrix_eigendecomp.md`
- 擬似逆（Moore–Penrose）: `matrix_pinv.md`
- 行列指数（expm，Padé + scaling & squaring）: `matrix_expm.md`

各章は である調・三部構成（背景知識／入力例と出力例／アルゴリズム）で，実装可能な手順・安定化・計算量・境界条件を記述している。
