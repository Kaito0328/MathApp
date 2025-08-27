use statistics::distribution::continuous::{exponential::Exponential, normal::Normal};
use statistics::distribution::discrete::poisson::Poisson;
use statistics::plot::{svg_continuous_pdf, svg_continuous_pdf_with, svg_discrete_pmf, SvgOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Normal(0,1)
    let n = Normal::new(0.0, 1.0)?;
    let svg_n = svg_continuous_pdf(&n, 480, 240, 400);
    std::fs::create_dir_all("statistics/plot")?;
    std::fs::write("statistics/plot/normal_pdf.svg", svg_n)?;
    println!("saved: statistics/plot/normal_pdf.svg");

    // Exponential(lambda=1)
    let e = Exponential::new(1.0)?;
    // Showcase options: custom bg and x-range
    let opts = SvgOptions {
        bg: "#ffffff",
        x_range: Some((0.0, 6.0)),
        samples: 300,
        ..Default::default()
    };
    let svg_e = svg_continuous_pdf_with(&e, 480, 240, &opts);
    std::fs::write("statistics/plot/exponential_pdf.svg", svg_e)?;
    println!("saved: statistics/plot/exponential_pdf.svg");

    // Poisson(lambda=3)
    let p = Poisson::new(3.0)?;
    let svg_p = svg_discrete_pmf(&p, 480, 240);
    std::fs::write("statistics/plot/poisson_pmf.svg", svg_p)?;
    println!("saved: statistics/plot/poisson_pmf.svg");

    Ok(())
}
