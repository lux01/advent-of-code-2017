use super::Day;

mod dancers;

use self::dancers::{DanceMove, Dancers};

pub struct Day16<'a> {
    input: &'a str,
}

impl<'a> Day<'a> for Day16<'a> {
    const NUM: u32 = 16;
    type Output1 = Dancers;
    type Output2 = Dancers;

    fn from_str(input: &'a str) -> Self {
        Day16 { input }
    }

    fn part_1(&self) -> Self::Output1 {
        let mut dancers = Dancers::new('p');
        dancers.dance_record(self.input);
        dancers
    }

    fn part_2(&self) -> Self::Output2 {
        let mut dancers = Dancers::new('p');
        let starting_dancers = Dancers::new('p');
        let dance_moves = DanceMove::parse_many(self.input).unwrap().1;
        let mut loop_size = 0;

        while ((dancers != starting_dancers && loop_size != 0) || loop_size == 0)
            && loop_size < 1_000_000_000_u64
        {
            dancers.dance_record(self.input);
            loop_size += 1;
        }

        if loop_size < 1_000_000_000_u64 {
            let remaning_iters = 1_000_000_000_u64 - loop_size;
            let meaningful_iters = remaning_iters % loop_size;

            for _ in 0..meaningful_iters {
                dancers.dance_all(&dance_moves);
            }

            dancers
        } else {
            dancers
        }
    }
}
