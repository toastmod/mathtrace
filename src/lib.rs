use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, Expr, ExprBinary, ExprCall, ExprLit, ItemFn, Lit, Pat, Stmt, parse_macro_input, parse_quote};

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
            quote! { format!("{} = {:?}", stringify!(#call_expr), #call_expr) }
        }
        Expr::Lit(ExprLit { lit: Lit::Int(_) | Lit::Float(_), .. }) => {
            quote! { format!("{:?}", #expr) }
        }
        Expr::Path(_) => {
            quote! { format!("{:?}", #expr) }
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
        match stmt {
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    let name = &local.pat;
                    let expr = &init.expr;

                    let etrace = trace_expr_rec(expr);
                    let trace_stmt: Stmt = parse_quote! {
                        println!("{} = {}", stringify!(#name), #etrace);
                    };

                    new_stmts.push(trace_stmt);
                }
                new_stmts.push(stmt.clone());
            }

            Stmt::Expr(expr, _) => {
                match expr {
                    Expr::Binary(_) | Expr::Call(_) => {
                        let etrace = trace_expr_rec(&expr);
                        let trace_stmt: Stmt = parse_quote! {
                            println!("{}", #etrace);
                        };
                        new_stmts.push(trace_stmt);
                    }
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
    let mut func = parse_macro_input!(input as ItemFn);

    process_block(&mut func.block);

    TokenStream::from(quote! { #func })
}
