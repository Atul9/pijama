use std::{include_str, time::Duration};

use pijama_ast::Literal;
use pijama_core::{lir::Term, machine::env::Env};
use pijama_driver::{run_with_machine, LangError, LangResult};

use crate::{machine_builder, panic_after, run};

#[test]
fn arithmetic() -> LangResult<'static, ()> {
    let input = include_str!("arithmetic.pj");
    let output = run(input)?;
    assert_eq!("121\n", output);
    Ok(())
}

#[test]
fn logic() -> LangResult<'static, ()> {
    let input = include_str!("logic.pj");
    let output = run(input)?;
    assert_eq!("0\n", output);
    Ok(())
}

#[test]
fn factorial() -> LangResult<'static, ()> {
    let input = include_str!("factorial.pj");
    let output = run(input)?;
    assert_eq!("3628800\n", output);
    Ok(())
}

#[test]
fn factorial_tail() -> LangResult<'static, ()> {
    let input = include_str!("factorial_tail.pj");
    let output = run(input)?;
    assert_eq!("3628800\n", output);
    Ok(())
}

#[test]
fn fancy_max() -> LangResult<'static, ()> {
    let input = include_str!("fancy_max.pj");
    let output = run(input)?;
    assert_eq!("10\n", output);
    Ok(())
}

#[test]
fn fibonacci() -> LangResult<'static, ()> {
    let input = include_str!("fibonacci.pj");
    let output = run(input)?;
    assert_eq!("21\n", output);
    Ok(())
}

#[test]
fn fibonacci_tail() -> LangResult<'static, ()> {
    let input = include_str!("fibonacci_tail.pj");
    let output = run(input)?;
    assert_eq!("21\n", output);
    Ok(())
}

#[test]
fn gcd() -> LangResult<'static, ()> {
    let input = include_str!("gcd.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn ackermann() -> LangResult<'static, ()> {
    let input = include_str!("ackermann.pj");
    let output = run(input)?;
    assert_eq!("5\n", output);
    Ok(())
}

#[test]
fn calling() -> LangResult<'static, ()> {
    let input = include_str!("calling.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn complex_calling() -> LangResult<'static, ()> {
    let input = include_str!("complex_calling.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn step() -> LangResult<'static, ()> {
    let input = include_str!("step.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn bit_and() -> LangResult<'static, ()> {
    let input = include_str!("bit_and.pj");
    let output = run(input)?;
    assert_eq!("64\n", output);
    Ok(())
}

#[test]
fn bit_or() -> LangResult<'static, ()> {
    let input = include_str!("bit_or.pj");
    let output = run(input)?;
    assert_eq!("192\n", output);
    Ok(())
}

#[test]
fn bit_xor() -> LangResult<'static, ()> {
    let input = include_str!("bit_xor.pj");
    let output = run(input)?;
    assert_eq!("128\n", output);
    Ok(())
}

#[test]
fn bit_shift_l() -> LangResult<'static, ()> {
    let input = include_str!("bit_shift_l.pj");
    let output = run(input)?;
    assert_eq!("128\n", output);
    Ok(())
}

#[test]
fn bit_shift_r() -> LangResult<'static, ()> {
    let input = include_str!("bit_shift_r.pj");
    let output = run(input)?;
    assert_eq!("32\n", output);
    Ok(())
}

#[test]
fn or_short_circuit() -> LangResult<'static, ()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("or_short_circuit.pj");
        let output = run(input)?;
        assert_eq!("1\n", output);
        Ok(())
    })
}

#[test]
fn and_short_circuit() -> LangResult<'static, ()> {
    panic_after(Duration::from_secs(1), || {
        let input = include_str!("and_short_circuit.pj");
        let output = run(input)?;
        assert_eq!("0\n", output);
        Ok(())
    })
}

#[test]
fn print_simple() -> LangResult<'static, ()> {
    let input = include_str!("print_simple.pj");
    let output = run(input)?;
    assert_eq!("10\n", output);
    Ok(())
}

#[test]
fn print_simple_fn() -> LangResult<'static, ()> {
    let input = include_str!("print_simple_fn.pj");
    let output = run(input)?;
    assert_eq!("(λ. _0)\n", output);
    Ok(())
}

#[test]
fn print_complex_fn() -> LangResult<'static, ()> {
    let input = include_str!("print_complex_fn.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn print_print() -> LangResult<'static, ()> {
    let input = include_str!("print_print.pj");
    let output = run(input)?;
    assert_eq!("10\n0\n", output);
    Ok(())
}

#[test]
fn print_redefine() {
    let input = include_str!("print_redefine.pj");
    let err = run(input).unwrap_err();
    assert!(matches!(err, LangError::Parse(_)))
}

#[test]
fn number_bases_cmp() -> LangResult<'static, ()> {
    let input = include_str!("number_bases_cmp.pj");
    let output = run(input)?;
    assert_eq!("1\n", output);
    Ok(())
}

#[test]
fn number_bases_arithmetic() -> LangResult<'static, ()> {
    let input = include_str!("number_bases_arithmetic.pj");
    let output = run(input)?;
    assert_eq!("2271532\n", output);
    Ok(())
}

#[test]
#[should_panic]
fn add_overflow_panics() {
    let input = include_str!("add_overflow_panics.pj");
    run(input).ok();
}

#[test]
#[should_panic]
fn neg_overflow_panics() {
    let input = include_str!("neg_overflow_panics.pj");
    run(input).ok();
}
