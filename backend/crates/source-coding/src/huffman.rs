use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::{CodeWords, Symbol, SymbolPr, Symbols};

#[derive(Debug, Clone)]
pub struct HuffmanCode {
    pub codes: HashMap<Symbol, Vec<u8>>, // canonical codes as bit vectors
    pub decode_table: HashMap<(usize, u64), Symbol>, // (length, prefix)->symbol for fast decode
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    prob: f64,
    // leaf
    sym: Option<Symbol>,
    // internal
    left: Option<Box<Node>>,  // 0
    right: Option<Box<Node>>, // 1
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is max-heap; we want min-heap by prob
        other
            .prob
            .partial_cmp(&self.prob)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                // deterministic tie-breaker on leaf presence
                let a = self.sym.is_some() as i32;
                let b = other.sym.is_some() as i32;
                a.cmp(&b)
            })
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Eq for Node {}

impl HuffmanCode {
    pub fn from_symbol_pr(pr: &SymbolPr) -> Self {
        // Build initial heap
        let mut heap = BinaryHeap::new();
        for &s in &pr.alphabet {
            let p = pr.s_to_prs.get(&s).cloned().unwrap_or(0.0);
            heap.push(Node { prob: p, sym: Some(s), left: None, right: None });
        }
        if heap.len() == 1 {
            // single symbol edge case: assign code "0"
            let s = heap.pop().unwrap().sym.unwrap();
            let mut codes = HashMap::new();
            codes.insert(s, vec![0]);
            let mut decode_table = HashMap::new();
            decode_table.insert((1, 0), s);
            return Self { codes, decode_table };
        }

        // Build tree
        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            let parent = Node { prob: a.prob + b.prob, sym: None, left: Some(Box::new(a)), right: Some(Box::new(b)) };
            heap.push(parent);
        }
        let root = heap.pop().unwrap();

        // Assign code lengths first
        let mut lengths: HashMap<Symbol, usize> = HashMap::new();
        fn walk(n: &Node, depth: usize, lengths: &mut HashMap<Symbol, usize>) {
            if let Some(s) = n.sym {
                lengths.insert(s, depth.max(1));
            } else {
                if let Some(ref l) = n.left { walk(l, depth + 1, lengths); }
                if let Some(ref r) = n.right { walk(r, depth + 1, lengths); }
            }
        }
        walk(&root, 0, &mut lengths);

        // Canonical assignment
        // Sort by (length asc, symbol asc)
        let mut syms: Vec<(Symbol, usize)> = pr
            .alphabet
            .iter()
            .filter_map(|&s| lengths.get(&s).copied().map(|len| (s, len)))
            .collect();
        syms.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let mut code: u64 = 0;
        let mut prev_len: usize = 0;
        let mut codes: HashMap<Symbol, Vec<u8>> = HashMap::new();
        let mut decode_table: HashMap<(usize, u64), Symbol> = HashMap::new();
        for (s, len) in syms {
            if len > prev_len { code <<= (len - prev_len) as u32; prev_len = len; }
            // record
            let mut bits = Vec::with_capacity(len);
            for i in (0..len).rev() { bits.push(((code >> i) & 1) as u8); }
            codes.insert(s, bits);
            decode_table.insert((len, code), s);
            code += 1;
        }
        Self { codes, decode_table }
    }

    pub fn encode(&self, symbols: &Symbols) -> CodeWords {
        let mut out = Vec::new();
        for &s in symbols {
            if let Some(bits) = self.codes.get(&s) { out.extend_from_slice(bits); }
        }
        out
    }

    pub fn decode(&self, length: usize, bits: &CodeWords) -> Symbols {
        let mut out = Vec::with_capacity(length);
        let mut idx = 0usize;
        let mut acc: u64 = 0;
        let mut acc_len: usize = 0;
        while out.len() < length && idx < bits.len() {
            acc = (acc << 1) | (bits[idx] as u64);
            acc_len += 1;
            idx += 1;
            if let Some(&sym) = self.decode_table.get(&(acc_len, acc)) {
                out.push(sym);
                acc = 0;
                acc_len = 0;
            }
        }
        out
    }
}
