use source_coding::{InternalCodeWords, Lz78Code};

#[test]
fn lz78_internal_roundtrip() {
    let input: Vec<char> = "ABAABABAABBBBBBA".chars().collect();
    let code = Lz78Code::encode_internal(&input);
    let dec = Lz78Code::decode_internal(&code);
    assert_eq!(dec, input);
}
