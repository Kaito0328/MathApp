use statistics::hypothesis::{
    chisq_gof, chisq_independence, correlation_t_test, f_test_variance_ratio, kruskal_wallis,
    mann_whitney_u, one_sample_t, one_way_anova, two_sample_t_pooled, two_sample_t_welch,
    wilcoxon_signed_rank, z_test_proportion, z_test_two_proportions, Tail,
};

fn main() {
    println!("== 一標本t検定 (母平均=0 を検定) ==");
    let xs = [2.0, 3.0, 4.0, 5.0, 6.0];
    println!("サンプル: {xs:?}");
    let t1 = one_sample_t(&xs, 0.0, Tail::TwoSided, Some(0.05)).unwrap();
    println!(
        "{}: t={:.3}, p={:.4}, df={:?}, CI={:?}",
        t1.method, t1.stat, t1.p_value, t1.df1, t1.ci
    );

    println!("\n== 二標本t検定（等分散仮定）: 群A > 群B を検定 ==");
    let a = [10.0, 11.0, 9.5, 10.5, 10.2];
    let b = [8.0, 7.8, 8.5, 8.2, 7.9];
    println!("群A: {a:?}");
    println!("群B: {b:?}");
    let t2 = two_sample_t_pooled(&a, &b, Tail::Greater, Some(0.05)).unwrap();
    println!(
        "{}: t={:.3}, p={:.4}, df={:?}, CI={:?}",
        t2.method, t2.stat, t2.p_value, t2.df1, t2.ci
    );

    println!("\n== Welchの二標本t検定（等分散を仮定しない）: 群A != 群B ==");
    let t2w = two_sample_t_welch(&a, &b, Tail::TwoSided, Some(0.05)).unwrap();
    println!(
        "{}: t={:.3}, p={:.4}, df={:?}, CI={:?}",
        t2w.method, t2w.stat, t2w.p_value, t2w.df1, t2w.ci
    );

    println!("\n== F検定（分散比）: Var(A) / Var(B) を検定 ==");
    let f = f_test_variance_ratio(&a, &b, Tail::TwoSided, Some(0.05)).unwrap();
    println!(
        "{}: F={:.3}, p={:.4}, df=({:?},{:?}), CI={:?}",
        f.method, f.stat, f.p_value, f.df1, f.df2, f.ci
    );

    println!("\n== 比率のZ検定（1標本）: p = 0.5 を検定 ==");
    let z = z_test_proportion(55, 100, 0.5, Tail::Greater, Some(0.05)).unwrap();
    println!("成功数=55, 試行数=100, 帰無 p0=0.5");
    println!(
        "{}: z={:.3}, p={:.4}, CI={:?}",
        z.method, z.stat, z.p_value, z.ci
    );

    println!("\n== 比率のZ検定（2標本）: p1 - p2 = 0 を検定 ==");
    let z2 = z_test_two_proportions(60, 100, 45, 100, Tail::TwoSided, Some(0.05)).unwrap();
    println!("(x1/n1)=(60/100), (x2/n2)=(45/100)");
    println!(
        "{}: z={:.3}, p={:.4}, CI={:?}",
        z2.method, z2.stat, z2.p_value, z2.ci
    );

    println!("\n== カイ二乗適合度検定 ==");
    let obs = [25.0, 30.0, 45.0];
    let exp = [33.3, 33.3, 33.3];
    println!("観測度数: {obs:?}");
    println!("期待度数: {exp:?}");
    let cg = chisq_gof(&obs, &exp, Tail::Greater).unwrap();
    println!(
        "{}: X^2={:.3}, p={:.4}, df={:?}",
        cg.method, cg.stat, cg.p_value, cg.df1
    );

    println!("\n== カイ二乗独立性検定（2x3表） ==");
    let table: [&[u64]; 2] = [&[10, 20, 30], &[20, 10, 20]];
    println!("表: {table:?}");
    let ci = chisq_independence(&table, Tail::Greater).unwrap();
    println!(
        "{}: X^2={:.3}, p={:.4}, df={:?}",
        ci.method, ci.stat, ci.p_value, ci.df1
    );

    {
        println!("\n== 一元配置分散分析(ANOVA) ==");
        let g1 = [2.0, 3.0, 2.5, 3.2];
        let g2 = [3.8, 4.1, 3.9, 4.0];
        let g3 = [5.2, 4.9, 5.1, 5.0];
        let an = one_way_anova(&[&g1, &g2, &g3], Tail::Greater).unwrap();
        println!(
            "{}: F={:.3}, p={:.4}, df=({:?},{:?})",
            an.method, an.stat, an.p_value, an.df1, an.df2
        );
    }
    {
        println!("\n== 相関の検定(t分布) ==");
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y = [0.9, 2.1, 2.9, 4.2, 5.1, 5.8];
        let ct = correlation_t_test(&x, &y, Tail::TwoSided, Some(0.05)).unwrap();
        println!(
            "{}: t={:.3}, p={:.4}, df={:?}, r~={:?}",
            ct.method, ct.stat, ct.p_value, ct.df1, ct.effect
        );
    }
    {
        println!("\n== ノンパラメトリック: Mann-Whitney U検定 ==");
        let x = [10.0, 11.0, 9.5, 10.5, 10.2];
        let y = [8.0, 7.8, 8.5, 8.2, 7.9];
        let mw = mann_whitney_u(&x, &y, Tail::TwoSided, true).unwrap();
        println!("{}: z={:.3}, p={:.4}", mw.method, mw.stat, mw.p_value);
    }
    {
        println!("\n== ノンパラメトリック: Wilcoxon符号付順位和検定(対応あり) ==");
        let before = [5.0, 6.0, 7.0, 6.5, 5.8, 6.2];
        let after = [5.5, 6.5, 7.2, 6.7, 6.1, 6.4];
        let wi = wilcoxon_signed_rank(&before, &after, Tail::TwoSided, true).unwrap();
        println!("{}: z={:.3}, p={:.4}", wi.method, wi.stat, wi.p_value);
    }
    {
        println!("\n== ノンパラメトリック: Kruskal–Wallis検定 ==");
        let g1 = [2.0, 3.0, 2.5, 3.2];
        let g2 = [3.8, 4.1, 3.9, 4.0];
        let g3 = [5.2, 4.9, 5.1, 5.0];
        let kw = kruskal_wallis(&[&g1, &g2, &g3], Tail::Greater).unwrap();
        println!(
            "{}: H={:.3}, p={:.4}, df={:?}",
            kw.method, kw.stat, kw.p_value, kw.df1
        );
    }
}
