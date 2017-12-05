# Advent of Code 2017

My solutions to the [Advent of Code 2017](https://adventofcode.com/2017) written using Rust.

The solutions for all the days can be calculated by using `cargo run --release`. In the future I may add command line flags to specify which days to run. This is currently fast enough for all practical purposes.

Each day also has a number of accompanying tests that validate the implementation against the samples from the day problem summary. You can run these using `cargo test`.

All the solution code for each day can be found in the corresponding `src/day_xx` folder with its sample input in `input/day_xx.txt`. Each day is implemented as a struct that implements the [`Day`](src/day.rs) trait which is a wrapper trait used by the `main.rs` binary to make the problem execution standard.