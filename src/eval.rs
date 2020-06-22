use std::ops::*;

trait Errorizable: syn::spanned::Spanned {
    fn errorize<T: std::fmt::Display, U>(&self, msg: T) -> syn::Result<U> {
        Err(syn::Error::new(self.span(), msg))
    }
}

impl<T: syn::spanned::Spanned> Errorizable for T {}

pub trait Evaluate<T> {
    fn evaluate(&self) -> syn::Result<T>;
}

pub trait Integer:
    std::str::FromStr<Err = std::num::ParseIntError>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
}

impl<T> Integer for T
where
    T: std::str::FromStr<Err = std::num::ParseIntError>,
    T: Add<Self, Output = Self>,
    T: Sub<Self, Output = Self>,
    T: Mul<Self, Output = Self>,
    T: Div<Self, Output = Self>,
    T: Rem<Self, Output = Self>,
    T: BitXor<Self, Output = Self>,
    T: BitAnd<Self, Output = Self>,
    T: BitOr<Self, Output = Self>,
    T: Shl<Self, Output = Self>,
    T: Shr<Self, Output = Self>,
{
}

impl<T: Integer> Evaluate<T> for syn::LitInt {
    fn evaluate(&self) -> syn::Result<T> {
        self.base10_parse()
    }
}

impl<T: Integer> Evaluate<T> for syn::Lit {
    fn evaluate(&self) -> syn::Result<T> {
        match self {
            Self::Int(i) => i.evaluate(),
            _ => self.errorize("Unable to evaluate!"),
        }
    }
}

impl<T: Integer> Evaluate<T> for syn::ExprLit {
    fn evaluate(&self) -> syn::Result<T> {
        self.lit.evaluate()
    }
}

impl<T: Integer> Evaluate<T> for syn::ExprBinary {
    fn evaluate(&self) -> syn::Result<T> {
        let l: T = self.left.evaluate()?;
        let r: T = self.right.evaluate()?;

        Ok(match self.op {
            syn::BinOp::Add(_) => l + r,
            syn::BinOp::Sub(_) => l - r,
            syn::BinOp::Mul(_) => l * r,
            syn::BinOp::Div(_) => l / r,
            syn::BinOp::Rem(_) => l % r,
            syn::BinOp::BitXor(_) => l ^ r,
            syn::BinOp::BitAnd(_) => l & r,
            syn::BinOp::BitOr(_) => l | r,
            syn::BinOp::Shl(_) => l << r,
            syn::BinOp::Shr(_) => l >> r,
            _ => self.errorize("Unable to evaluate!")?,
        })
    }
}

impl<T: Integer> Evaluate<T> for syn::Expr {
    fn evaluate(&self) -> syn::Result<T> {
        match self {
            Self::Lit(l) => l.evaluate(),
            Self::Binary(b) => b.evaluate(),
            _ => self.errorize("Unable to evaluate!"),
        }
    }
}
