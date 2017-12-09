use super::program::Program;

use std::str;
use std::str::FromStr;


fn is_digit(c: char) -> bool {
    c.is_digit(10)
}


named!(children_list(&str) -> Vec<&str>, do_parse!(
    tag!(" -> ") >>
    children: many1!(do_parse!( opt!(tag!(", ")) >> name: take_while_s!(char::is_alphabetic) >> (name))) >>
    (children)
));

named!(parse_weight(&str) -> u64,
    delimited!(tag!("("), map_res!(take_while_s!(is_digit), FromStr::from_str), tag!(")"))
);

named!(program(&str) -> (Program, Vec<&str>), do_parse!(
    name: take_until_s!(" ") >>
    tag!(" ") >>
    weight: parse_weight >>
    children: opt!(complete!(children_list)) >>
    ((Program::new(name, weight), children.unwrap_or(vec![])))
));

named!(pub many_programs(&str) -> Vec<(Program, Vec<&str>)>,
    many1!(ws!(program)));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_weight_test() {
        let input = "(12)";
        let output = 12;

        let (_, parser_output) = parse_weight(&input).unwrap();

        assert_eq!(output, parser_output);
    }

    #[test]
    fn parse_program_with_children() {
        let input = "ugml (68) -> gyxo, ebii, jptl";
        let output = (Program::new("ugml", 68), vec!["gyxo", "ebii", "jptl"]);

        let (unused, parsed_output) = program(&input).unwrap();

        assert_eq!("", unused);
        assert_eq!(output, parsed_output);
    }

    #[test]
    fn parse_program_without_children() {
        let input = "ugml (68)";
        let output = (Program::new("ugml", 68), vec![]);

        let (unused, parsed_output) = program(&input).unwrap();

        assert_eq!("", unused);
        assert_eq!(output, parsed_output);
    }
}