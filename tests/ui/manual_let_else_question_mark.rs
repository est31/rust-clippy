//@run-rustfix
#![allow(unused_braces, unused_variables, dead_code)]
#![allow(
    clippy::collapsible_else_if,
    clippy::unused_unit,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::never_loop
)]
#![warn(clippy::manual_let_else, clippy::question_mark)]

enum Variant {
    A(usize, usize),
    B(usize),
    C,
}

fn g() -> Option<(u8, u8)> {
    None
}

fn e() -> Variant {
    Variant::A(0, 0)
}

fn main() {}

fn foo() -> Option<()> {
    // Fire here, normal case
    let Some(v) = g() else { return None };

    // Don't fire here, the pattern is refutable
    let Variant::A(v, w) = e() else { return None };

    // Fire here, the pattern is irrefutable
    let Some((v, w)) = g() else { return None };

    // Don't fire manual_let_else in this instance: question mark can be used instead.
    let v = if let Some(v_some) = g() { v_some } else { return None };

    Some(())
}
