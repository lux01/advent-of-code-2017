use std::fmt::Display;

/// A generic wrapper for each day in the Advent of Code.
pub trait Day<'a> {
    /// The day of the advent calendar that this implementation is for.
    const NUM: u32;
    type Output1: Display;
    type Output2: Display;

    /// Each challenge has an input, this constructs the challenge data
    /// from an input string.
    fn from_str(input: &'a str) -> Self;

    /// Calculate part 1 of the challenge.
    fn part_1(&self) -> Self::Output1;

    /// Calculate part 2 of the challenge.
    fn part_2(&self) -> Self::Output2;

    /// Run both parts of the challenge in order, printing the results.
    fn run(&self) {
        println!("Day {}", Self::NUM);
        println!("========================================");
        println!("Part 1: {}", self.part_1());
        println!("Part 2: {}", self.part_2());
        println!("");
    }
}