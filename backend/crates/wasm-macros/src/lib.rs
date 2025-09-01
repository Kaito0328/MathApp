use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, Expr, ExprLit, FnArg, Ident, ImplItem, ItemFn, ItemImpl, Lit, Meta, Pat, PatType, Path, Result, ReturnType, Token, Type
};

fn get_export_name(args: &Punctuated<Meta, Token![,]>, _default: &str) -> Option<String> {
    for meta in args.iter() {
        if let Meta::NameValue(nv) = meta {
            if nv.path.is_ident("export") {
                if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &nv.value {
                    return Some(s.value());
                }
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn wasm_json(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let args = parser.parse(attr).unwrap_or_else(|_| Punctuated::new());
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let export_name = get_export_name(&args, &fn_name.to_string()).unwrap_or_else(|| fn_name.to_string());

    // 引数収集: self/receiverは禁止。1個以上。
    let mut arg_tys: Vec<Type> = Vec::new();
    let mut field_idents: Vec<proc_macro2::Ident> = Vec::new();
    let mut local_idents: Vec<proc_macro2::Ident> = Vec::new();

    for (i, arg) in input_fn.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                let err = quote! { compile_error!("#[wasm_json] does not support methods with self; use a free function or a facade."); };
                return TokenStream::from(quote! { #input_fn #err });
            }
            FnArg::Typed(PatType { pat, ty, .. }) => {
                arg_tys.push((**ty).clone());
                let fid = match &**pat {
                    Pat::Ident(pi) => pi.ident.clone(),
                    _ => syn::Ident::new(&format!("arg{}", i), proc_macro2::Span::call_site()),
                };
                field_idents.push(fid);
                local_idents.push(syn::Ident::new(&format!("__arg{}", i), proc_macro2::Span::call_site()));
            }
        }
    }

    if arg_tys.is_empty() {
        let err = quote! { compile_error!("#[wasm_json] requires at least one argument"); };
        return TokenStream::from(quote! { #input_fn #err });
    }

    // 返り値が Result<T, E> かどうかを判定
    let ret_is_result;
    match &input_fn.sig.output {
        ReturnType::Default => {
            ret_is_result = false;
        }
        ReturnType::Type(_, ty) => {
            // Path が Result<..> なら true
            let mut is_res = false;
            if let Type::Path(tp) = &**ty {
                if let Some(seg) = tp.path.segments.last() {
                    if seg.ident == "Result" { is_res = true; }
                }
            }
            ret_is_result = is_res;
        }
    }

    let wrapper_ident = format_ident!("{}_wasm", fn_name);

    // 逆シリアライズロジック（単一/複数）
    let deser_code = if arg_tys.len() == 1 {
        let ty0 = &arg_tys[0];
        let l0 = &local_idents[0];
        let f0 = &field_idents[0];
        let helper_ident = format_ident!("__{}_args", fn_name);
        quote! {
            // まずは単一値としてデコード
            if let Ok(v) = serde_wasm_bindgen::from_value::<#ty0>(input.clone()) {
                let #l0 = v;
            } else if let Ok(tuple0) = serde_wasm_bindgen::from_value::<(#ty0,)>(input.clone()) {
                let (#l0,) = tuple0;
            } else {
                #[derive(serde::Deserialize)]
                struct #helper_ident { pub #f0: #ty0 }
                let args: #helper_ident = serde_wasm_bindgen::from_value(input)
                    .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde from_value error: {}", e)))?;
                let #l0 = args.#f0;
            }
        }
    } else {
        // 複数引数: タプル→オブジェクト順に試す
        let tuple_tys = quote! { ( #(#arg_tys),* ) };
        let tuple_pat = quote! { ( #(#local_idents),* ) };
        let helper_ident = format_ident!("__{}_args", fn_name);
        quote! {
            if let Ok(tuple_args) = serde_wasm_bindgen::from_value::<#tuple_tys>(input.clone()) {
                let #tuple_pat = tuple_args;
            } else {
                #[derive(serde::Deserialize)]
                struct #helper_ident { #(pub #field_idents: #arg_tys),* }
                let args: #helper_ident = serde_wasm_bindgen::from_value(input)
                    .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde from_value error: {}", e)))?;
                #(let #local_idents = args.#field_idents;)*
            }
        }
    };

    // 呼び出しと戻り値のJsValue化
    let call_args = quote! { #(#local_idents),* };
    let call_and_convert = if ret_is_result {
        quote! {
            match #fn_name(#call_args) {
                Ok(v) => serde_wasm_bindgen::to_value(&v)
                    .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde to_value error: {}", e))),
                Err(e) => Err(wasm_bindgen::JsValue::from_str(&format!("{}", e)))
            }
        }
    } else {
        quote! {
            let v = #fn_name(#call_args);
            serde_wasm_bindgen::to_value(&v)
                .map_err(|e| wasm_bindgen::JsValue::from_str(&format!("serde to_value error: {}", e)))
        }
    };

    let expanded = quote! {
        // 元関数はそのまま保持
        #input_fn

        // JS公開用ラッパー（単一DTOのJsValue受け渡し）
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #export_name)]
        pub fn #wrapper_ident(input: wasm_bindgen::JsValue) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
            #deser_code
            #call_and_convert
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn wasm_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 直通型向け: そのまま #[wasm_bindgen] を付与
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let args = parser.parse(attr).unwrap_or_else(|_| Punctuated::new());
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let export_name = get_export_name(&args, &fn_name.to_string()).unwrap_or_else(|| fn_name.to_string());

    let expanded = quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #export_name)]
        #input_fn
    };
    TokenStream::from(expanded)
}

// 複数のimplを1つのwasmクラスにまとめるためのマクロ引数
struct WasmClassMergeArgs {
    internal_ty: Type,
    js_name: String,
    ops: Vec<Ident>,
    indexer: bool,
    iterator: bool,
}

impl Parse for WasmClassMergeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut internal_ty = None;
        let mut js_name = None;
        let mut ops = Vec::new();
        let mut indexer = false;
        let mut iterator = false;

        let metas = input.parse_terminated(Meta::parse, Token![,])?;
        for meta in metas {
            match meta {
                Meta::NameValue(nv) => {
                    if nv.path.is_ident("internal") {
                        match nv.value {
                            syn::Expr::Lit(expr_lit) => {
                                if let Lit::Str(s) = expr_lit.lit {
                                    internal_ty = Some(syn::parse_str(&s.value())?);
                                }
                            }
                            syn::Expr::Macro(expr_mac) => {
                                let tokens = expr_mac.mac.tokens.clone();
                                internal_ty = Some(syn::parse2::<Type>(tokens)?);
                            }
                            _ => {}
                        }
                    } else if nv.path.is_ident("js_name") {
                        match nv.value {
                            syn::Expr::Lit(expr_lit) => {
                                if let Lit::Str(s) = expr_lit.lit {
                                    js_name = Some(s.value());
                                }
                            }
                            syn::Expr::Macro(expr_mac) => {
                                let s = expr_mac.mac.tokens.to_string();
                                js_name = Some(s.replace(' ', ""));
                            }
                            _ => {}
                        }
                    } else if nv.path.is_ident("indexer") {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let Lit::Bool(b) = expr_lit.lit {
                                indexer = b.value();
                            }
                        }
                    } else if nv.path.is_ident("iterator") {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let Lit::Bool(b) = expr_lit.lit {
                                iterator = b.value();
                            }
                        }
                    }
                }
                Meta::List(ml) => {
                    if ml.path.is_ident("ops") {
                        let parser = Punctuated::<Path, Token![,]>::parse_terminated;
                        let paths = parser.parse(ml.tokens.into())?;
                        for p in paths {
                            if let Some(ident) = p.get_ident() {
                                ops.push(ident.clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(WasmClassMergeArgs {
            internal_ty: internal_ty
                .ok_or_else(|| input.error("`internal = \"...\"` attribute is required"))?,
            js_name: js_name
                .ok_or_else(|| input.error("`js_name = \"...\"` attribute is required"))?,
            ops,
            indexer,
            iterator,
        })
    }
}

#[proc_macro_attribute]
pub fn wasm_class_merge(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as WasmClassMergeArgs);
    let internal_ty = &args.internal_ty;
    let js_name_str = &args.js_name;
    let wasm_ty = format_ident!("{}", js_name_str);
    
    // 複数のimplブロックをパース
    let input = syn::parse::<syn::File>(item).expect("Expected multiple impl blocks");
    
    let mut all_methods = Vec::new();
    
    // 各impleブロックからメソッドを収集
    for item in input.items {
        if let syn::Item::Impl(input_impl) = item {
            let methods = input_impl
                .items
                .iter()
                .filter_map(|item| {
                    if let ImplItem::Fn(method) = item {
                        let sig = &method.sig;
                        let method_name = &sig.ident;
                        let output = &sig.output;

                        // マーカーアトリビュートをチェック
                        let mut is_constructor = false;
                        for attr in &method.attrs {
                            if attr.path().is_ident("constructor") {
                                is_constructor = true;
                                break;
                            }
                        }
                        
                        let final_attrs = method
                            .attrs
                            .iter()
                            .filter(|a| !a.path().is_ident("constructor"))
                            .collect::<Vec<_>>();

                        let constructor_attr = if is_constructor {
                            quote! { #[wasm_bindgen(constructor)] }
                        } else {
                            quote! {}
                        };

                        let (final_sig_args, call_args) = build_args(sig, &wasm_ty);
                        let is_method = matches!(sig.inputs.first(), Some(syn::FnArg::Receiver(_)));

                        if is_method {
                            let body = if is_self_return(output) {
                                quote! { Self(self.0.#method_name(#call_args)) }
                            } else {
                                quote! { self.0.#method_name(#call_args) }
                            };

                            Some(quote! {
                                #(#final_attrs)*
                                pub fn #method_name(#final_sig_args) #output {
                                    #body
                                }
                            })
                        } else {
                            Some(quote! {
                                #(#final_attrs)*
                                #constructor_attr
                                pub fn #method_name(#final_sig_args) #output {
                                    Self(#internal_ty::#method_name(#call_args))
                                }
                            })
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            
            all_methods.extend(methods);
        }
    }

    // ops引数に基づいて四則演算メソッドを自動生成
    for op in args.ops {
        let method_name = format_ident!("{}", op.to_string().to_lowercase());
        let op_symbol = match op.to_string().as_str() {
            "Add" => quote! { + },
            "Sub" => quote! { - },
            "Mul" => quote! { * },
            "Div" => quote! { / },
            _ => continue,
        };
        all_methods.push(quote! {
            pub fn #method_name(&self, rhs: &#wasm_ty) -> Self {
                Self(self.0.clone() #op_symbol rhs.0.clone())
            }
        });
    }

    // `internal`の型からジェネリック`<T>`の部分を抽出
    let internal_generic_ty =
        if let Type::Path(type_path) = internal_ty {
            if let Some(last_seg) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                        Some(ty.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

    // indexer = trueなら、getter/setterを自動生成
    if args.indexer {
        if let Some(ty) = &internal_generic_ty {
            all_methods.push(quote! {
                #[wasm_bindgen(getter)]
                pub fn get(&self, index: usize) -> #ty {
                    self.0[index].clone()
                }
            });
            all_methods.push(quote! {
                #[wasm_bindgen(setter)]
                pub fn set(&mut self, index: usize, value: #ty) {
                    self.0[index] = value;
                }
            });
        }
    }

    // iterator = trueなら、データを配列で返すメソッドを自動生成
    if args.iterator {
        if let Some(ty) = &internal_generic_ty {
            all_methods.push(quote! {
                #[wasm_bindgen(js_name = "toArray")]
                pub fn to_array(&self) -> Vec<#ty> {
                    self.0.clone().into_iter().collect()
                }
            });
        }
    }

    // 最終的なコードを生成
    let expanded = quote! {
        #[allow(dead_code)]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name_str)]
        #[derive(Clone)]
        pub struct #wasm_ty(#internal_ty);

        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name_str)]
        impl #wasm_ty {
            #(#all_methods)*
        }
    };

    TokenStream::from(expanded)
}

// マクロの引数 #[wasm_class(internal = "...", js_name = "...", ...)] をパースするための構造体
struct WasmClassArgs {
    internal_ty: Type,
    js_name: String,
    ops: Vec<Ident>,
    indexer: bool,
    iterator: bool,
}

// マクロ引数のパーサー
impl Parse for WasmClassArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut internal_ty = None;
        let mut js_name = None;
        let mut ops = Vec::new();
        let mut indexer = false;
        let mut iterator = false;

        let metas = input.parse_terminated(Meta::parse, Token![,])?;
        for meta in metas {
            match meta {
                Meta::NameValue(nv) => {
                        if nv.path.is_ident("internal") {
                            match nv.value {
                                syn::Expr::Lit(expr_lit) => {
                                    if let Lit::Str(s) = expr_lit.lit {
                                        internal_ty = Some(syn::parse_str(&s.value())?);
                                    }
                                }
                                syn::Expr::Macro(expr_mac) => {
                                    // 例: internal = stringify!(linalg::Matrix<f64>) のような場合
                                    // マクロ内のトークン列を型としてパースして受け取る
                                    let tokens = expr_mac.mac.tokens.clone();
                                    internal_ty = Some(syn::parse2::<Type>(tokens)?);
                                }
                                _ => {}
                            }
                    } else if nv.path.is_ident("js_name") {
                            match nv.value {
                                syn::Expr::Lit(expr_lit) => {
                                    if let Lit::Str(s) = expr_lit.lit {
                                        js_name = Some(s.value());
                                    }
                                }
                                syn::Expr::Macro(expr_mac) => {
                                    // 例: js_name = stringify!(MatrixF64)
                                    let s = expr_mac.mac.tokens.to_string();
                                    js_name = Some(s.replace(' ', ""));
                                }
                                _ => {}
                            }
                    } else if nv.path.is_ident("indexer") {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let Lit::Bool(b) = expr_lit.lit {
                                indexer = b.value();
                            }
                        }
                    } else if nv.path.is_ident("iterator") {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let Lit::Bool(b) = expr_lit.lit {
                                iterator = b.value();
                            }
                        }
                    }
                }
                Meta::List(ml) => {
                    if ml.path.is_ident("ops") {
                        let parser = Punctuated::<Path, Token![,]>::parse_terminated;
                        let paths = parser.parse(ml.tokens.into())?;
                        for p in paths {
                            if let Some(ident) = p.get_ident() {
                                ops.push(ident.clone());
                            }
                        }
                    } else if ml.path.is_ident("delegate") {
                        // delegate属性は削除されたため、無視する
                    }
                }
                _ => {} // Meta::Path は無視
            }
        }

        Ok(WasmClassArgs {
            internal_ty: internal_ty
                .ok_or_else(|| input.error("`internal = \"...\"` attribute is required"))?,
            js_name: js_name
                .ok_or_else(|| input.error("`js_name = \"...\"` attribute is required"))?,
            ops,
            indexer,
            iterator,
        })
    }
}

#[proc_macro_attribute]
pub fn wasm_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as WasmClassArgs);
    let internal_ty = &args.internal_ty;
    let js_name_str = &args.js_name;
    let wasm_ty = format_ident!("{}", js_name_str);
    let input_impl = parse_macro_input!(item as ItemImpl);

    // ユーザーが宣言したメソッドを生成
    let mut methods = input_impl.items.iter().filter_map(|item| {
        if let ImplItem::Fn(method) = item {
            let sig = &method.sig;
            let method_name = &sig.ident;
            let output = &sig.output;
            
            // --- ここからが修正・追加箇所 ---
            let mut is_constructor = false;
            let mut trait_path = None;

            for attr in &method.attrs {
                if attr.path().is_ident("constructor") {
                    is_constructor = true;
                } else if attr.path().is_ident("trait_method") {
                    // `#[trait_method(Ring)]` の `Ring` 部分をPathとしてパース
                    if let Ok(path) = attr.parse_args::<Path>() {
                        trait_path = Some(path);
                    }
                }
            }
            // マーカーとして使ったアトリビュートは最終コードから除外
            let final_attrs = method.attrs.iter()
                .filter(|a| !a.path().is_ident("constructor") && !a.path().is_ident("trait_method"))
                .collect::<Vec<_>>();
            
            let constructor_attr = if is_constructor { quote! { #[wasm_bindgen(constructor)] } } else { quote! {} };
            
            let (final_sig_args, call_args) = build_args(sig, &wasm_ty);
            let is_method = matches!(sig.inputs.first(), Some(syn::FnArg::Receiver(_)));

            // メソッド呼び出し部分を、トレイトの有無で分岐させる
            let method_call = if let Some(trait_p) = trait_path {
                // トレイトメソッドの場合 (例: Ring::identity(size))
                if is_method {
                    quote! { #trait_p::#method_name(&self.0, #call_args) }
                } else {
                    quote! { #trait_p::#method_name(#call_args) }
                }
            } else {
                // Inherent method の場合 (これまでと同じ)
                if is_method {
                    quote! { self.0.#method_name(#call_args) }
                } else {
                    quote! { #internal_ty::#method_name(#call_args) }
                }
            };
            // --- 修正・追加ここまで ---

            let body = if is_self_return(output) {
                quote! { Self(#method_call) }
            } else {
                quote! { #method_call }
            };

            if is_method {
                Some(quote! {
                    #(#final_attrs)*
                    pub fn #method_name(#final_sig_args) #output { #body }
                })
            } else {
                Some(quote! {
                    #(#final_attrs)*
                    #constructor_attr
                    pub fn #method_name(#final_sig_args) #output { #body }
                })
            }
        } else { None }
    }).collect::<Vec<_>>();

    // ops, indexer, iterator の自動生成ロジック (変更なし)
    for op in args.ops {
        let method_name = format_ident!("{}", op.to_string().to_lowercase());
        let op_symbol = match op.to_string().as_str() {
            "Add" => quote! { + }, "Sub" => quote! { - },
            "Mul" => quote! { * }, "Div" => quote! { / },
            _ => continue,
        };
        methods.push(quote! {
            pub fn #method_name(&self, rhs: &#wasm_ty) -> Self {
                Self(self.0.clone() #op_symbol rhs.0.clone())
            }
        });
    }

    let internal_generic_ty = get_internal_generic(internal_ty);

    if args.indexer {
        if let Some(ty) = &internal_generic_ty {
             methods.push(quote! {
                #[wasm_bindgen(getter)]
                pub fn get(&self, index: usize) -> #ty { self.0[index].clone() }
            });
            methods.push(quote! {
                #[wasm_bindgen(setter)]
                pub fn set(&mut self, index: usize, value: #ty) { self.0[index] = value; }
            });
        }
    }
    if args.iterator {
        if let Some(ty) = &internal_generic_ty {
            methods.push(quote! {
                #[wasm_bindgen(js_name = "toArray")]
                pub fn to_array(&self) -> Vec<#ty> { self.0.clone().into_iter().collect() }
            });
        }
    }

    // 最終的なコードを生成 (変更なし)
    let expanded = quote! {
        #[allow(dead_code)]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name_str)]
        #[derive(Clone)]
        pub struct #wasm_ty(#internal_ty);

        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name_str)]
        impl #wasm_ty {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

// -----------------------------------------------------------------------------
// ヘルパー関数群
// -----------------------------------------------------------------------------

// 引数リストを構築するヘルパー関数
fn build_args(sig: &syn::Signature, wasm_ty: &Ident) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut call_args = proc_macro2::TokenStream::new();
    let mut fn_args_without_receiver = proc_macro2::TokenStream::new();
    let is_method = matches!(sig.inputs.first(), Some(syn::FnArg::Receiver(_)));

    for arg in sig.inputs.iter().skip(if is_method { 1 } else { 0 }) {
        if let syn::FnArg::Typed(pt) = arg {
            if let syn::Pat::Ident(pi) = &*pt.pat {
                let arg_name = &pi.ident;
                let arg_ty = &pt.ty;
                
                let new_arg_ty = if quote!(#arg_ty).to_string() == "Self" { quote! { & #wasm_ty } } else { quote! { #arg_ty } };
                fn_args_without_receiver.extend(quote! { #arg_name: #new_arg_ty, });

                if quote!(#arg_ty).to_string() == "Self" { call_args.extend(quote! { &#arg_name.0, }); } else { call_args.extend(quote! { #arg_name, }); }
            }
        }
    }

    let final_sig_args = if is_method {
        let mut final_args = quote! { &self, };
        final_args.extend(fn_args_without_receiver);
        final_args
    } else {
        fn_args_without_receiver
    };

    (final_sig_args, call_args)
}

// 返り値をSelf()でラップするかどうかを決定するヘルパー関数
fn is_self_return(output: &syn::ReturnType) -> bool {
    if let syn::ReturnType::Type(_, ty) = output {
        if quote!(#ty).to_string() == "Self" {
            return true;
        }
    }
    false
}

fn get_internal_generic(internal_ty: &Type) -> Option<Type> {
    if let Type::Path(type_path) = internal_ty {
        if let Some(last_seg) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                    return Some(ty.clone());
                }
            }
        }
    }
    None
}