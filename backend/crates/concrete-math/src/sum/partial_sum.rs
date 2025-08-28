use crate::sequence::core::ClosedForm;
use crate::sequence::recurrence_relation::RecurrenceRelation;

pub fn partial_sum(cf: &ClosedForm) -> ClosedForm {
    let a0 = cf.term(0).re;
    let rr = RecurrenceRelation::new(vec![1.0], cf.terms.clone(), vec![a0]);
    rr.solve()
}
