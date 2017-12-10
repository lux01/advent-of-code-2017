#![warn(missing_docs)]

//! # Advent of Code 2017
//!
//! This crate contains my solutions for the [Advent of Code 2017](https://adventofcode.com/2017) challenges.
//! Each solution is separated into a different [Day](trait.Day.html). Use `cargo test` to run each implementation against
//! the provided sample input, and use `cargo run` to run all the tests.

#[macro_use]
extern crate nom;

mod day;

macro_rules! use_days {
    ($($day:ident),+)=> ( $(mod $day; pub use $day::*; )+)
}
pub mod util;

pub use util::file_as_string;
pub use day::Day;

use_days!(
    day_01,
    day_02,
    day_03,
    day_04,
    day_05,
    day_06,
    day_07,
    day_08
);
