use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Expr, ExprBinary, ExprLit, ItemFn, Lit, Stmt};

#[proc_macro_attribute]
pub fn mathtrace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    let mut new_stmts = Vec::new();

    for stmt in &func.block.stmts {
        match stmt {
            // handle let statements
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    match init.expr.as_ref() {
                        Expr::Binary(bin) => {
                            let name = &local.pat;
                            let left = &bin.left;
                            let right = &bin.right;
                            let op = &bin.op;

                            new_stmts.push(parse_quote! {
                                println!(
                                    "{} = {} {} {} = ({:?}) {} ({:?})",
                                    stringify!(#name),
                                    stringify!(#left),
                                    stringify!(#op),
                                    stringify!(#right),
                                    #left,
                                    stringify!(#op),
                                    #right
                                );
                            });
                        }
                        Expr::Lit(ExprLit { lit: Lit::Int(_), .. }) |
                        Expr::Lit(ExprLit { lit: Lit::Float(_), .. }) => {
                            let name = &local.pat;
                            let val = &init.expr;

                            new_stmts.push(parse_quote! {
                                println!("{} = {:?}", stringify!(#name), #val);
                            });
                        }
                        _ => {}
                    }
                }

                new_stmts.push(stmt.clone());
            }

            // handle standalone expressions
            Stmt::Expr(expr, _) => {
                match expr {
                    Expr::Binary(bin) => {
                        let left = &bin.left;
                        let right = &bin.right;
                        let op = &bin.op;

                        new_stmts.push(parse_quote! {
                            println!(
                                "{} {} {} = ({:?}) {} ({:?})",
                                stringify!(#left),
                                stringify!(#op),
                                stringify!(#right),
                                #left,
                                stringify!(#op),
                                #right
                            );
                        });
                    }
                    Expr::Lit(ExprLit { lit: Lit::Int(_) | Lit::Float(_), .. }) => {
                        new_stmts.push(parse_quote! {
                            println!("= {:?}", #expr);
                        });
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
