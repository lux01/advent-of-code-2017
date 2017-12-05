pub use super::Day;

mod spiral_iter;
use self::spiral_iter::SpiralIterator;

mod cached_summed_spiral_iter;
use self::cached_summed_spiral_iter::CachedSummedSpiralIterator;

/// [Day 3](https://adventofcode.com/2017/day/3). Calculate the Manhatten distance of a number from
/// the centre of number spirals.
pub struct Day03 {
    number: i32,
}

impl Day for Day03 {
    const NUM: u32 = 3;

    fn from_str(input: &str) -> Self {
        Day03 { number: input.parse().unwrap() }
    }

    fn part_1(&self) -> isize {
        let mut iter = SpiralIterator::new().skip_while(|&(_, number)| number != self.number);

        match iter.next() {
            Some(((x, y), _)) => (x.abs() + y.abs()) as isize,
            None => unreachable!(),
        }
    }

    fn part_2(&self) -> isize {
        let mut iter =
            CachedSummedSpiralIterator::new().skip_while(|&(_, number)| number < self.number);

        match iter.next() {
            Some((_, value)) => value as isize,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1() {
        assert_eq!(0, Day03::from_str("1").part_1());
        assert_eq!(3, Day03::from_str("12").part_1());
    }

    #[test]
    pub fn part_2() {
        assert_eq!(54, Day03::from_str("50").part_2());
    }
}