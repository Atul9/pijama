//! Pijama's types.
//!
//! This module exposes the `Ty` type which is the type representation used by the
//! type-checker.
use std::fmt;

use pijama_ast::ty::Ty as TyAST;

/// A type used by the type-checker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    /// The type of booleans.
    Bool,
    /// The type of (signed) integers.
    Int,
    /// The [unit type](https://en.wikipedia.org/wiki/Unit_type).
    Unit,
    /// The type of functions between two types.
    Arrow(Box<Ty>, Box<Ty>),
    /// Type variable, used for unification.
    Var(usize),
}

impl Ty {
    /// Checks if the index of a `Ty::Var` is contained inside the type.
    pub fn contains(&self, index: usize) -> bool {
        match self {
            Ty::Bool | Ty::Int | Ty::Unit => false,
            Ty::Arrow(ty1, ty2) => ty1.contains(index) || ty2.contains(index),
            Ty::Var(inner) => *inner == index,
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Ty::*;
        match self {
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Unit => write!(f, "Unit"),
            Arrow(t1, t2) => {
                if let Arrow(_, _) = t1.as_ref() {
                    write!(f, "({}) -> {}", t1, t2)
                } else {
                    write!(f, "{} -> {}", t1, t2)
                }
            }
            Var(index) => write!(f, "?X{}", index),
        }
    }
}

impl Ty {
    pub fn from_ast(ty_ast: TyAST) -> Option<Self> {
        match ty_ast {
            // FIXME: In general we should translate missing types into type variables inside
            // `ty_check::Context`.
            TyAST::Missing => None,
            TyAST::Bool => Some(Ty::Bool),
            TyAST::Int => Some(Ty::Int),
            TyAST::Unit => Some(Ty::Unit),
            TyAST::Arrow(t1, t2) => Some(Ty::Arrow(
                Box::new(Ty::from_ast(*t1)?),
                Box::new(Ty::from_ast(*t2)?),
            )),
        }
    }
}
