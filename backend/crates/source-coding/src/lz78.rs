use std::collections::HashMap;

use crate::{CodeWords, Symbols};

pub type InternalCodeWord = (usize, char); // (index, next_char)
pub type InternalCodeWords = Vec<InternalCodeWord>;

#[derive(Debug, Clone)]
pub struct Lz78Code;

impl Lz78Code {
    pub fn encode_internal(input: &Symbols) -> InternalCodeWords {
        let mut dict: HashMap<String, usize> = HashMap::new();
        dict.insert(String::new(), 0);
        let mut next_index = 1usize;
        let mut w = String::new();
        let mut out: InternalCodeWords = Vec::new();
        for &c in input {
            let mut wc = w.clone(); wc.push(c);
            if dict.contains_key(&wc) {
                w = wc;
            } else {
                let idx = *dict.get(&w).unwrap();
                out.push((idx, c));
                dict.insert(wc, next_index);
                next_index += 1;
                w.clear();
            }
        }
        if !w.is_empty() { out.push((*dict.get(&w).unwrap(), '\0')); }
        out
    }

    pub fn decode_internal(code: &InternalCodeWords) -> Symbols {
        let mut dict: Vec<String> = vec![String::new()];
        let mut out = String::new();
        for &(idx, c) in code {
            let mut s = dict[idx].clone();
            if c != '\0' { s.push(c); }
            out.push_str(&s);
            dict.push(s);
        }
        out.chars().collect()
    }

    // simple bit packing: prefix each pair with fixed-width for index inferred from growth
    pub fn encode(&self, input: &Symbols) -> CodeWords {
        let internal = Self::encode_internal(input);
        if internal.is_empty() { return vec![]; }
        let max_index = internal.len() + 1; // approximate
        let mut bits = 0usize; let mut tmp = max_index; while tmp > 0 { bits += 1; tmp >>= 1; }
        let mut out: CodeWords = Vec::new();
        for &(idx, c) in &internal {
            for i in (0..bits).rev() { out.push(((idx >> i) & 1) as u8); }
            let b = c as u32 as u8; // ASCII assumed
            for i in (0..8).rev() { out.push(((b >> i) & 1) as u8); }
        }
        out
    }

    pub fn decode(&self, _length: usize, bits: &CodeWords) -> Symbols {
        // Not robust without explicit header; provide simple heuristic assuming 8-bit chars and remaining bits for index
        let n = bits.len();
        if n == 0 { return vec![]; }
        // assume index width is 8 for simplicity
        let idx_bits = 8usize;
        let pair_bits = idx_bits + 8;
        let mut code: InternalCodeWords = Vec::new();
        let mut i = 0;
        while i + pair_bits <= n {
            let mut idx = 0usize;
            for _ in 0..idx_bits { idx = (idx << 1) | (bits[i] as usize); i += 1; }
            let mut b = 0u8;
            for _ in 0..8 { b = (b << 1) | bits[i]; i += 1; }
            code.push((idx, b as char));
        }
        Self::decode_internal(&code)
    }
}
