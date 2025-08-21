use crate::sequence::core::ClosedForm;
use crate::sequence::recurrence_relation::RecurrenceRelation;

/// S(n) = sum_{i=0}^n a_i の閉形式を返す。
/// a_n の閉形式 cf を受け取り、S_n = S_{n-1} + a_n, S_0 = a_0 の一次再帰として解く。
pub fn partial_sum(cf: &ClosedForm) -> ClosedForm {
    // a_n の線形再帰は未知だが、S_n は S_{n-1} + a_n で次数1。
    // ここでは RecurrenceRelation を使って S を解くために、
    // 非同次項に a_n（= cf）を入れ、係数は [1.0]（S_n - S_{n-1} = a_n）とする。
    // 初期値 S_0 = a_0
    let a0 = cf.term(0).re; // 実数部分のみ使用（現状の実装制約に合わせる）

    // cf.terms を GeneralTerm としてそのまま非同次項に渡すために clone
    let rr = RecurrenceRelation::new(vec![1.0], cf.terms.clone(), vec![a0]);
    rr.solve()
}
