//! Parsers for nodes.
//!
//! The entry point for this module is the [`node`] parser. Each variant of the [`Node`] type has a
//! submodule here, with the exception of `Literal` and `Name` whose parsers are simply wrappers
//! over the [`literal`] and [`name`] parsers respectively.
//!
//! The [`binary_op`] module is particularly important here so it is a good idea to check those
//! module docs too.
mod binary_op;
mod call;
mod cond;
mod fn_def;
mod let_bind;
mod unary_op;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1},
    combinator::map,
    sequence::pair,
};

use crate::{
    ast::{Located, Node},
    parser::{
        helpers::{in_brackets, lookahead},
        literal::literal,
        name::name,
        primitive::primitive,
        un_op::un_op,
        IResult, Span,
    },
};

/// Parser for [`Node`]s.
///
/// To understand its behaviour please refer to the [`binary_op`] docs.
pub fn node(input: Span) -> IResult<Located<Node>> {
    binary_op::binary_op(input)
}

/// Parser for base nodes and nodes inside brackets.
///
/// A base node is every node that is not a binary operation, i.e., all the variants of the
/// [`Node`] type are base nodes with the exception of the [`Node::BinaryOp`] variant.
///
/// For nodes inside brackets, there can be any number of spaces between the brackets and the node.
///
/// This parser also does small lookaheads using the [`lookahead`] combinator. This improves
/// significantly the error messages generated by nom. The lookaheads are the following:
///
/// - If the input starts with `if` and a space or line break, the [`cond`] parser is applied.
/// - If the input starts with `fn` and a space, the [`fn_def`] parser is applied.
/// - If the input starts with a name, the [`let_bind`], [`call`] or [`name`] parser is applied.
/// - If the input starts with a unary operator, the [`un_op`] parser is applied.
///
/// This function is very order sensitive. Be careful if you swap the parsers order.
fn base_node(input: Span) -> IResult<Located<Node>> {
    alt((
        lookahead(
            char('('),
            alt((
                call::call,
                map(in_brackets(node), |Located { mut content, loc }| {
                    content.loc = loc;
                    content
                }),
            )),
        ),
        map(literal, |Located { content, loc }| {
            Located::new(Node::Literal(content), loc)
        }),
        lookahead(pair(tag("if"), multispace1), cond::cond),
        lookahead(tag("fn"), fn_def::fn_def),
        lookahead(primitive, call::call),
        lookahead(
            name,
            alt((
                let_bind::let_bind,
                call::call,
                map(name, |Located { content, loc }| {
                    Located::new(Node::Name(content), loc)
                }),
            )),
        ),
        lookahead(un_op, unary_op::unary_op),
    ))(input)
}
