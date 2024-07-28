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

#[allow(unused_imports)]
use log::{info, trace, warn};

fn rem_3(input_num: u32) -> u32 {
    trace!("Called template::rem_3({input_num})");
    let result: u32 = input_num % 3;
    trace!("template::rem_3 calculated result = {result}");
    return result;
}

fn main() -> Result<()> {
    let a: u32 = 7;
    let res: u32 = rem_3(a);
    if res > 3 {
        panic!("Something Bad Happened");
    }
    println!("{a} % 3 = {res}");
    Ok(())
}
