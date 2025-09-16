use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::Symbols;

pub type SymbolsToCodeWord = HashMap<Symbols, Vec<usize>>; // q-ary digits 0..q-1

#[derive(Debug, Clone)]
pub struct SymbolsPr {
    pub block_alphabet: Vec<Symbols>,
    pub ss_to_prs: HashMap<Symbols, f64>,
}

#[derive(Debug, Clone)]
struct Node {
    // internal: children indices; leaf: empty
    children: Vec<usize>,
    sym: Option<Symbols>,
    pr: f64,
}

#[derive(Debug, Clone)]
struct HeapItem(usize, f64); // (node index, prob)

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering { other.1.partial_cmp(&self.1).unwrap_or(Ordering::Equal) }
}
impl PartialOrd for HeapItem { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl PartialEq for HeapItem { fn eq(&self, other: &Self) -> bool { self.1 == other.1 } }
impl Eq for HeapItem {}

pub struct BlockHuffmanTree {
    q: usize,
    symbols_pr: SymbolsPr,
}

impl BlockHuffmanTree {
    pub fn new(q: usize, symbols_pr: SymbolsPr) -> Self { Self { q: q.max(2), symbols_pr } }

    fn build_tree(&self) -> (Vec<Node>, usize) {
        let mut nodes: Vec<Node> = Vec::new();
        let mut heap = BinaryHeap::new();
        for s in &self.symbols_pr.block_alphabet {
            let pr = *self.symbols_pr.ss_to_prs.get(s).unwrap_or(&0.0);
            nodes.push(Node { children: vec![], sym: Some(s.clone()), pr });
            heap.push(HeapItem(nodes.len() - 1, pr));
        }
        if nodes.len() == 1 {
            // single symbol, tree is just that leaf
            return (nodes, 0);
        }
        if self.q > 2 {
            let n = self.symbols_pr.block_alphabet.len();
            let pad = (self.q - 1 - (n - 1) % (self.q - 1)) % (self.q - 1);
            for _ in 0..pad {
                nodes.push(Node { children: vec![], sym: None, pr: 0.0 });
                heap.push(HeapItem(nodes.len() - 1, 0.0));
            }
        }
        let mut root_idx = 0usize;
        while heap.len() > 1 {
            let mut ch: Vec<usize> = Vec::with_capacity(self.q);
            let mut pr_sum = 0.0;
            for _ in 0..self.q {
                let HeapItem(i, p) = heap.pop().unwrap();
                ch.push(i);
                pr_sum += p;
            }
            nodes.push(Node { children: ch, sym: None, pr: pr_sum });
            let new_idx = nodes.len() - 1;
            heap.push(HeapItem(new_idx, pr_sum));
            root_idx = new_idx;
        }
        (nodes, root_idx)
    }

    pub fn huffman_code(&self) -> SymbolsToCodeWord {
        let (nodes, root_idx) = self.build_tree();
        let mut map: SymbolsToCodeWord = HashMap::new();
        if nodes.len() == 1 {
            // Only one symbol: assign 0
            if let Some(sym) = nodes[0].sym.clone() { map.insert(sym, vec![0]); }
            return map;
        }
        fn dfs(nodes: &Vec<Node>, q: usize, cur: usize, path: &mut Vec<usize>, out: &mut SymbolsToCodeWord) {
            let node = &nodes[cur];
            if let Some(sym) = &node.sym {
                // skip padded leaves (sym None handled before)
                out.insert(sym.clone(), path.clone());
                return;
            }
            for (i, &child) in node.children.iter().enumerate() {
                if i >= q { break; }
                path.push(i);
                dfs(nodes, q, child, path, out);
                path.pop();
            }
        }
        let mut tmp = Vec::new();
        dfs(&nodes, self.q, root_idx, &mut tmp, &mut map);
        map
    }

    pub fn encode(&self, blocks: &[Symbols]) -> Vec<usize> {
        let codes = self.huffman_code();
        let mut out: Vec<usize> = Vec::new();
        if codes.len() == 1 {
            // single-symbol edge case: nothing to emit, but return zeros of length blocks.len()
            out.resize(blocks.len(), 0);
            return out;
        }
        for b in blocks {
            if let Some(code) = codes.get(b) { out.extend_from_slice(code); }
        }
        out
    }

    pub fn decode(&self, length: usize, digits: &[usize]) -> Vec<Symbols> {
        // Build a trie from codes
        #[derive(Default)]
        struct Trie { children: Vec<Option<Box<Trie>>>, sym: Option<Symbols> }
        fn mk_children(q: usize) -> Vec<Option<Box<Trie>>> {
            let mut v: Vec<Option<Box<Trie>>> = Vec::with_capacity(q);
            for _ in 0..q { v.push(None); }
            v
        }
        let codes = self.huffman_code();
        if codes.len() == 1 {
            // single symbol repeated
            let sym = codes.keys().next().unwrap().clone();
            return vec![sym; length];
        }
        let mut root = Trie { children: mk_children(self.q), sym: None };
        for (sym, code) in &codes {
            let mut node = &mut root;
            for &d in code {
                if d >= self.q { continue; }
                if node.children.is_empty() { node.children = mk_children(self.q); }
                if node.children[d].is_none() { node.children[d] = Some(Box::new(Trie { children: mk_children(self.q), sym: None })); }
                node = node.children[d].as_mut().unwrap();
            }
            node.sym = Some(sym.clone());
        }
        let mut out: Vec<Symbols> = Vec::with_capacity(length);
        let mut node = &root;
        for &d in digits {
            if d >= self.q { continue; }
            if let Some(ref child) = node.children.get(d).and_then(|x| x.as_ref()) {
                if child.sym.is_some() {
                    out.push(child.sym.as_ref().unwrap().clone());
                    node = &root;
                    if out.len() >= length { break; }
                } else {
                    node = child;
                }
            } else {
                node = &root; // reset on invalid path
            }
        }
        out
    }
}
