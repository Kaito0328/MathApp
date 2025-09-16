use source_coding::{ArithmeticCode, SymbolPr};
use std::collections::HashMap;

fn pr(alphabet: &[char], probs: &[f64]) -> SymbolPr {
    let mut map = HashMap::new();
    for (s, &p) in alphabet.iter().zip(probs.iter()) { map.insert(*s, p); }
    SymbolPr { alphabet: alphabet.to_vec(), s_to_prs: map }
}

#[test]
fn roundtrip_simple_binary() {
    let alphabet = ['A', 'B'];
    let probs = [0.25, 0.75];
    let ac = ArithmeticCode::new(pr(&alphabet, &probs)).expect("new");
    let msg: Vec<char> = "ABBAAAB".chars().collect();
    let code = ac.encode(&msg).expect("encode");
    let dec = ac.decode(msg.len(), &code).expect("decode");
    assert_eq!(dec, msg);
}

#[test]
fn roundtrip_ternary() {
    let alphabet = ['x', 'y', 'z'];
    let probs = [0.2, 0.5, 0.3];
    let ac = ArithmeticCode::new(pr(&alphabet, &probs)).expect("new");
    let msg: Vec<char> = "xyzzy".chars().collect();
    let code = ac.encode(&msg).expect("encode");
    let dec = ac.decode(msg.len(), &code).expect("decode");
    assert_eq!(dec, msg);
}

#[test]
fn empty_decode_zero_len() {
    let alphabet = ['0', '1'];
    let probs = [0.5, 0.5];
    let ac = ArithmeticCode::new(pr(&alphabet, &probs)).expect("new");
    let dec = ac.decode(0, &vec![]).expect("decode empty");
    assert_eq!(dec.len(), 0);
}

#[test]
fn invalid_distribution_rejected() {
    let alphabet = ['a', 'b'];
    let probs = [0.6, 0.5]; // sums to 1.1
    assert!(ArithmeticCode::new(pr(&alphabet, &probs)).is_err());
}
