use super::util::parse_isize;
use super::Day;

#[derive(PartialEq, Eq, Debug)]
pub struct Firewall {
    depth: isize,
    range: isize,
}

named!(parse_firewalls(&str) -> Vec<Firewall>,
    many1!(ws!(do_parse!(
        depth: call!(parse_isize) >>
        tag_s!(": ") >>
        range: call!(parse_isize) >> 
        (Firewall::new(depth, range))
    )))
);

impl Firewall {
    pub fn from_str(input: &str) -> Vec<Firewall> {
        parse_firewalls(input).unwrap().1
    }

    pub fn new(depth: isize, range: isize) -> Firewall {
        Firewall { depth, range }
    }

    pub fn scans_packet(&self, start_time: isize) -> bool {
        (start_time + self.depth) % (2 * self.range - 2) == 0
    }

    pub fn severity(&self) -> isize {
        self.depth * self.range
    }
}

pub struct Day13 {
    firewalls: Vec<Firewall>,
}

impl<'a> Day<'a> for Day13 {
    const NUM: u32 = 13;
    type Output1 = isize;
    type Output2 = isize;

    fn from_str(input: &str) -> Self {
        Day13 { firewalls: Firewall::from_str(input) }
    }

    fn part_1(&self) -> Self::Output1 {
        self.firewalls
            .iter()
            .filter(|&fw| fw.scans_packet(0))
            .fold(0, |acc, fw| acc + (fw.depth * fw.range))
    }

    fn part_2(&self) -> Self::Output2 {
        let mut t = 0;
        loop {
            if self.firewalls.iter().all(|fw| !fw.scans_packet(t)) {
                return t;
            }
            t += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn parse_input_firewalls() {
        let output = vec![
            Firewall { depth: 0, range: 3 },
            Firewall { depth: 1, range: 2 },
            Firewall { depth: 4, range: 4 },
            Firewall { depth: 6, range: 4 },
        ];

        assert_eq!(output, Firewall::from_str(&INPUT));
    }

    #[test]
    fn calculate_tzero_severity() {
        assert_eq!(24, Day13::from_str(&INPUT).part_1());
    }

    #[test]
    fn safe_time() {
        assert_eq!(10, Day13::from_str(&INPUT).part_2());
    }
}