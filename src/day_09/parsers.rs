use super::GroupMember;

fn flatten(arr: Vec<&str>) -> String {
    let string_buffer = String::with_capacity(arr.len());
    arr.into_iter().fold(string_buffer, |mut buffer, str_part| {
        buffer.push_str(str_part);
        buffer
    })
}

named!(garbage(&str) -> GroupMember, do_parse!(
    tag!("<") >>
    contents: many_till!(
        alt_complete!(
            do_parse!(tag_s!("!") >> take_s!(1) >> ("")) |
            take_s!(1)
        ),
        tag_s!(">")
    ) >>
    (GroupMember::Garbage(flatten(contents.0)))
));

named!(pub group(&str) -> GroupMember, ws!(do_parse!(
    tag!("{") >>
    members: separated_list!(tag!(","),
        alt_complete!(
            garbage | group
        )
    ) >>
    tag!("}") >>
    (GroupMember::Group(0, members))
)));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    fn make_garbage(input: &str) -> GroupMember {
        match garbage(input) {
            IResult::Done(unused, output) => {
                assert_eq!("", unused);
                output
            }
            IResult::Error(err) => {
                panic!("Error: {:?}", err);
            }
            IResult::Incomplete(needed) => {
                panic!("Needed: {:?}", needed);
            }
        }
    }

    fn make_group(input: &str) -> GroupMember {
        match group(input) {
            IResult::Done(unused, output) => {
                assert_eq!("", unused);
                output
            }
            IResult::Error(err) => {
                panic!("Error: {:?}", err);
            }
            IResult::Incomplete(needed) => {
                panic!("Needed: {:?}", needed);
            }
        }
    }

    #[test]
    fn garbage_test() {
        assert_eq!(GroupMember::Garbage("".to_owned()), make_garbage("<>"));
        assert_eq!(GroupMember::Garbage("a".to_owned()), make_garbage("<a>"));
        assert_eq!(GroupMember::Garbage("".to_owned()), make_garbage("<!!>"));
        assert_eq!(
            GroupMember::Garbage("ab".to_owned()),
            make_garbage("<a!>b>")
        );
    }

    #[test]
    fn group_test() {
        assert_eq!(GroupMember::Group(0, vec![]), make_group("{}"));
        assert_eq!(
            GroupMember::Group(
                0,
                vec![GroupMember::Group(0, vec![GroupMember::Group(0, vec![])])],
            ),
            make_group("{{{}}}")
        );

        assert_eq!(
            GroupMember::Group(
                0,
                vec![
                    GroupMember::Garbage("a".to_owned()),
                    GroupMember::Garbage("b".to_owned()),
                    GroupMember::Garbage("c".to_owned()),
                    GroupMember::Garbage("d".to_owned()),
                    GroupMember::Garbage("e".to_owned()),
                ],
            ),
            make_group("{<a>,<b>,<c>,<d>,<e>}")
        );

        assert_eq!(
            GroupMember::Group(
                0,
                vec![
                    GroupMember::Group(
                        0,
                        vec![
                            GroupMember::Group(0, vec![GroupMember::Garbage("a".to_owned())]),
                            GroupMember::Garbage("b".to_owned()),
                            GroupMember::Garbage("c".to_owned()),
                            GroupMember::Garbage("d".to_owned()),
                            GroupMember::Garbage("e".to_owned()),
                        ]
                    ),
                ],
            ),
            make_group("{{{<a>},<b>,<c>,<d>,<e>}}")
        );

        assert_eq!(
            GroupMember::Group(
                0,
                vec![
                    GroupMember::Group(
                        0,
                        vec![
                            GroupMember::Group(0, vec![GroupMember::Garbage("a},<b".to_owned())]),
                        ]
                    ),
                ],
            ),
            make_group("{{{<a!>},<b>}}}")
        );

        assert_eq!(
            GroupMember::Group(
                0,
                vec![
                    GroupMember::Group(
                        0,
                        vec![GroupMember::Garbage("a},{<a},{<a},{<ab".to_owned())]
                    ),
                ],
            ),
            make_group("{{<a!>},{<a!>},{<a!>},{<ab>}}")
        );
    }
}