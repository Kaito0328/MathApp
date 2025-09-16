use std::collections::HashMap;

use source_coding::{Markov};

#[test]
fn markov_block_pr_sums_to_one_for_length_1() {
    let alphabet = vec!['A','B'];
    let mut init = HashMap::new();
    init.insert('A', 0.6);
    init.insert('B', 0.4);
    let mut cond = HashMap::new();
    cond.insert(('A','A'), 0.7);
    cond.insert(('A','B'), 0.3);
    cond.insert(('B','A'), 0.2);
    cond.insert(('B','B'), 0.8);
    let mk = Markov::new(alphabet.clone(), init, cond);
    let blocks = mk.enumerate_block_pr(1);
    let sum: f64 = blocks.iter().map(|(_,p)| *p).sum();
    assert!((sum - 1.0).abs() < 1e-9);
}
