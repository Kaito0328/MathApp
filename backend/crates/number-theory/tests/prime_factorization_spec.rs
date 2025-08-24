use number_theory::prime_factorization::factor;

#[test]
fn factor_small_primes() {
    assert_eq!(factor(2), vec![2]);
    assert_eq!(factor(3), vec![3]);
    assert_eq!(factor(5), vec![5]);
    assert_eq!(factor(7), vec![7]);
}

#[test]
fn factor_composites() {
    assert_eq!(factor(1), vec![]);
    assert_eq!(factor(4), vec![2, 2]);
    assert_eq!(factor(6), vec![2, 3]);
    assert_eq!(factor(12), vec![2, 2, 3]);
    assert_eq!(factor(60), vec![2, 2, 3, 5]);
    assert_eq!(factor(97 * 101), vec![97, 101]);
}

#[test]
fn factor_medium_semiprime() {
    let n = 1_000_003u64 * 1_000_033u64; // 中規模な素数の積
    let mut f = factor(n);
    f.sort();
    assert_eq!(f, vec![1_000_003u64, 1_000_033u64]);
}
