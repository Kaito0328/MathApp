use crate::discrete::TransferFunction as DiscreteTransferFunction;
use crate::statespace::DiscreteStateSpace;

/// 離散TF: インパルス応答（長さ len）
pub(crate) fn impulse_response_discrete_tf(tf: &DiscreteTransferFunction, len: usize) -> Vec<f64> {
    if len == 0 {
        return Vec::new();
    }
    let mut delta = vec![0.0; len];
    delta[0] = 1.0;
    tf.apply(&delta)
}

/// 離散TF: ステップ応答（長さ len）
pub(crate) fn step_response_discrete_tf(tf: &DiscreteTransferFunction, len: usize) -> Vec<f64> {
    if len == 0 {
        return Vec::new();
    }
    let step = vec![1.0; len];
    tf.apply(&step)
}

/// 離散状態空間: インパルス応答（長さ len）を直接シミュレーション
pub(crate) fn impulse_response_discrete_ss(ss: &DiscreteStateSpace, len: usize) -> Vec<f64> {
    simulate_discrete_ss(ss, len, /*is_step=*/ false)
}

/// 離散状態空間: ステップ応答（長さ len）を直接シミュレーション
pub(crate) fn step_response_discrete_ss(ss: &DiscreteStateSpace, len: usize) -> Vec<f64> {
    simulate_discrete_ss(ss, len, /*is_step=*/ true)
}

fn simulate_discrete_ss(ss: &DiscreteStateSpace, len: usize, is_step: bool) -> Vec<f64> {
    if len == 0 {
        return Vec::new();
    }
    let n = ss.a.rows;
    let m = ss.b.cols; // 入力数（ここでは1を想定）
    assert!(
        m == 1 && ss.c.rows == 1 && ss.d.rows == 1 && ss.d.cols == 1,
        "SISO only"
    );
    let mut x = vec![0.0_f64; n];
    let mut y = Vec::with_capacity(len);
    for k in 0..len {
        let u = if is_step || k == 0 { 1.0 } else { 0.0 };
        // y_k = C x + D u
        let mut yk = 0.0;
        for (j, xj) in x.iter().enumerate() {
            yk += ss.c[(0, j)] * xj;
        }
        yk += ss.d[(0, 0)] * u;
        y.push(yk);
        // x_{k+1} = A x_k + B u_k
        let mut x_next = vec![0.0; n];
        for (i, xi) in x_next.iter_mut().enumerate() {
            let mut sum = 0.0;
            for (j, xj) in x.iter().enumerate() {
                sum += ss.a[(i, j)] * xj;
            }
            sum += ss.b[(i, 0)] * u;
            *xi = sum;
        }
        x = x_next;
    }
    y
}
