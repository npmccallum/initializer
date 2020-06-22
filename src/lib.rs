use proc_macro::TokenStream;
use quote::quote;

mod eval;

use eval::Evaluate;

#[derive(Debug)]
struct Initializer {
    brkt: syn::token::Bracket,
    func: syn::Expr,
    semi: syn::token::Semi,
    size: syn::Expr,
}

impl syn::parse::Parse for Initializer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let bracketed = syn::bracketed!(content in input);

        Ok(Self {
            brkt: bracketed,
            func: content.parse()?,
            semi: content.parse()?,
            size: content.parse()?,
        })
    }
}

#[proc_macro]
pub fn init_with(input: TokenStream) -> TokenStream {
    let init = syn::parse_macro_input!(input as Initializer);

    let size = match init.size.evaluate() {
        Ok(size) => size,
        Err(e) => return e.to_compile_error().into(),
    };

    let vals: Vec<_> = (0usize..size)
        .map(|i| match &init.func {
            syn::Expr::Path(p) => quote!(#p(#i)),

            syn::Expr::Closure(c) => {
                let name = c.inputs.first();
                let body = &c.body;
                quote!({ let #name = #i; #body })
            }

            e => panic!("Unsupported expression: {:?}", e),
        })
        .collect();

    quote!([ #( #vals ),* ]).into()
}
