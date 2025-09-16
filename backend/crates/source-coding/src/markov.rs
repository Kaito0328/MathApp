use std::collections::HashMap;

use crate::{Alphabet, Symbol, Symbols};

#[derive(Debug, Clone)]
pub struct Markov {
    pub alphabet: Alphabet,
    pub init_pr: HashMap<Symbol, f64>,
    pub cond_pr: HashMap<(Symbol, Symbol), f64>, // P(next | prev)
}

impl Markov {
    pub fn new(alphabet: Alphabet, init_pr: HashMap<Symbol, f64>, cond_pr: HashMap<(Symbol, Symbol), f64>) -> Self {
        Self { alphabet, init_pr, cond_pr }
    }

    // Generate probability of a symbol block (sequence)
    pub fn block_pr(&self, block: &Symbols) -> f64 {
        if block.is_empty() { return 0.0; }
        let mut p = *self.init_pr.get(&block[0]).unwrap_or(&0.0);
        for i in 1..block.len() {
            let prev = block[i-1];
            let cur = block[i];
            p *= self.cond_pr.get(&(prev, cur)).cloned().unwrap_or(0.0);
        }
        p
    }

    // Generate all length-m sequences' probabilities (simple, may explode)
    pub fn enumerate_block_pr(&self, m: usize) -> Vec<(Symbols, f64)> {
        let mut res: Vec<(Symbols, f64)> = Vec::new();
        fn dfs(this: &Markov, m: usize, cur: &mut Symbols, out: &mut Vec<(Symbols, f64)>) {
            if cur.len() == m {
                let p = this.block_pr(cur);
                out.push((cur.clone(), p));
                return;
            }
            for &s in &this.alphabet {
                cur.push(s);
                dfs(this, m, cur, out);
                cur.pop();
            }
        }
        let mut tmp = Vec::new();
        dfs(self, m, &mut tmp, &mut res);
        res
    }
}
