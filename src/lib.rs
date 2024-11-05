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

#[allow(unused_imports)]
use crate::prelude::*;

#[must_use] pub const fn add(left: usize, right: usize) -> usize {
    left + right
}

#[must_use] pub const fn sub_with_panic(left: usize, right: usize) -> usize {
    assert!(left >= right, "Left is less than right");
    left - right
}

// #[cfg(test)] omitted because I want linting
mod tests {
    #[allow(clippy::wildcard_imports)]
    use rstest::*;

    #[fixture]
    fn setup() {
        // #[allow(clippy::unwrap_used)]
        // color_eyre::install().unwrap();
    }

    #[rstest]
    fn test_add(setup: ()) {
        assert_eq!(super::add(1, 2), 3);
    }

    #[rstest]
    fn test_sub_with_panic_1(setup: ()) {
        assert_eq!(super::sub_with_panic(2, 1), 1);
    }

    #[rstest]
    #[should_panic(expected = "Left is less than right")]
    fn test_sub_with_panic_2(setup: ()) {
        super::sub_with_panic(1, 2);
    }
}