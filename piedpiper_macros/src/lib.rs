mod punct;

use crate::punct::Pipe;
use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Peek},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprCall, ExprInfer, Result, Token,
};

#[proc_macro]
pub fn pp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PathSegments);

    match pp_macro(input) {
        Ok(s) => s.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn pp_macro(PathSegments { segments }: PathSegments) -> Result<TokenStream2> {
    let mut iter = segments.iter().map(|x| x.clone());
    match iter.next() {
        Some(expr) => {
            let result = pipe(expr, &mut iter);

            Ok(quote! { #result })
        }
        None => {
            // empty iterator do nothing
            return Ok(quote! {});
        }
    }
}

fn pipe(prev: Expr, iter: &mut dyn Iterator<Item = Expr>) -> Expr {
    match iter.next() {
        Some(expr) => match expr {
            Expr::Call(ExprCall {
                func,
                args,
                attrs,
                paren_token,
            }) => {
                let args = args
                    .iter()
                    .map(|expr| match expr {
                        Expr::Infer(ExprInfer { .. }) => prev.clone(),
                        _ => expr.clone(),
                    })
                    .collect::<Punctuated<Expr, Token![,]>>();

                let expr = Expr::Call(ExprCall {
                    attrs,
                    func,
                    paren_token,
                    args,
                });

                pipe(expr, iter)
            }
            _ => expr,
        },
        None => prev,
    }
}

struct PathSegments {
    segments: Punctuated<Expr, Pipe>,
}

impl Parse for PathSegments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Punctuated::new();

        let first = parse_until(input, Pipe)?;
        segments.push_value(syn::parse2(first.into())?);

        while input.peek(Pipe) {
            segments.push_punct(input.parse()?);

            let next = parse_until(input, Pipe)?;
            segments.push_value(syn::parse2(next.into())?);
        }

        Ok(PathSegments { segments })
    }
}

fn parse_until<E: Peek>(input: ParseStream, end: E) -> Result<TokenStream2> {
    let mut tokens = TokenStream2::new();
    while !input.is_empty() && !input.peek(end) {
        let next: TokenTree = input.parse()?;
        tokens.extend(Some(next));
    }
    Ok(tokens)
}
