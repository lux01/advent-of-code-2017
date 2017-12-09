use super::Day;

struct JumpIncrementIter {
    pc: usize,
    len: usize,
    offsets: Vec<isize>,
    increment_fn: fn(isize) -> isize,
}

impl JumpIncrementIter {
    pub fn new(offsets: Vec<isize>, increment_fn: fn(isize) -> isize) -> JumpIncrementIter {
        JumpIncrementIter {
            pc: 0,
            len: offsets.len(),
            offsets,
            increment_fn,
        }
    }
}

impl Iterator for JumpIncrementIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let ref mut offset = self.offsets[self.pc];
        let next_pc = self.pc as isize + *offset;
        if next_pc < self.len as isize && next_pc >= 0 {
            *offset = (self.increment_fn)(*offset);
            self.pc = next_pc as usize;
            Some(())
        } else {
            None
        }
    }
}

/// [Day 05](https://adventofcode.com/2017/day/5) - Calculate how many steps are needed to exit
/// a series of consecutive jump instructions
pub struct Day05 {
    offsets: Vec<isize>,
}

impl Day for Day05 {
    const NUM: u32 = 5;
    type Output = usize;

    fn from_str(input: &str) -> Day05 {
        let offsets = input.lines().map(|line| line.parse().unwrap()).collect();

        Day05 { offsets }
    }

    fn part_1(&self) -> usize {
        1 + JumpIncrementIter::new(self.offsets.clone(), |offset| offset + 1).count()
    }

    fn part_2(&self) -> usize {
        1 +
            JumpIncrementIter::new(self.offsets.clone(), |offset| if offset >= 3 {
                offset - 1
            } else {
                offset + 1
            }).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0
3
0
1
-3";


    #[test]
    fn part_1() {
        assert_eq!(5, Day05::from_str(TEST_INPUT).part_1());
    }

    #[test]
    fn part_2() {
        assert_eq!(10, Day05::from_str(TEST_INPUT).part_2());
    }
}