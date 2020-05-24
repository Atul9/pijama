use std::mem::discriminant;

use thiserror::Error;

use crate::{
    ast::{
        analysis::RecursionChecker, BinOp, Block, Branch, Literal, Located, Location, Name, Node,
        UnOp,
    },
    mir::{LetKind, Term},
    ty::{Binding, Ty},
};

pub type LowerResult<T> = Result<T, LowerError>;

#[derive(Error, Debug)]
pub enum LowerError {
    #[error("Recursive functions need a return type annotation")]
    RecWithoutTy(Location),
    #[error("Anonymous functions cannot have a return type annotation")]
    AnonWithTy(Location),
}

impl LowerError {
    pub fn loc(&self) -> Location {
        match self {
            LowerError::RecWithoutTy(loc) | LowerError::AnonWithTy(loc) => *loc,
        }
    }
}

impl PartialEq for LowerError {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for LowerError {}

pub fn lower_blk<'a>(blk: Located<Block<'a>>) -> LowerResult<Located<Term<'a>>> {
    let mut terms = blk.content.into_iter().rev().map(lower_node);
    if let Some(term) = terms.next() {
        let mut term = term?;
        for prev_term in terms {
            let prev_term = prev_term?;
            let next_term = Box::new(term);

            let loc = prev_term.loc;
            let content = if let Term::Let(kind, name, value, _) = prev_term.content {
                Term::Let(kind, name, value, next_term)
            } else {
                Term::Seq(Box::new(prev_term), next_term)
            };
            term = Located::new(content, loc);
        }
        Ok(term)
    } else {
        Ok(Located::new(Term::Lit(Literal::Unit), blk.loc))
    }
}

fn lower_node(node: Located<Node<'_>>) -> LowerResult<Located<Term<'_>>> {
    let loc = node.loc;
    match node.content {
        Node::Name(name) => Ok(Located::new(Term::Var(name), loc)),
        Node::Cond(if_branch, branches, el_blk) => lower_cond(loc, if_branch, branches, el_blk),
        Node::Literal(lit) => Ok(Located::new(Term::Lit(lit), loc)),
        Node::Call(node, args) => lower_call(loc, *node, args),
        Node::BinaryOp(bin_op, node1, node2) => lower_binary_op(loc, bin_op, *node1, *node2),
        Node::UnaryOp(un_op, node) => lower_unary_op(loc, un_op, *node),
        Node::LetBind(name, opt_ty, node) => lower_let_bind(loc, name, opt_ty, *node),
        Node::FnDef(opt_name, binds, body, opt_ty) => {
            lower_fn_def(loc, opt_name, binds, body, opt_ty)
        }
        Node::PrimFn(prim) => Ok(Located::new(Term::PrimFn(prim), loc)),
    }
}

fn lower_cond<'a>(
    loc: Location,
    if_branch: Branch<'a>,
    branches: Vec<Branch<'a>>,
    el_blk: Located<Block<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    let mut el_term = Box::new(lower_blk(el_blk)?);

    for branch in branches.into_iter().rev() {
        el_term = Box::new(Located::new(
            Term::Cond(
                Box::new(lower_blk(branch.cond)?),
                Box::new(lower_blk(branch.body)?),
                el_term,
            ),
            loc,
        ));
    }

    let if_blk = if_branch.cond;
    let do_blk = if_branch.body;

    Ok(Located::new(
        Term::Cond(
            Box::new(lower_blk(if_blk)?),
            Box::new(lower_blk(do_blk)?),
            el_term,
        ),
        loc,
    ))
}

fn lower_call<'a>(
    loc: Location,
    node: Located<Node<'a>>,
    args: Block<'a>,
) -> LowerResult<Located<Term<'a>>> {
    let mut term = lower_node(node)?;
    for node in args {
        term = Located::new(Term::App(Box::new(term), Box::new(lower_node(node)?)), loc);
    }
    Ok(term)
}

fn lower_binary_op<'a>(
    loc: Location,
    bin_op: BinOp,
    node1: Located<Node<'a>>,
    node2: Located<Node<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    Ok(Located::new(
        Term::BinaryOp(
            bin_op,
            Box::new(lower_node(node1)?),
            Box::new(lower_node(node2)?),
        ),
        loc,
    ))
}

fn lower_unary_op(
    loc: Location,
    un_op: UnOp,
    node: Located<Node<'_>>,
) -> LowerResult<Located<Term<'_>>> {
    Ok(Located::new(
        Term::UnaryOp(un_op, Box::new(lower_node(node)?)),
        loc,
    ))
}

fn lower_let_bind<'a>(
    loc: Location,
    name: Located<Name<'a>>,
    opt_ty: Option<Located<Ty>>,
    node: Located<Node<'a>>,
) -> LowerResult<Located<Term<'a>>> {
    let term = lower_node(node)?;

    Ok(Located::new(
        Term::Let(
            LetKind::NonRec(opt_ty),
            name,
            Box::new(term),
            Box::new(Located::new(
                Term::Lit(Literal::Unit),
                Location::new(loc.end, loc.end),
            )),
        ),
        loc,
    ))
}

fn lower_fn_def<'a>(
    loc: Location,
    opt_name: Option<Located<Name<'a>>>,
    binds: Vec<Located<Binding<'a>>>,
    body: Located<Block<'a>>,
    opt_ty: Option<Located<Ty>>,
) -> LowerResult<Located<Term<'a>>> {
    // if the user added a return type annotation, we transform this type into the type of the
    // function using the bindings.
    let opt_ty = opt_ty.map(|located_ty| {
        let mut ty = located_ty.content;
        let ty_loc = located_ty.loc;

        for bind in binds.iter().rev() {
            ty = Ty::Arrow(Box::new(bind.content.ty.clone()), Box::new(ty));
        }

        Located::new(ty, ty_loc)
    });

    // we need to decide if the function is recursive or not
    let kind = match opt_name.as_ref() {
        // functions can only be recursive if they have a name.
        Some(name) if RecursionChecker::run(name.content, &body.content) => {
            // if the function is recursive, we need the return type.
            opt_ty
                .map(LetKind::Rec)
                .ok_or_else(|| LowerError::RecWithoutTy(name.loc))?
        }
        // anonymous functions cannot have type annotations
        None if opt_ty.is_some() => {
            return Err(LowerError::AnonWithTy(opt_ty.unwrap().loc));
        }
        // otherwise the function is either anonymous without a type anotation or a named
        // non-recursive function with or without a type annotation.
        _ => LetKind::NonRec(opt_ty),
    };

    let mut term = lower_blk(body)?;

    for bind in binds.into_iter().rev() {
        term = Located::new(Term::Abs(bind.content, Box::new(term)), loc);
    }

    if let Some(name) = opt_name {
        term = Located::new(
            Term::Let(
                kind,
                name,
                Box::new(term),
                Box::new(Located::new(
                    Term::Lit(Literal::Unit),
                    Location::new(loc.end, loc.end),
                )),
            ),
            loc,
        );
    }

    Ok(term)
}
