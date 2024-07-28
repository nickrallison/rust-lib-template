#![deny(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod error;
mod prelude;

#[allow(unused_imports)]
use crate::prelude::*;

fn main() -> Result<()> {
    let a: u32 = 7;
    let b: u32 = 3;
    let res: u32 = a % b;
    println!("{a} % {b} = {res}");
    Ok(())
}
