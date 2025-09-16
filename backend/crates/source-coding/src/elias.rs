use crate::CodeWords;

pub fn elias_gamma_encode(n: u64) -> CodeWords {
    assert!(n >= 1, "gamma code defined for n>=1");
    let mut bits = Vec::new();
    let mut val = n;
    let mut bin = Vec::new();
    while val > 0 { bin.push((val & 1) as u8); val >>= 1; }
    bin.reverse();
    let len = bin.len();
    // unary for length-1 zeros, then the binary
    bits.extend(std::iter::repeat(0u8).take(len-1));
    bits.extend_from_slice(&bin);
    bits
}

pub fn elias_gamma_decode(bits: &CodeWords, start: usize) -> Option<(u64, usize)> {
    let mut idx = start;
    let mut zeros = 0usize;
    while idx < bits.len() && bits[idx] == 0 { zeros += 1; idx += 1; }
    if idx >= bits.len() { return None; }
    // read zeros+1 bits including leading 1
    let mut val: u64 = 0;
    for _ in 0..(zeros+1) {
        if idx >= bits.len() { return None; }
        val = (val << 1) | bits[idx] as u64;
        idx += 1;
    }
    Some((val, idx))
}
