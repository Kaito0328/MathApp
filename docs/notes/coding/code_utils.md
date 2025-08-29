# Coding utilities（coding/src/code_utils.rs）

## 背景知識
線形符号の基本操作は行列演算で定義できる。系統形 [I|P] に変換すれば検査行列 H=[P^T|I] が直ちに構成でき、シンドローム復号の基盤が整う。学習用の小規模実験では、重み・距離・最小距離復号・シンドローム復号が実装の中心となる。

## 入力例と出力例
例（概念）。
- 重み/距離: ベクトル/ベクトル対を入力し、ハミング重み/距離を整数で返す。
- 系統形化: 生成行列 G を入力し、列の並び替え情報と [I|P] 形の行列を返す。
- H構成: 系統形 G から H=[P^T|I] を返す。
- 最小距離復号: 受信語 r とコードブック C を入力し、距離最小の符号語 c を返す。
- GF(2)シンドローム復号: H と r を入力し、最尤な推定誤り e と推定符号語 c=r−e を返す。

## アルゴリズム
以下に各機能の定義と手順を述べる。

重みと距離.
ハミング重みはベクトルの非零成分の個数である。ハミング距離は二つの同長ベクトルにおいて成分が異なる位置の個数である。いずれも長さ n に対して O(n) で計算でき、GF(2) では重みと距離がビット演算（XOR→popcount）で効率良く求まる。
```text
# hamming_weight(v)
w = 0
for i in 0..dim(v)-1:
    if v[i] != 0: w += 1
return w

# hamming_distance(a,b)
assert dim(a) == dim(b)
d = 0
for i in 0..dim(a)-1:
    if a[i] != b[i]: d += 1
return d
```

最小距離と重み分布.
線形符号の最小距離はゼロ語を除いた符号語の重みの最小値で定義される。実装ではコードブックを走査し最小の重みを更新する。重み分布は各符号語の重みを数え上げ、0..n のビンに加算してヒストグラムを作る。いずれもコードブックの大きさに比例した計算量である。
```text
# linear_hamming_d_min(codebook)  # ゼロ語除外
n = length(codeword)
best = n
for c in codebook:
    if c == 0: continue
    w = hamming_weight(c)
    best = min(best, w)
return best

# weight_distribution(codebook)  # 0..n のヒストグラム
bins = [0]*(n+1)
for c in codebook:
    w = hamming_weight(c)
    bins[w] += 1
return bins
```

最小距離復号.
受信語 r とコードブック中の各符号語とのハミング距離を計算し、最も近いものを選ぶ。距離が同じ候補が複数ある場合は先に見つかったものを採用する（実装依存）。計算量は O(|C|·n) であり、大きなコードでは近似手法が必要になる。
```text
# md_decode(codebook, r)
best = +inf
arg = 0
for i, c in enumerate(codebook):
    d = hamming_distance(c, r)
    if d < best:
        best = d
        arg = i
return arg
```

GF(p) の全長 k ベクトル列挙とコードブック生成.
全長 k の GF(p) ベクトルは p^k 通り存在する。再帰や桁上がりの要領で全列挙し、各ベクトル u に対して行列積 u·G を計算して符号語を得る。p^k が急増するため、現実には小さな k に限られる。
```text
# generate_vectors_gfp<P>(k)
if k == 0: return [[]]
prev = generate_vectors_gfp<P>(k-1)
out = []
for v in prev:
    for x in 0..P-1:
        out.push(v + [x])
return out

# generate_codebook_gfp(G)
codebook = []
for u in generate_vectors_gfp<P>(k):
    c = (u * G).row(0)
    codebook.push(c)
return codebook
```

系統形化（to_systematic_g）.
列の入れ替えも許容しながらピボットを探索し、行基本変形で左 k 列を単位行列に整える。各列入替の履歴 perm を保持して返す。ランク不足のときは単位行列が得られず、エラーで打ち切る。
```text
# 入力: G (k x n)
A = G.copy()
perm = [0,1,..,n-1]
row = 0
for col in 0..n-1:
    if row == k: break
    # pivot 探索（列交換可）
    find (r,c) with r in [row..k-1], c in [col..n-1] s.t. A[r,c] != 0
    if not found: continue
    swap_columns(A, col, c); swap(perm[col], perm[c])
    swap_rows(A, row, r)
    # ピボットを1に、同列の他行を0に
    inv = 1 / A[row,col]
    A[row,*] *= inv
    for rr in 0..k-1:
        if rr == row: continue
        fac = A[rr,col]
        A[rr,*] -= fac * A[row,*]
    row += 1
return (A, perm)  # 期待形: [I_k | P]
```

H構成（formed_g_to_h）と G→H.
標準形 G=[I|P] に対して H=[P^T|I] を構成すると H·G^T=0 が成り立つ。一般の G からは、まず系統形化で G_sys=[I|P] を得て H_sys を作り、列順 perm に従って元の順序に戻す。
```text
# formed_g_to_h(G_sys=[I|P])
r = n - k
H = zeros(r, n)
for i in 0..r-1:
    for j in 0..k-1:
        H[i,j] = G_sys[j, k+i]   # 左ブロックに P^T
for i in 0..r-1:
    H[i, k+i] = 1               # 右ブロックに I_r
return H

# parity_check_from_generator(G)
G_sys, perm = to_systematic_g(G)
H_sys = formed_g_to_h(G_sys)
H = zeros(n-k, n)
for j in 0..n-1:
    orig = perm[j]
    H[*, orig] = H_sys[*, j]    # 列順を元の G に戻す
return H
```

GF(2)シンドローム計算と復号.
シンドローム s=H·r^T は受信語 r の誤りベクトル e に対し s=H·e^T に等しい。重み 1..t の誤り候補を列挙してシンドローム表を作り、s と一致する e を見つけたら c=r−e を返す。範囲内に該当がなければ復号失敗とする。
```text
# compute_syndrome_gf2(H, r)
s = zeros(rows(H))
for i in 0..rows(H)-1:
    acc = 0
    for j in 0..cols(H)-1:
        acc += H[i,j] * r[j]    # GF(2) での加算・乗算
    s[i] = acc
return s

# syndrome_decode_gf2(H, r, t)
s = compute_syndrome_gf2(H, r)
if s == 0: return r
Table = {}  # syndrome -> error (最小重み優先)
for w in 1..t:
    for each combination idxs (size w) of [0..n-1]:
        e = zeros(n); for j in idxs: e[j]=1
        s_e = H * e^T
        if s_e not in Table: Table[s_e] = e
if s in Table:
    return r - Table[s]
error("DecodeFailure")
```
