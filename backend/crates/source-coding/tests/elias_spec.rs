use source_coding::{elias_gamma_decode, elias_gamma_encode};

#[test]
fn elias_gamma_basic() {
    let nums = [1u64, 2, 3, 4, 5, 8, 13, 21];
    let mut bits_all = Vec::new();
    for &n in &nums { bits_all.extend(elias_gamma_encode(n)); }
    let mut idx = 0usize;
    let mut got = Vec::new();
    while idx < bits_all.len() {
        let (n, next) = elias_gamma_decode(&bits_all, idx).expect("dec");
        got.push(n);
        idx = next;
    }
    assert_eq!(got, nums);
}
