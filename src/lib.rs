use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Expr, ExprBinary, ExprCall, ExprLit, ItemFn, Lit, Stmt, Pat};

/// Recursively build a trace string for any expression
fn trace_expr(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Binary(bin) => {
            let left_tokens = trace_expr(&bin.left);
            let right_tokens = trace_expr(&bin.right);
            let op = &bin.op;

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
            // fallback for other expressions
            let expr_tokens = quote! { #expr };
            quote! { format!("{:?}", #expr_tokens) }
        }
    }
}

#[proc_macro_attribute]
pub fn mathtrace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let mut new_stmts = Vec::new();

    for stmt in &func.block.stmts {
        match stmt {
            // let statements
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    let expr = &init.expr;
                    let name = &local.pat;

                    // generate trace
                    let etrace = trace_expr(expr);
                    let trace_stmt = parse_quote! {
                        println!("{} = {}", stringify!(#name), #etrace);
                    };

                    new_stmts.push(trace_stmt);
                }
                new_stmts.push(stmt.clone());
            }

            // standalone expressions
            Stmt::Expr(expr, _) => {
                // Only trace binary or call expressions
                match expr {
                    Expr::Binary(_) | Expr::Call(_) => {
                        let etrace = trace_expr(expr);
                        let trace_stmt = parse_quote! {
                            println!("{}", #etrace);
                        };
                        new_stmts.push(trace_stmt);
                    }
                    _ => {}
                }

                new_stmts.push(stmt.clone());
            }

            _ => new_stmts.push(stmt.clone()),
        }
    }

    func.block.stmts = new_stmts;

    TokenStream::from(quote! { #func })
}