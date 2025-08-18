use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::One;
use utils::prime_factorization::factor_for_big;

fn product(nums: &[BigInt]) -> BigInt {
    nums.iter().fold(BigInt::one(), |acc, x| acc * x)
}

#[test]
fn qs_factors_small_semiprime() {
    // 101 * 103 = 10403
    let p = BigInt::from(101u32);
    let q = BigInt::from(103u32);
    let n = &p * &q;
    let fs = factor_for_big(&n);
    assert!(!fs.is_empty(), "no factors found");
    let prod = product(&fs);
    assert_eq!(prod, n, "factors do not multiply back to n");
    for f in fs {
        assert!(f > BigInt::from(1u32) && f < n);
    }
}

#[test]
fn qs_factors_mid_semiprime() {
    // 1009 * 1013 = 1022117
    let p = BigInt::from(1009u32);
    let q = BigInt::from(1013u32);
    let n = &p * &q;
    let fs = factor_for_big(&n);
    assert!(!fs.is_empty(), "no factors found");
    let prod = product(&fs);
    assert_eq!(prod, n, "factors do not multiply back to n");
    for f in fs {
        assert!(f > BigInt::from(1u32) && f < n);
    }
}

#[test]
fn qs_factors_practical_minimum_semiprime() {
    // 平方篩法が有効になり始める、約80ビット（25桁）の数
    // p, q はそれぞれ40ビット程度の大きさの素数

    // p = 1,000,000,000,039 (40-bit prime)
    let p = BigInt::from_str("1000000000039").unwrap();

    // q = 3,000,000,000,017 (42-bit prime)
    let q = BigInt::from_str("3000000000017").unwrap();

    // n = p * q = 3,000,000,000,134,000,000,000,063 (約82ビット)
    let n = &p * &q;

    // factor_for_big は、内部でQSやRho等で完全因数分解を返す。
    let fs = factor_for_big(&n);
    assert!(!fs.is_empty(), "no factors found for {n}");
    // すべての因数の積が n に一致することを確認
    let prod: BigInt = fs.iter().product();
    assert_eq!(prod, n, "factors do not multiply back to n");
    // 全因数が非自明（>1）であること
    for f in &fs {
        assert!(f > &BigInt::from(1u32) && f < &n);
    }
}

#[test]
fn qs_factors_larger_semiprime() {
    // 65537 * 10007 = 655_828_759 (約 30 ビット)
    let p = BigInt::from(655371283u64);
    let q = BigInt::from(10007123340194019u64);
    let n = &p * &q;
    let fs = factor_for_big(&n);
    assert!(!fs.is_empty(), "no factors found");
    let prod = product(&fs);
    assert_eq!(prod, n, "factors do not multiply back to n");
    for f in fs {
        assert!(f > BigInt::from(1u32) && f < n);
    }
}
