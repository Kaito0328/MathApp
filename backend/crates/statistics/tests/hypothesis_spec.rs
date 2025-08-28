use statistics::hypothesis::{
    chisq_gof, chisq_independence, correlation_t_test, f_test_variance_ratio, kruskal_wallis,
    mann_whitney_u, one_sample_t, one_way_anova, two_sample_t_pooled, two_sample_t_welch,
    wilcoxon_signed_rank, z_test_proportion, z_test_two_proportions, Tail,
};

#[test]
fn one_sample_t_smoke() {
    let xs = [2.0, 3.0, 4.0, 5.0, 6.0];
    let r = one_sample_t(&xs, 0.0, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(r.p_value < 0.05);
    assert!(r.ci.is_some());
}

#[test]
fn welch_t_smoke() {
    let x = [10.0, 11.0, 9.5, 10.5, 10.2];
    let y = [8.0, 7.8, 8.5, 8.2, 7.9];
    let r = two_sample_t_welch(&x, &y, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(r.p_value < 0.05);
}

#[test]
fn pooled_t_basic() {
    let x = [10.0, 11.0, 9.5, 10.5, 10.2];
    let y = [8.0, 7.8, 8.5, 8.2, 7.9];
    let r = two_sample_t_pooled(&x, &y, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(r.p_value < 0.05);
    assert!(r.df1.is_some());
}

#[test]
fn anova_one_way_significant() {
    let g1 = [2.0, 3.0, 2.5, 3.2];
    let g2 = [3.8, 4.1, 3.9, 4.0];
    let g3 = [5.2, 4.9, 5.1, 5.0];
    let r = one_way_anova(&[&g1, &g2, &g3], Tail::Greater).unwrap();
    assert!(r.p_value < 0.01);
    assert_eq!(
        r.df2.unwrap() as usize,
        (g1.len() + g2.len() + g3.len() - 3)
    );
}

#[test]
fn correlation_t_strong_positive() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let y = [0.9, 2.1, 2.9, 4.2, 5.1, 5.8];
    let r = correlation_t_test(&x, &y, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(r.p_value < 0.01);
    assert!(r.effect.unwrap() > 0.9);
}

#[test]
fn mann_whitney_detects_shift() {
    let x = [10.0, 11.0, 9.5, 10.5, 10.2];
    let y = [8.0, 7.8, 8.5, 8.2, 7.9];
    let r = mann_whitney_u(&x, &y, Tail::TwoSided, true).unwrap();
    assert!(r.p_value < 0.05);
}

#[test]
fn wilcoxon_signed_rank_detects_change() {
    let before = [5.0, 6.0, 7.0, 6.5, 5.8, 6.2];
    let after = [5.5, 6.5, 7.2, 6.7, 6.1, 6.4];
    let r = wilcoxon_signed_rank(&before, &after, Tail::TwoSided, true).unwrap();
    assert!(r.p_value < 0.05);
}

#[test]
fn kruskal_wallis_significant() {
    let g1 = [2.0, 3.0, 2.5, 3.2];
    let g2 = [3.8, 4.1, 3.9, 4.0];
    let g3 = [5.2, 4.9, 5.1, 5.0];
    let r = kruskal_wallis(&[&g1, &g2, &g3], Tail::Greater).unwrap();
    assert!(r.p_value < 0.05);
}

#[test]
fn chisq_tests_smoke() {
    let obs = [25.0, 30.0, 45.0];
    let exp = [33.3, 33.3, 33.3];
    let gof = chisq_gof(&obs, &exp, Tail::Greater).unwrap();
    assert!(gof.p_value < 0.05);

    let table: [&[u64]; 2] = [&[10, 20, 30], &[20, 10, 20]];
    let ind = chisq_independence(&table, Tail::Greater).unwrap();
    assert!(ind.p_value < 0.05);
}

#[test]
fn z_tests_smoke() {
    let one = z_test_proportion(65, 100, 0.5, Tail::Greater, Some(0.05)).unwrap();
    assert!(one.p_value < 0.01);

    let two = z_test_two_proportions(60, 100, 45, 100, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(two.p_value < 0.05);
}

#[test]
fn f_test_basic_props() {
    let x = [-10.0, -9.0, 0.0, 9.0, 10.0];
    let y = [-0.2, -0.1, 0.0, 0.1, 0.2];
    let r = f_test_variance_ratio(&x, &y, Tail::Greater, Some(0.05)).unwrap();
    assert!(r.stat.is_finite());
    assert!(r.df1.unwrap() > 0.0);
    assert!(r.df2.unwrap() > 0.0);
}

#[test]
fn error_cases() {
    // correlation: length mismatch
    assert!(correlation_t_test(&[1.0, 2.0], &[1.0], Tail::TwoSided, None).is_err());
    // anova: too small group
    assert!(one_way_anova(&[&[1.0], &[1.0, 2.0]], Tail::Greater).is_err());
    // wilcoxon: all diffs zero -> empty after filtering
    assert!(wilcoxon_signed_rank(&[1.0, 2.0], &[1.0, 2.0], Tail::TwoSided, true).is_err());
}

#[test]
fn tails_behavior_ttest() {
    let xs = [2.0, 3.0, 4.0, 5.0, 6.0];
    let less = one_sample_t(&xs, 0.0, Tail::Less, Some(0.05)).unwrap();
    let greater = one_sample_t(&xs, 0.0, Tail::Greater, Some(0.05)).unwrap();
    let two = one_sample_t(&xs, 0.0, Tail::TwoSided, Some(0.05)).unwrap();
    // two-sided p should be >= each one-sided p
    assert!(two.p_value >= less.p_value.min(greater.p_value));
    // one-sided p's should sum to ~1
    assert!(((less.p_value + greater.p_value) - 1.0).abs() < 1e-12);
}

#[test]
fn ci_presence_absence() {
    let xs = [2.0, 3.0, 4.0, 5.0, 6.0];
    let with = one_sample_t(&xs, 0.0, Tail::TwoSided, Some(0.05)).unwrap();
    let without = one_sample_t(&xs, 0.0, Tail::TwoSided, None).unwrap();
    assert!(with.ci.is_some());
    assert!(without.ci.is_none());
}

#[test]
fn z_test_invalids() {
    assert!(z_test_proportion(0, 0, 0.5, Tail::TwoSided, None).is_err());
    assert!(z_test_proportion(5, 3, 0.5, Tail::TwoSided, None).is_err());
    assert!(z_test_proportion(1, 10, 1.5, Tail::TwoSided, None).is_err());
}

#[test]
fn chisq_gof_invalid_expected() {
    let obs = [10.0, 20.0];
    let exp = [10.0, 0.0];
    assert!(chisq_gof(&obs, &exp, Tail::Greater).is_err());
}

#[test]
fn ties_handling_nonparametric() {
    // mann-whitney with many ties
    let x = [1.0, 1.0, 2.0, 2.0, 3.0];
    let y = [1.0, 2.0, 2.0, 3.0, 3.0];
    let r = mann_whitney_u(&x, &y, Tail::TwoSided, true).unwrap();
    assert!(r.p_value >= 0.0 && r.p_value <= 1.0);

    // wilcoxon with ties in |diff|
    let a = [1.0, 2.0, 3.0, 4.0];
    let b = [1.5, 2.5, 2.5, 3.5]; // diffs: -0.5,-0.5,0.5,0.5 -> ties in abs
    let w = wilcoxon_signed_rank(&a, &b, Tail::TwoSided, true).unwrap();
    assert!(w.p_value >= 0.0 && w.p_value <= 1.0);
}

#[test]
fn correlation_negative() {
    let x: Vec<f64> = (1..=20).map(|i| i as f64).collect();
    // strong negative correlation with mild noise
    let y: Vec<f64> = (1..=20)
        .map(|i| -(i as f64) + if i % 2 == 0 { -0.1 } else { 0.1 })
        .collect();
    let r = correlation_t_test(&x, &y, Tail::TwoSided, Some(0.05)).unwrap();
    assert!(r.effect.unwrap() < -0.9);
    assert!(r.p_value >= 0.0 && r.p_value <= 1.0);
}

#[test]
fn anova_nonsignificant_when_equal_means() {
    let g1 = [1.0, 2.0, 3.0, 4.0];
    let g2 = [1.0, 2.0, 3.0, 4.0];
    let g3 = [1.0, 2.0, 3.0, 4.0];
    let r = one_way_anova(&[&g1, &g2, &g3], Tail::Greater).unwrap();
    assert!(r.p_value > 0.2);
}

#[test]
fn f_test_ci_sanity() {
    let x = [5.0, 7.0, 9.0, 6.0, 8.0, 7.5];
    let y = [5.1, 5.2, 4.9, 5.3, 5.0, 5.1];
    let r = f_test_variance_ratio(&x, &y, Tail::Greater, Some(0.05)).unwrap();
    if let Some((lo, hi)) = r.ci {
        assert!(lo.is_finite() && hi.is_finite());
        assert!(lo >= 0.0);
        assert!(lo <= hi);
    }
}
