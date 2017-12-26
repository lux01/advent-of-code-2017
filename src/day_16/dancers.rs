use super::super::util::parse_usize;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

impl DanceMove {
    named!(pub parse(&str)-> DanceMove, alt_complete!(
        do_parse!(tag_s!("s") >> n: call!(parse_usize) >> (DanceMove::Spin(n))) |
        do_parse!(tag_s!("x") >> a: call!(parse_usize) >> tag_s!("/") >> b: call!(parse_usize) >> (DanceMove::Exchange(a, b))) |
        do_parse!(tag_s!("p") >> a: map!(take_s!(1), first_char) >> tag_s!("/") >> b: map!(take_s!(1), first_char) >> (DanceMove::Partner(a, b)))
    ));

    named!(pub parse_many(&str)-> Vec<DanceMove>, separated_list_complete!(tag_s!(","), call!(DanceMove::parse)));
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dancers {
    dancers: Vec<char>,
}

impl Dancers {
    pub fn new(last: char) -> Dancers {
        let start_u = 'a' as u8;
        let last_u = last as u8;
        let dancers = (start_u..last_u + 1).map(|n| n as char).collect();

        Dancers { dancers }
    }

    pub fn num_dancers(&self) -> usize {
        self.dancers.len()
    }

    fn dance(&mut self, dance_move: DanceMove) {
        match dance_move {
            DanceMove::Spin(n) => {
                let len = self.dancers.len();
                let mut new_dancers = Vec::with_capacity(len);
                for i in 0..len {
                    new_dancers.push(self.dancers[(i + len - n) % len]);
                }
                self.dancers = new_dancers;
            }
            DanceMove::Exchange(a, b) => {
                self.dancers.swap(a, b);
            }
            DanceMove::Partner(a, b) => {
                let index_of_a = self.dancers.iter().position(|&c| c == a).unwrap();
                let index_of_b = self.dancers.iter().position(|&c| c == b).unwrap();
                self.dancers.swap(index_of_a, index_of_b);
            }
        }
    }

    pub fn dance_all(&mut self, dance_moves: &[DanceMove]) {
        for dance_move in dance_moves.iter() {
            self.dance(*dance_move);
        }
    }

    pub fn dance_record(&mut self, input: &str) {
        let dance_moves = DanceMove::parse_many(input)
            .to_result()
            .expect("Invalid dance moves");

        self.dance_all(&dance_moves);
    }
}

impl PartialEq<str> for Dancers {
    fn eq(&self, other: &str) -> bool {
        self.dancers.iter().zip(other.chars()).all(|(&a, b)| a == b)
    }
}

impl PartialEq<Dancers> for str {
    fn eq(&self, other: &Dancers) -> bool {
        other == self
    }
}

impl<'a> PartialEq<Dancers> for &'a str {
    fn eq(&self, other: &Dancers) -> bool {
        other == *self
    }
}

impl fmt::Display for Dancers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.dancers.iter() {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dance_move_parse() {
        use self::DanceMove::*;
        assert_eq!(Spin(3), DanceMove::parse("s3").unwrap().1);
        assert_eq!(Exchange(2, 7), DanceMove::parse("x2/7").unwrap().1);
        assert_eq!(Partner('i', 'b'), DanceMove::parse("pi/b").unwrap().1);

        assert_eq!(
            vec![Spin(13), Exchange(1, 8), Partner('j', 'k')],
            DanceMove::parse_many("s13,x1/8,pj/k").unwrap().1
        )
    }

    #[test]
    fn dancers_display() {
        assert_eq!("abcde", Dancers::new('e'));
        assert_eq!("abcdefghijklmnop", Dancers::new('p'));
    }

    #[test]
    fn dance_move_iterate() {
        let mut start = Dancers::new('e');

        start.dance(DanceMove::Spin(1));
        assert_eq!("eabcd", start);

        start.dance(DanceMove::Exchange(3, 4));
        assert_eq!("eabdc", start);

        start.dance(DanceMove::Partner('e', 'b'));
        assert_eq!("baedc", start);
    }

    #[test]
    fn dance_move_iterate_from_str() {
        let mut start = Dancers::new('e');
        start.dance_record("s1,x3/4,pe/b");
        assert_eq!("baedc", start);
    }
}
