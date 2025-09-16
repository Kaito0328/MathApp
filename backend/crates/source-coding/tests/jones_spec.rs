use std::collections::HashMap;

use source_coding::{ArithmeticCode, JonesCode, SymbolPr};

#[test]
fn jones_basic_roundtrip() {
    let alphabet = vec!['A','B','C'];
    let mut s_to_prs = HashMap::new();
    s_to_prs.insert('A', 0.5);
    s_to_prs.insert('B', 0.3);
    s_to_prs.insert('C', 0.2);
    let pr = SymbolPr { alphabet: alphabet.clone(), s_to_prs };
    let j = JonesCode::from_symbol_pr(&pr, 1<<16);
    let seq: Vec<char> = "ABACAB".chars().collect();
    let bits = j.encode(&seq);
    let dec = j.decode(seq.len(), &bits);
    assert_eq!(dec, seq);
}
