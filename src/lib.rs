#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod lucina;
mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    lucina::install();
}