use coding::GFp;

fn main() {
    type F = GFp<5>;
    let a = F::new(7); // 2 mod 5
    let b = F::new(9); // 4 mod 5
    println!("a={a}, b={b}");
    println!("a+b={}, a-b={}, a*b={}, b/a={}", a + b, a - b, a * b, b / a);
}
