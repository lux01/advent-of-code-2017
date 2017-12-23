use super::Day;
use super::util::parse_u64;
use std::iter::Zip;

pub struct Generator {
    previous_value: u64,
    factor: u64,
}

impl Generator {
    pub fn new_pair(seed_a: u64, seed_b: u64) -> (Generator, Generator) {
        (
            Generator {
                previous_value: seed_a,
                factor: 16807,
            },
            Generator {
                previous_value: seed_b,
                factor: 48271,
            },
        )
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = (self.previous_value * self.factor) % 2147483647;

        self.previous_value = next_value;
        Some(next_value)
    }
}

pub struct FactorGenerator<A> {
    iter: A,
    factor: u64,
}

impl<A> FactorGenerator<A> {
    pub fn new(iter: A, factor: u64) -> FactorGenerator<A> {
        FactorGenerator { iter, factor }
    }
}

impl<A> Iterator for FactorGenerator<A>
where
    A: Iterator<Item = u64>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(num) = self.iter.next() {
            if num % self.factor == 0 {
                return Some(num);
            }
        }
        None
    }
}

pub struct Judge<A, B> {
    iter: Zip<A, B>,
    count: usize,
}

impl<A, B> Judge<A, B>
where
    A: Iterator<Item = u64>,
    B: Iterator<Item = u64>,
{
    pub fn new(a: A, b: B) -> Judge<A, B> {
        Judge {
            iter: a.zip(b),
            count: 0,
        }
    }
}

impl<A, B> Iterator for Judge<A, B>
where
    A: Iterator<Item = u64>,
    B: Iterator<Item = u64>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((a, b)) => {
                if (a & 0xffff) == (b & 0xffff) {
                    self.count += 1;
                }

                Some(self.count)
            }
            None => None,
        }
    }
}

pub struct Day15 {
    seed_a: u64,
    seed_b: u64,
}

named!(parse_day15(&str) -> Day15, do_parse!(
    tag_s!("Generator A starts with ") >>
    seed_a: call!(parse_u64) >>
    ws!(tag_s!("Generator B starts with ")) >>
    seed_b: call!(parse_u64) >>
    (Day15 {
        seed_a,
        seed_b
    })
));

impl<'a> Day<'a> for Day15 {
    const NUM: u32 = 15;
    type Output1 = usize;
    type Output2 = usize;

    fn from_str(input: &str) -> Self {
        parse_day15(input).unwrap().1
    }

    fn part_1(&self) -> Self::Output1 {
        let (gen_a, gen_b) = Generator::new_pair(self.seed_a, self.seed_b);
        Judge::new(gen_a, gen_b)
            .skip(40_000_000 - 1)
            .next()
            .unwrap()
    }

    fn part_2(&self) -> Self::Output1 {
        let (gen_a, gen_b) = Generator::new_pair(self.seed_a, self.seed_b);
        let gen_a = FactorGenerator::new(gen_a, 4);
        let gen_b = FactorGenerator::new(gen_b, 8);
        Judge::new(gen_a, gen_b).skip(5_000_000 - 1).next().unwrap()
    }
}