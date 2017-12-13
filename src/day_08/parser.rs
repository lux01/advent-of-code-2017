use super::{Condition, Predicate, Instruction, OpCode};

use std::str::FromStr;

named!(parse_value(&str) -> i32, do_parse!(
    negative: opt!(ws!(tag_s!("-"))) >>
    value: ws!(map_res!(take_while_s!(|c: char| c.is_digit(10)), FromStr::from_str)) >>
    (if negative.is_some() { -1 * value } else { value })
));

named!(parse_predicate(&str) -> Predicate, do_parse!(
    cmp: alt!(tag_s!(">=") | tag_s!("<=") | tag_s!("==") | tag_s!("!=") | tag_s!(">") | tag_s!("<")) >>
    (match cmp {
        ">=" => Predicate::GreaterThanOrEqual,
        "<=" => Predicate::LessThanOrEqual,
        "==" => Predicate::Equal,
        "!=" => Predicate::NotEqual,
        ">" => Predicate::GreaterThan,
        "<" => Predicate::LessThan,
        _ => unreachable!()
    })
));

named!(parse_op_code(&str) -> OpCode,
    switch!( alt_complete!(tag_s!("inc") | tag_s!("dec")),
        "inc" => value!(OpCode::Inc) |
        "dec" => value!(OpCode::Dec)
    )
);

named!(parse_condition(&str) -> Condition, do_parse!(
    tag_s!("if ") >>
    register: take_until_s!(" ") >>
    tag_s!(" ") >>
    pred: call!(parse_predicate) >>
    tag_s!(" ") >>
    value: call!(parse_value) >>
    (Condition {
        predicate: pred,
        register: register,
        value: value,
    })
));

named!(pub parse_instruction(&str) -> Instruction, do_parse!(
    register: take_until_s!(" ") >>
    tag_s!(" ") >>
    op_code: parse_op_code >>
    tag_s!(" ") >>
    value: call!(parse_value) >>
    condition: ws!(call!(parse_condition)) >>
    ( Instruction {
        condition,
        op_code,
        register,
        value, 
    })
));

named!(pub parse_many_instructions(&str) -> Vec<Instruction>,
many1!(ws!(parse_instruction)));