
pub type CodeWord = Vec<u8>;
pub type CodeBook = Vec<CodeWord>; // index 0 unused; index i => code for symbol i

// クラフト不等式を満たす符号長列から接頭語符号を構成する（2分木）
// 入力: alphabet_size (>= number of lengths), code_lengths per symbol index starting at 1
pub fn craft_code(alphabet_size: usize, code_lengths: &[usize]) -> CodeBook {
    // ノードを BFS で割り当てながら、必要な深さの葉を作る
    #[derive(Clone)]
    struct Node { left: Option<Box<Node>>, right: Option<Box<Node>>, is_leaf: bool, symbol: usize }
    let mut root = Node { left: None, right: None, is_leaf: false, symbol: 0 };

    fn make_children(n: &mut Node) {
        if n.left.is_none() { n.left = Some(Box::new(Node { left: None, right: None, is_leaf: false, symbol: 0 })); }
        if n.right.is_none() { n.right = Some(Box::new(Node { left: None, right: None, is_leaf: false, symbol: 0 })); }
    }

    fn assign(depth: usize, node: &mut Node, symbol_index: usize) -> bool {
        if node.is_leaf { return false; }
        if depth == 0 {
            node.is_leaf = true;
            node.symbol = symbol_index;
            return true;
        }
        make_children(node);
        if let Some(ref mut l) = node.left { if assign(depth-1, l, symbol_index) { return true; } }
        if let Some(ref mut r) = node.right { if assign(depth-1, r, symbol_index) { return true; } }
        false
    }

    for i in 0..alphabet_size.min(code_lengths.len()) {
        let depth = code_lengths[i];
        let _ = assign(depth, &mut root, i+1);
    }

    let mut codebook: CodeBook = vec![vec![]; alphabet_size+1];
    fn build(node: &Node, code: &mut Vec<u8>, out: &mut CodeBook) {
        if node.is_leaf {
            if node.symbol < out.len() { out[node.symbol] = code.clone(); }
            return;
        }
        if let Some(ref l) = node.left {
            code.push(0); build(l, code, out); code.pop();
        }
        if let Some(ref r) = node.right {
            code.push(1); build(r, code, out); code.pop();
        }
    }
    let mut tmp = vec![];
    build(&root, &mut tmp, &mut codebook);
    codebook
}
