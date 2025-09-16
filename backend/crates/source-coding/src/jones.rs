use std::collections::HashMap;

use crate::{Alphabet, CodeWords, Symbol, SymbolPr, Symbols};

#[derive(Debug, Clone)]
pub struct JonesCode {
    pub alphabet: Alphabet,
    pub cum: HashMap<Symbol, (u64, u64)>, // cumulative integer range [l, r)
    pub total: u64,
}

impl JonesCode {
    // Build integerized cumulative ranges using denominator `total` (power of two suggested)
    pub fn from_symbol_pr(pr: &SymbolPr, total: u64) -> Self {
        let mut cum = HashMap::new();
        let mut acc = 0u64;
        for &s in &pr.alphabet {
            let p = pr.s_to_prs.get(&s).cloned().unwrap_or(0.0);
            let w = (p * total as f64).round() as u64;
            let l = acc;
            let r = (acc + w).min(total);
            cum.insert(s, (l, r));
            acc = r;
        }
        Self { alphabet: pr.alphabet.clone(), cum, total }
    }

    pub fn encode(&self, symbols: &Symbols) -> CodeWords {
        // Interval refinement over integers, output binary of midpoint
        let mut l = 0u64;
        let mut r = self.total;
        for &s in symbols {
            let (ls, rs) = self.cum.get(&s).copied().unwrap_or((0, 1));
            let len = r - l;
            let nl = l + (ls * len) / self.total;
            let nr = l + (rs * len) / self.total;
            l = nl; r = nr;
        }
        let mid = (l + r) / 2;
        let mut out = Vec::new();
        let mut k = 0u64;
        while (1u64 << k) < self.total { k += 1; }
        for i in (0..k).rev() { out.push(((mid >> i) & 1) as u8); }
        out
    }

    pub fn decode(&self, length: usize, bits: &CodeWords) -> Symbols {
        let mut p = 0u64;
        for &b in bits { p = (p << 1) | (b as u64); }
        let mut out = Vec::with_capacity(length);
        let mut l = 0u64;
        let mut r = self.total;
        for _ in 0..length {
            // find symbol such that p in [map(l,r,ls), map(l,r,rs))
            let mut found = self.alphabet[0];
            for &s in &self.alphabet {
                let (ls, rs) = self.cum[&s];
                let len = r - l;
                let tl = l + (ls * len) / self.total;
                let tr = l + (rs * len) / self.total;
                if tl <= p && p < tr { found = s; break; }
            }
            let (ls, rs) = self.cum[&found];
            let len = r - l;
            let tl = l + (ls * len) / self.total;
            let tr = l + (rs * len) / self.total;
            out.push(found);
            l = tl; r = tr;
        }
        out
    }
}
