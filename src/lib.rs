use proc_macro::TokenStream;
use quote::quote;

mod eval;

use eval::Evaluate;

#[derive(Debug)]
struct Initializer {
    keyw: syn::token::Do,
    func: syn::Expr,
    semi: syn::token::Semi,
    size: syn::Expr,
}

impl syn::parse::Parse for Initializer {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyw: input.parse()?,
            func: input.parse()?,
            semi: input.parse()?,
            size: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn init_with(input: TokenStream) -> TokenStream {
    let site = proc_macro2::Span::call_site();

    let init = syn::parse_macro_input!(input as Initializer);

    let size = match init.size.evaluate() {
        Ok(size) => size,
        Err(e) => return e.to_compile_error().into(),
    };

    let vals: Vec<_> = (0usize..size)
        .map(|i| syn::LitInt::new(&i.to_string(), site))
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
