use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Symbol = char;
pub type Alphabet = Vec<Symbol>;
pub type Symbols = Vec<Symbol>;
pub type CodeWords = Vec<u8>; // 0/1 bits

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPr {
    pub alphabet: Alphabet,
    pub s_to_prs: HashMap<Symbol, f64>,
}

#[derive(Debug, Clone)]
pub struct SymbolRange {
    pub alphabet: Alphabet,
    pub s_to_rs: HashMap<Symbol, (f64, f64)>,
}

#[derive(thiserror::Error, Debug)]
pub enum ArithmeticError {
    #[error("invalid probability distribution")]
    InvalidDistribution,
    #[error("empty input")]
    EmptyInput,
}

pub type Result<T> = std::result::Result<T, ArithmeticError>;

#[derive(Debug, Clone)]
pub struct ArithmeticCode {
    symbol_range: SymbolRange,
}

impl ArithmeticCode {
    pub fn new(symbol_pr: SymbolPr) -> Result<Self> {
        // validate probabilities sum to ~1 and non-negative
        let mut sum = 0.0f64;
        for &s in &symbol_pr.alphabet {
            let p = *symbol_pr.s_to_prs.get(&s).ok_or(ArithmeticError::InvalidDistribution)?;
            if !(p >= 0.0) { return Err(ArithmeticError::InvalidDistribution); }
            sum += p;
        }
        if (sum - 1.0).abs() > 1e-9 { return Err(ArithmeticError::InvalidDistribution); }
        Ok(Self { symbol_range: Self::spr_to_sr(symbol_pr) })
    }

    fn spr_to_sr(symbol_pr: SymbolPr) -> SymbolRange {
        let mut current_pr = 0.0f64;
        let mut s_to_rs = HashMap::new();
        for &symbol in &symbol_pr.alphabet {
            let p = symbol_pr.s_to_prs[&symbol];
            let first = current_pr;
            current_pr += p;
            let second = current_pr;
            s_to_rs.insert(symbol, (first, second));
        }
        SymbolRange { alphabet: symbol_pr.alphabet, s_to_rs }
    }

    fn calc_range(&self, symbols: &Symbols) -> (f64, f64) {
        let mut current = (0.0f64, 1.0f64);
        for &sym in symbols {
            let (l, r) = self.symbol_range.s_to_rs[&sym];
            let len = current.1 - current.0;
            let min = current.0;
            current.0 = min + l * len;
            current.1 = min + r * len;
        }
        current
    }

    fn in_range(&self, p: f64) -> Symbol {
        for &sym in &self.symbol_range.alphabet {
            let (l, r) = self.symbol_range.s_to_rs[&sym];
            if l <= p && p < r { return sym; }
        }
        // fallback
        self.symbol_range.alphabet[0]
    }

    pub fn encode(&self, symbols: &Symbols) -> Result<CodeWords> {
        if symbols.is_empty() { return Err(ArithmeticError::EmptyInput); }
        let range = self.calc_range(symbols);
        let mid = (range.0 + range.1) / 2.0;
        let range_len = (range.1 - range.0).max(f64::EPSILON);
        let length = ((-range_len.log2()).ceil() as i64 + 1).max(1) as usize;
        let mut cws = vec![0u8; length];
        let mut bin = (0.0f64, 1.0f64);
        for i in 0..length {
            let border = (bin.0 + bin.1) / 2.0;
            if mid < border {
                cws[i] = 0;
                bin.1 = border;
            } else {
                cws[i] = 1;
                bin.0 = border;
            }
        }
        Ok(cws)
    }

    pub fn decode(&self, length: usize, cws: &CodeWords) -> Result<Symbols> {
        if length == 0 { return Ok(vec![]); }
        let mut p = 0.0f64;
        let mut bin = 1.0f64;
        for &b in cws {
            bin /= 2.0;
            p += (b as f64) * bin;
        }
        let mut out = Vec::with_capacity(length);
        for _ in 0..length {
            let sym = self.in_range(p);
            let (l, r) = self.symbol_range.s_to_rs[&sym];
            out.push(sym);
            p = (p - l) / (r - l);
        }
        Ok(out)
    }
}
