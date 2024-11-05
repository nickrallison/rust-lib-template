#![feature(const_for)]
#![feature(const_fn_floating_point_arithmetic)]

#![allow(clippy::items_after_statements)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::no_effect)]
#![allow(unused_must_use)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::never_loop)]
#![allow(clippy::eq_op)]

mod error;
mod prelude;

use crate::prelude::*;

#[must_use] pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)] omitted because I want linting
mod tests {
    #[allow(clippy::wildcard_imports)]
    use rstest::*;

    #[fixture]
    fn setup() {
        #[allow(clippy::unwrap_used)]
        color_eyre::install().unwrap();
    }

    #[rstest]
    fn test_add(setup: ()) {
        assert_eq!(super::add(1, 2), 3);
    }
}