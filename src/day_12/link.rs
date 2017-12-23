use super::super::util::parse_usize;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Link {
    pub from: usize,
    pub to: usize,
}

impl Link {
    fn between(a: usize, b: usize) -> Link {
        Link {
            from: a.min(b),
            to: a.max(b),
        }
    }

    pub fn from_str(input: &str) -> HashSet<Link> {
        parse_links(input).unwrap().1
    }
}

named!(parse_single_link(&str) -> (usize, Vec<usize>), do_parse!(
    from: call!(parse_usize) >>
    tag_s!(" <-> ") >>
    to: separated_list_complete!(tag_s!(", "), call!(parse_usize)) >>
    ((from, to))
));

named!(parse_links(&str) -> HashSet<Link>, ws!(
    fold_many1!(
        ws!(call!(parse_single_link)),
        HashSet::new(),
        |mut set: HashSet<Link>, (from, links): (usize, Vec<usize>)| {
            for to in links.into_iter() {
                set.insert(Link::between(from, to));
            }
            set
        }
    )
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_input_line() {
        assert_eq!((0, vec![2]), parse_single_link("0 <-> 2").unwrap().1);
        assert_eq!(
            (2, vec![0, 3, 4]),
            parse_single_link("2 <-> 0, 3, 4").unwrap().1
        );
    }

    #[test]
    fn parse_many_input_lines() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let mut output = HashSet::new();
        output.insert(Link::between(0, 2));
        output.insert(Link::between(1, 1));
        output.insert(Link::between(2, 3));
        output.insert(Link::between(2, 4));
        output.insert(Link::between(3, 4));
        output.insert(Link::between(4, 6));
        output.insert(Link::between(5, 6));

        assert_eq!(output, parse_links(&input).unwrap().1);
    }
}
