use std::collections::HashMap;

use source_coding::{BlockHuffmanTree, BlockSymbolsPr};

fn block(sym1: &str, sym2: &str) -> Vec<char> { sym1.chars().chain(sym2.chars()).collect() }

#[test]
fn huffman_block_roundtrip() {
    // Build a block alphabet of 2-letter blocks over {A,B}
    let aa: Vec<char> = vec!['A','A'];
    let ab: Vec<char> = vec!['A','B'];
    let ba: Vec<char> = vec!['B','A'];
    let bb: Vec<char> = vec!['B','B'];
    let block_alphabet = vec![aa.clone(), ab.clone(), ba.clone(), bb.clone()];
    let mut ss_to_prs = HashMap::new();
    ss_to_prs.insert(aa.clone(), 0.4);
    ss_to_prs.insert(ab.clone(), 0.3);
    ss_to_prs.insert(ba.clone(), 0.2);
    ss_to_prs.insert(bb.clone(), 0.1);
    let pr = BlockSymbolsPr { block_alphabet: block_alphabet.clone(), ss_to_prs };
    let tree = BlockHuffmanTree::new(3, pr); // ternary

    let seq = vec![aa, ab, ba, bb];
    let digits = tree.encode(&seq);
    let dec = tree.decode(4, &digits);
    assert_eq!(dec, seq);
}
