use source_coding::{HuffmanCode, SymbolPr};
use std::collections::HashMap;

fn pr(alphabet: &[char], probs: &[f64]) -> SymbolPr {
    let mut map = HashMap::new();
    for (s, &p) in alphabet.iter().zip(probs.iter()) { map.insert(*s, p); }
    SymbolPr { alphabet: alphabet.to_vec(), s_to_prs: map }
}

#[test]
fn huffman_roundtrip() {
    let alphabet = ['A', 'B', 'C', 'D'];
    let probs = [0.1, 0.2, 0.3, 0.4];
    let h = HuffmanCode::from_symbol_pr(&pr(&alphabet, &probs));
    let msg: Vec<char> = "ABCDDCBA".chars().collect();
    let bits = h.encode(&msg);
    let dec = h.decode(msg.len(), &bits);
    assert_eq!(dec, msg);
}

#[test]
fn huffman_single_symbol() {
    let alphabet = ['X'];
    let probs = [1.0];
    let h = HuffmanCode::from_symbol_pr(&pr(&alphabet, &probs));
    let msg: Vec<char> = "XXXXXX".chars().collect();
    let bits = h.encode(&msg);
    let dec = h.decode(msg.len(), &bits);
    assert_eq!(dec, msg);
}
