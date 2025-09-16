use wasm_bindgen::prelude::*;

use source_coding::{ArithmeticCode, SymbolPr, HuffmanCode, craft_code, elias_gamma_encode, elias_gamma_decode, JonesCode, Lz78Code, Markov, BlockHuffmanTree, BlockSymbolsPr};

// ---------- Common type conversions ----------
fn vec_u8_to_bools(bits: Vec<u8>) -> Vec<u8> { bits.into_iter().map(|b| if b == 0 { 0 } else { 1 }).collect() }

#[wasm_bindgen(js_name = SourceArithmetic)]
pub struct JsArithmetic(ArithmeticCode);

#[wasm_bindgen(js_class = "SourceArithmetic")]
impl JsArithmetic {
    #[wasm_bindgen(constructor)]
    pub fn new(alphabet: String, probs: Vec<f64>) -> Result<JsArithmetic, JsError> {
        let chars: Vec<char> = alphabet.chars().collect();
        if chars.len() != probs.len() { return Err(JsError::new("alphabet/probs length mismatch")); }
        let mut s_to_prs = std::collections::HashMap::new();
        for (i, &c) in chars.iter().enumerate() { s_to_prs.insert(c, probs[i]); }
        let pr = SymbolPr { alphabet: chars, s_to_prs };
        let ac = ArithmeticCode::new(pr).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(JsArithmetic(ac))
    }
    pub fn encode(&self, symbols: String) -> Result<Vec<u8>, JsError> {
        let syms: Vec<char> = symbols.chars().collect();
        let bits = self.0.encode(&syms).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(vec_u8_to_bools(bits))
    }
    pub fn decode(&self, length: usize, bits: Vec<u8>) -> Result<String, JsError> {
        let bits = vec_u8_to_bools(bits);
        let syms = self.0.decode(length, &bits).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(syms.into_iter().collect())
    }
}

#[wasm_bindgen(js_name = SourceHuffman)]
pub struct JsHuffman(HuffmanCode);

#[wasm_bindgen(js_class = "SourceHuffman")]
impl JsHuffman {
    #[wasm_bindgen(constructor)]
    pub fn new(alphabet: String, probs: Vec<f64>) -> Result<JsHuffman, JsError> {
        let chars: Vec<char> = alphabet.chars().collect();
        if chars.len() != probs.len() { return Err(JsError::new("alphabet/probs length mismatch")); }
        let mut s_to_prs = std::collections::HashMap::new();
        for (i, &c) in chars.iter().enumerate() { s_to_prs.insert(c, probs[i]); }
        let pr = SymbolPr { alphabet: chars, s_to_prs };
        Ok(JsHuffman(HuffmanCode::from_symbol_pr(&pr)))
    }
    pub fn encode(&self, symbols: String) -> Vec<u8> {
        let syms: Vec<char> = symbols.chars().collect();
        vec_u8_to_bools(self.0.encode(&syms))
    }
    pub fn decode(&self, length: usize, bits: Vec<u8>) -> String {
        let bits = vec_u8_to_bools(bits);
        self.0.decode(length, &bits).into_iter().collect()
    }
}

#[wasm_bindgen(js_name = EliasGamma)]
pub struct JsEliasGamma;
#[wasm_bindgen(js_class = "EliasGamma")]
impl JsEliasGamma {
    #[wasm_bindgen(js_name = encode)]
    pub fn encode_js(n: u64) -> Vec<u8> { vec_u8_to_bools(elias_gamma_encode(n)) }
    #[wasm_bindgen(js_name = decode)]
    pub fn decode_js(bits: Vec<u8>, start: usize) -> Option<js_sys::Array> {
        let bits = vec_u8_to_bools(bits);
        elias_gamma_decode(&bits, start).map(|(v, idx)| {
            let arr = js_sys::Array::new();
            arr.push(&JsValue::from_f64(v as f64));
            arr.push(&JsValue::from_f64(idx as f64));
            arr
        })
    }
}

#[wasm_bindgen(js_name = CraftCode)]
pub struct JsCraft;
#[wasm_bindgen(js_class = "CraftCode")]
impl JsCraft {
    #[wasm_bindgen(js_name = build)]
    pub fn build_js(alphabet_size: usize, code_lengths: Vec<usize>) -> js_sys::Array {
        let cb = craft_code(alphabet_size, &code_lengths);
        let arr = js_sys::Array::new();
        for code in cb { let jsbits = js_sys::Uint8Array::from(&code[..]); arr.push(&jsbits.into()); }
        arr
    }
}

#[wasm_bindgen(js_name = Jones)]
pub struct JsJones(JonesCode);
#[wasm_bindgen(js_class = "Jones")]
impl JsJones {
    #[wasm_bindgen(constructor)]
    pub fn new(alphabet: String, probs: Vec<f64>, total: u32) -> Result<JsJones, JsError> {
        let chars: Vec<char> = alphabet.chars().collect();
        if chars.len() != probs.len() { return Err(JsError::new("alphabet/probs length mismatch")); }
        let mut s_to_prs = std::collections::HashMap::new();
        for (i, &c) in chars.iter().enumerate() { s_to_prs.insert(c, probs[i]); }
        let pr = SymbolPr { alphabet: chars, s_to_prs };
        Ok(JsJones(JonesCode::from_symbol_pr(&pr, total as u64)))
    }
    pub fn encode(&self, symbols: String) -> Vec<u8> { let syms: Vec<char> = symbols.chars().collect(); vec_u8_to_bools(self.0.encode(&syms)) }
    pub fn decode(&self, length: usize, bits: Vec<u8>) -> String { self.0.decode(length, &vec_u8_to_bools(bits)).into_iter().collect() }
}

#[wasm_bindgen(js_name = Lz78)]
pub struct JsLz78(Lz78Code);
#[wasm_bindgen(js_class = "Lz78")]
impl JsLz78 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsLz78 { JsLz78(Lz78Code) }
    #[wasm_bindgen(js_name = encodeInternal)]
    pub fn encode_internal_js(&self, input: String) -> js_sys::Array {
        let code = Lz78Code::encode_internal(&input.chars().collect());
        let arr = js_sys::Array::new();
        for (idx, ch) in code { let pair = js_sys::Array::new(); pair.push(&JsValue::from_f64(idx as f64)); pair.push(&JsValue::from_str(&ch.to_string())); arr.push(&pair.into()); }
        arr
    }
    #[wasm_bindgen(js_name = decodeInternal)]
    pub fn decode_internal_js(&self, pairs: js_sys::Array) -> String {
        let mut code = Vec::new();
        for v in pairs.iter() {
            let p = js_sys::Array::from(&v);
            let idx = p.get(0).as_f64().unwrap_or(0.0) as usize;
            let s = p.get(1).as_string().unwrap_or_default();
            let ch = s.chars().next().unwrap_or('\0');
            code.push((idx, ch));
        }
        Lz78Code::decode_internal(&code).into_iter().collect()
    }
}

#[wasm_bindgen(js_name = Markov)]
pub struct JsMarkov(Markov);
#[wasm_bindgen(js_class = "Markov")]
impl JsMarkov {
    #[wasm_bindgen(constructor)]
    pub fn new(alphabet: String, init_pr: Vec<f64>, cond_pr: js_sys::Array) -> JsMarkov {
        let alphabet: Vec<char> = alphabet.chars().collect();
        let mut init = std::collections::HashMap::new();
        for (i, &c) in alphabet.iter().enumerate() { init.insert(c, init_pr.get(i).copied().unwrap_or(0.0)); }
        let mut cond = std::collections::HashMap::new();
        for v in cond_pr.iter() {
            let t = js_sys::Array::from(&v);
            let a = t.get(0).as_string().unwrap_or_default().chars().next().unwrap_or('\0');
            let b = t.get(1).as_string().unwrap_or_default().chars().next().unwrap_or('\0');
            let p = t.get(2).as_f64().unwrap_or(0.0);
            cond.insert((a, b), p);
        }
        JsMarkov(Markov::new(alphabet, init, cond))
    }
    #[wasm_bindgen(js_name = blockPr)]
    pub fn block_pr(&self, symbols: String) -> f64 { self.0.block_pr(&symbols.chars().collect()) }
}

#[wasm_bindgen(js_name = BlockHuffman)]
pub struct JsBlockHuffman(BlockHuffmanTree);
#[wasm_bindgen(js_class = "BlockHuffman")]
impl JsBlockHuffman {
    #[wasm_bindgen(constructor)]
    pub fn new(q: usize, blocks: js_sys::Array, probs: Vec<f64>) -> JsBlockHuffman {
        let mut block_alphabet: Vec<Vec<char>> = Vec::new();
        for item in blocks.iter() {
            let arr = js_sys::Array::from(&item);
            let mut s = Vec::new();
            for ch in arr.iter() { let c = ch.as_string().unwrap_or_default().chars().next().unwrap_or('\0'); s.push(c); }
            block_alphabet.push(s);
        }
        let mut ss_to_prs = std::collections::HashMap::new();
        for (i, ss) in block_alphabet.iter().enumerate() { ss_to_prs.insert(ss.clone(), probs.get(i).copied().unwrap_or(0.0)); }
        let pr = BlockSymbolsPr { block_alphabet, ss_to_prs };
        JsBlockHuffman(BlockHuffmanTree::new(q, pr))
    }
    pub fn encode(&self, blocks: js_sys::Array) -> Vec<usize> {
        let mut seq: Vec<Vec<char>> = Vec::new();
        for item in blocks.iter() {
            let arr = js_sys::Array::from(&item);
            let mut s = Vec::new();
            for ch in arr.iter() { let c = ch.as_string().unwrap_or_default().chars().next().unwrap_or('\0'); s.push(c); }
            seq.push(s);
        }
        self.0.encode(&seq)
    }
    pub fn decode(&self, length: usize, digits: Vec<usize>) -> js_sys::Array {
        let blocks = self.0.decode(length, &digits);
        let arr = js_sys::Array::new();
        for ss in blocks { let a = js_sys::Array::new(); for c in ss { a.push(&JsValue::from_str(&c.to_string())); } arr.push(&a); }
        arr
    }
}
