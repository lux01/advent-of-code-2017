use super::Day;

use std::collections::HashSet;

/// [Day 06](https://adventofcode.com/2017/day/6) - Given a list of bins of various sizes, how many
/// reallocations need to be performed to reach a previously seen state.
pub struct Day06 {
    initial_state: Vec<u8>,
}

impl Day06 {
    fn find_most_full_bucket(buckets: &[u8]) -> (usize, u8) {
        let (index, &val) = buckets
            .iter()
            .enumerate()
            .max_by(|&(i, val1), &(j, val2)| val1.cmp(&val2).then(j.cmp(&i)))
            .unwrap();
        (index, val)
    }

    fn redistribute(buckets: &[u8]) -> Vec<u8> {
        let buckets_len = buckets.len();
        let (mut index, bucket_size) = Day06::find_most_full_bucket(buckets);
        let mut output = buckets.to_owned();

        output[index] = 0;
        index = (index + 1) % buckets_len;

        for _ in 0..bucket_size {
            output[index] += 1;
            index = (index + 1) % buckets_len;
        }

        output
    }

    fn find_fixed_point(initial_state: &[u8]) -> (Vec<u8>, usize) {
        let mut previous_states = HashSet::new();
        let mut state = initial_state.to_owned();

        while previous_states.insert(state.clone()) {
            state = Day06::redistribute(&state);
        }

        (state, previous_states.len())
    }
}

impl<'a> Day<'a> for Day06 {
    const NUM: u32 = 6;
    type Output1 = usize;
    type Output2 = usize;

    fn from_str(input: &str) -> Day06 {
        let initial_state = input
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();

        Day06 { initial_state }
    }

    fn part_1(&self) -> usize {
        Day06::find_fixed_point(&self.initial_state).1
    }

    fn part_2(&self) -> usize {
        let fixed_point = Day06::find_fixed_point(&self.initial_state).0;
        Day06::find_fixed_point(&fixed_point).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_index_of_most_full_bucket() {
        let samples = [
            ([0, 2, 7, 0], (2, 7)),
            ([2, 4, 1, 2], (1, 4)),
            ([3, 1, 2, 3], (0, 3)),
            ([0, 2, 3, 4], (3, 4)),
            ([1, 3, 4, 1], (2, 4)),
        ];

        for &(input, output) in samples.iter() {
            assert_eq!(output, Day06::find_most_full_bucket(&input));
        }
    }

    #[test]
    fn redistributes_correctly() {
        let samples = [
            (vec![0, 2, 7, 0], vec![2, 4, 1, 2]),
            (vec![2, 4, 1, 2], vec![3, 1, 2, 3]),
            (vec![3, 1, 2, 3], vec![0, 2, 3, 4]),
            (vec![0, 2, 3, 4], vec![1, 3, 4, 1]),
            (vec![1, 3, 4, 1], vec![2, 4, 1, 2]),
        ];

        for &(ref input, ref output) in samples.iter() {
            assert_eq!(*output, Day06::redistribute(&input[..]));
        }
    }

    #[test]
    fn parse_input() {
        let input = "0  2    7 0";
        let output = vec![0, 2, 7, 0];

        assert_eq!(output, Day06::from_str(&input).initial_state);
    }

    #[test]
    fn part_1_test() {
        let input = "0 2 7 0";
        let output = 5;

        assert_eq!(output, Day06::from_str(&input).part_1());
    }

    #[test]
    fn find_fixed_point() {
        let input = vec![0, 2, 7, 0];
        let output = vec![2, 4, 1, 2];

        assert_eq!(output, Day06::find_fixed_point(&input).0);
    }

    #[test]
    fn part_2_test() {
        let input = "0  2 7  0";
        let output = 4;

        assert_eq!(output, Day06::from_str(&input).part_2());
    }
}