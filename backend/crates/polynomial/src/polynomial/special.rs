use linalg::Field;
use num_traits::FromPrimitive;

use crate::polynomial::Polynomial;

/// n次の第一種チェビシェフ多項式 T_n(x) を生成する。
pub fn chebyshev_first_kind<F: Field>(n: usize) -> Polynomial<F> {
    let p_0 = Polynomial::one(); // T₀(x) = 1
    let p_1 = Polynomial::new(vec![F::zero(), F::one()]); // T₁(x) = x

    if n == 0 {
        return p_0;
    }
    if n == 1 {
        return p_1;
    }

    let mut t_n_minus_2 = p_0;
    let mut t_n_minus_1 = p_1;
    let x_poly = Polynomial::new(vec![F::zero(), F::one()]); // x
    let two = F::one() + F::one();

    for _ in 2..=n {
        // Tₙ(x) = 2x * T_{n-1}(x) - T_{n-2}(x)
        let t_n = &(&(&x_poly * &t_n_minus_1) * two.clone()) - &t_n_minus_2;

        // 次の反復のために更新
        t_n_minus_2 = t_n_minus_1;
        t_n_minus_1 = t_n;
    }

    t_n_minus_1
}

/// n次の第二種チェビシェフ多項式 U_n(x) を生成する。
pub fn chebyshev_second_kind<F: Field>(n: usize) -> Polynomial<F> {
    let two = F::one() + F::one();
    let p_0 = Polynomial::one(); // U₀(x) = 1
    let p_1 = Polynomial::new(vec![F::zero(), two]); // U₁(x) = 2x
    if n == 0 {
        return p_0;
    }
    if n == 1 {
        return p_1;
    }

    let mut t_n_minus_2 = p_0;
    let mut t_n_minus_1 = p_1;
    let x_poly = Polynomial::new(vec![F::zero(), F::one()]); // x
    let two = F::one() + F::one();

    for _ in 2..=n {
        // Tₙ(x) = 2x * T_{n-1}(x) - T_{n-2}(x)
        let t_n = &(&(&x_poly * &t_n_minus_1) * two.clone()) - &t_n_minus_2;

        // 次の反復のために更新
        t_n_minus_2 = t_n_minus_1;
        t_n_minus_1 = t_n;
    }

    t_n_minus_1
}

/// n次のルジャンドル多項式 P_n(x) を生成する。
pub fn legendre<F: Field + FromPrimitive>(n: usize) -> Polynomial<F> {
    if n == 0 {
        return Polynomial::one();
    }
    if n == 1 {
        return Polynomial::new(vec![F::zero(), F::one()]);
    }

    let mut p_i_minus_2 = Polynomial::one(); // P₀(x)
    let mut p_i_minus_1 = Polynomial::new(vec![F::zero(), F::one()]); // P₁(x)
    let x_poly = p_i_minus_1.clone();

    // i を 1 から n-1 までループさせて P₂(x) から Pₙ(x) を順に計算
    for i in 1..n {
        // --- ★修正点：係数をループ内で毎回計算する ---
    let i_f = F::from_usize(i).unwrap_or_else(|| F::zero());
    let i_plus_1_f = F::from_usize(i + 1).unwrap_or_else(|| F::one());
    let two_i_plus_1_f = F::from_usize(2 * i + 1).unwrap_or_else(|| F::one());
        // ------------------------------------------

        // 漸化式: (i+1)P_{i+1} = (2i+1)xPᵢ - iP_{i-1}
        // 変形: P_{i+1} = ( (2i+1)xPᵢ - iP_{i-1} ) / (i+1)

        // (2i+1)xPᵢ の部分
        let term1 = &(&x_poly * &p_i_minus_1) * two_i_plus_1_f;

        // iP_{i-1} の部分
        let term2 = &p_i_minus_2 * i_f;

        // Pᵢ₊₁ を計算
        let p_i = &(&term1 - &term2) / i_plus_1_f;

        // 次の反復のために更新
        p_i_minus_2 = p_i_minus_1;
        p_i_minus_1 = p_i;
    }

    p_i_minus_1 // ループ終了後、これがPₙ(x)になっている
}

/// 物理学者のエルミート多項式 H_n(x) を生成する。
/// 漸化式: H_0 = 1, H_1 = 2x, H_n = 2x H_{n-1} - 2(n-1) H_{n-2}
pub fn hermite_physicists<F: Field + FromPrimitive>(n: usize) -> Polynomial<F> {
    let two = F::one() + F::one();
    let p0 = Polynomial::one();
    let p1 = Polynomial::new(vec![F::zero(), two.clone()]); // 2x
    if n == 0 {
        return p0;
    }
    if n == 1 {
        return p1;
    }

    let x_poly = Polynomial::new(vec![F::zero(), F::one()]);
    let mut h_nm2 = p0;
    let mut h_nm1 = p1;
    for k in 2..=n {
    let k_minus_1 = F::from_usize(k - 1).unwrap_or_else(|| F::zero());
        let coef = two.clone() * k_minus_1; // 2(n-1)
        let term1 = &(&x_poly * &h_nm1) * two.clone(); // 2x H_{n-1}
        let term2 = &h_nm2 * coef; // 2(n-1) H_{n-2}
        let h_n = &term1 - &term2;
        h_nm2 = h_nm1;
        h_nm1 = h_n;
    }
    h_nm1
}

/// ラゲール多項式 L_n(x) を生成する（α=0）。
/// 漸化式: L_0 = 1, L_1 = 1 - x,
/// n L_n = (2n-1 - x) L_{n-1} - (n-1) L_{n-2}
pub fn laguerre<F: Field + FromPrimitive>(n: usize) -> Polynomial<F> {
    let p0 = Polynomial::one();
    let p1 = Polynomial::new(vec![F::one(), -(F::one())]); // 1 - x
    if n == 0 {
        return p0;
    }
    if n == 1 {
        return p1;
    }

    let x_poly = Polynomial::new(vec![F::zero(), F::one()]);
    let mut l_nm2 = p0;
    let mut l_nm1 = p1;
    for k in 2..=n {
    let k_f = F::from_usize(k).unwrap_or_else(|| F::one());
    let two_k_minus_1 = F::from_usize(2 * k - 1).unwrap_or_else(|| F::one());
        // (2n-1 - x) L_{n-1} = (2n-1)L_{n-1} - x L_{n-1}
        let term_a = &l_nm1 * two_k_minus_1;
        let term_b = &x_poly * &l_nm1;
        let left = &term_a - &term_b;
    let right = &l_nm2 * F::from_usize(k - 1).unwrap_or_else(|| F::one());
        let num = &left - &right; // 分子
        let l_n = &num / k_f; // / n
        l_nm2 = l_nm1;
        l_nm1 = l_n;
    }
    l_nm1
}
