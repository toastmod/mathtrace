use std::any::{Any, TypeId};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, Expr, ExprBinary, ExprCall, ExprLit, Item, ItemFn, Lit, Pat, Stmt, parse_macro_input, parse_quote};

// pub fn type_extract<'static, T>(t: &T) -> bool {
//     let type_id = t.type_id(); 
//     type_id == TypeId::of::<u8>()
//         || type_id == TypeId::of::<u16>()
//         || type_id == TypeId::of::<u32>()
//         || type_id == TypeId::of::<u64>()
//         || type_id == TypeId::of::<usize>()
//         || type_id == TypeId::of::<i8>()
//         || type_id == TypeId::of::<i16>()
//         || type_id == TypeId::of::<i32>()
//         || type_id == TypeId::of::<i64>()
//         || type_id == TypeId::of::<isize>()
//         || type_id == TypeId::of::<f32>()
//         || type_id == TypeId::of::<f64>()
// } 

// Recursively generate trace string for any expression
fn trace_expr_rec(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Binary(bin) => {
            let left = &bin.left;
            let right = &bin.right;
            let op = &bin.op;
            let left_tokens = trace_expr_rec(left);
            let right_tokens = trace_expr_rec(right);

            quote! {
                format!("({}) {} ({})", #left_tokens, stringify!(#op), #right_tokens)
            }
        }
        Expr::Call(call) => {
            let call_expr = quote! { #call };
            // quote! { format!("{} = {:?}", stringify!(#call_expr), #call_expr) }
            quote! { format!("{}", stringify!(#call_expr)) }
        }
        Expr::Lit(ExprLit { lit: Lit::Int(_) | Lit::Float(_), .. }) => {
            quote! { format!("{:?}", #expr) }
        }
        Expr::Path(_) => {
            quote! { format!("{}", stringify!(#expr)) }
        }
        _ => {
            let expr_tokens = quote! { #expr };
            quote! { format!("{:?}", #expr_tokens) }
        }
    }
}

// Recursively process any block
fn process_block(block: &mut Block) {
    let mut new_stmts = Vec::new();

    for stmt in &mut block.stmts {
        let sstmt = stmt.clone();
        match stmt {
            Stmt::Local(local) => {

                new_stmts.push(sstmt);
                if let Some(init) = &local.init {
                    let name = &local.pat;
                    let expr = &init.expr;

                    if let Expr::Binary(_) = expr.as_ref() {
                        let etrace = trace_expr_rec(expr);
                        let trace_stmt: Stmt = parse_quote! {
                            println!("{} = {} = {}", stringify!(#name), #etrace, #name);
                        };
    
                        new_stmts.push(trace_stmt);

                    } else {
                        // Handle function type decoding here
                    }

                }

            }

            Stmt::Expr(expr, _) => {
                match expr {
                    Expr::Binary(_) => {
                        let etrace = trace_expr_rec(&expr);
                        let trace_stmt: Stmt = parse_quote! {
                            println!("{}", #etrace);
                        };
                        new_stmts.push(trace_stmt);
                    },
                    // Expr::Call(c) => {
                    //     // If it's a call, check it's return type
                    //     let etrace = trace_expr_rec(&expr);
                    //     let trace_stmt: Stmt = parse_quote! {
                    //         println!("{}", #etrace);
                    //     };
                    //     new_stmts.push(trace_stmt);
                    // }
                    _ => {}
                }

                // recurse into nested blocks
                match expr {
                    Expr::If(expr_if) => process_block(&mut expr_if.then_branch),
                    Expr::Block(expr_block) => process_block(&mut expr_block.block),
                    Expr::While(expr_while) => process_block(&mut expr_while.body),
                    Expr::ForLoop(expr_for) => process_block(&mut expr_for.body),
                    _ => {}
                }

                new_stmts.push(stmt.clone());
            }

            _ => new_stmts.push(stmt.clone()),
        }
    }

    block.stmts = new_stmts;
}

#[proc_macro_attribute]
pub fn mathtrace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse any Rust item
    let mut item = parse_macro_input!(input as Item);

    match &mut item {
        Item::Fn(func) => {
            process_block(&mut func.block);
        }
        Item::Impl(item_impl) => {
            for impl_item in &mut item_impl.items {
                if let syn::ImplItem::Fn(method) = impl_item {
                    process_block(&mut method.block);
                }
            }
        }
        Item::Mod(item_mod) => {
            if let Some((_, items)) = &mut item_mod.content {
                for inner_item in items {
                    if let Item::Fn(inner_fn) = inner_item {
                        process_block(&mut inner_fn.block);
                    }
                }
            }
        }
        _ => {
            // TODO: extend?
        }
    }

    TokenStream::from(quote! { #item })
}