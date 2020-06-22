use proc_macro::TokenStream;
use quote::quote;

mod eval;

use eval::Evaluate;

#[derive(Debug)]
enum Kind {
    Const(syn::token::Const),
    Static(syn::token::Static, Option<syn::token::Mut>),
}

impl quote::ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Kind::Const(c) => c.to_tokens(tokens),
            Kind::Static(s, m) => {
                s.to_tokens(tokens);
                m.to_tokens(tokens);
            }
        }
    }
}

impl syn::parse::Parse for Kind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Const) {
            Ok(Kind::Const(input.parse()?))
        } else if lookahead.peek(syn::token::Static) {
            let kwstatic = input.parse()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::token::Mut) {
                let kwmut = input.parse()?;
                Ok(Kind::Static(kwstatic, Some(kwmut)))
            } else {
                Ok(Kind::Static(kwstatic, None))
            }
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct Left {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    kind: Kind,
    ident: syn::Ident,
    colon_token: syn::token::Colon,
    ty: syn::TypeArray,
}

impl quote::ToTokens for Left {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for a in &self.attrs {
            a.to_tokens(tokens);
        }

        self.vis.to_tokens(tokens);
        self.kind.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl syn::parse::Parse for Left {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            kind: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct Right {
    brkt: syn::token::Bracket,
    expr: syn::Expr,
    semi: syn::token::Semi,
    size: syn::Expr,
}

impl syn::parse::Parse for Right {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let bracketed = syn::bracketed!(content in input);

        Ok(Self {
            brkt: bracketed,
            expr: content.parse()?,
            semi: content.parse()?,
            size: content.parse()?,
         })
    }
}

#[derive(Debug)]
struct Full {
    left: Left,
    assn: syn::token::Eq,
    rght: Right,
    semi: syn::token::Semi,
}

impl syn::parse::Parse for Full {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            left: input.parse()?,
            assn: input.parse()?,
            rght: input.parse()?,
            semi: input.parse()?,
        })
    }
}

impl From<Full> for (Left, Right) {
    fn from(value: Full) -> Self {
        (value.left, value.rght)
    }
}

#[proc_macro_attribute]
pub fn func(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let (left, right) = syn::parse_macro_input!(input as Full).into();
    let iter = 0usize..right.size.evaluate().unwrap();
    let func = match right.expr {
        // Don't modify path statements.
        syn::Expr::Path(p) => quote! { #p },

        // Closures aren't const, so we convert a closure into a const fn.
        syn::Expr::Closure(c) => {
            let name = c.inputs.first();
            let retv = &left.ty.elem;
            let body = &c.body;

            quote! {
                {
                    const fn a6275f0683648f565baa29cc3b98e89414ac333b(#name: usize) -> #retv {
                        #body
                    }

                    a6275f0683648f565baa29cc3b98e89414ac333b
                }
            }
        }

        e => panic!("Unsupported expression: {:?}", e),
    };

    TokenStream::from(quote! {
        #left = [ #( #func(#iter) ),* ];
    })
}
