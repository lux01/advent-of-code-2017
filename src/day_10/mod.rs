use super::Day;
use super::util::knot_hash::{KnotHash, KnotHasher};

pub struct Day10<'a> {
    input: &'a str,
}

impl<'a> Day10<'a> {
    fn as_nums(input: &str) -> Vec<usize> {
        input
            .trim()
            .split(",")
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    }
}

impl<'a> Day<'a> for Day10<'a> {
    const NUM: u32 = 10;
    type Output1 = u32;
    type Output2 = String;

    fn from_str(input: &'a str) -> Self {
        Day10 { input }
    }

    fn part_1(&self) -> Self::Output1 {
        KnotHasher::new(256).check_round(&Day10::as_nums(self.input))
    }

    fn part_2(&self) -> Self::Output2 {
        KnotHash::hash_str(self.input).to_str()
    }
}
