mod parser;

use super::Day;

use std::collections::HashMap;
use std::cmp;

type Register<'a> = &'a str;

#[derive(Debug, PartialEq, Eq)]
pub enum Predicate {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl Predicate {
    pub fn eval(&self, a: i32, b: i32) -> bool {
        match *self {
            Predicate::GreaterThan => a > b,
            Predicate::GreaterThanOrEqual => a >= b,
            Predicate::LessThan => a < b,
            Predicate::LessThanOrEqual => a <= b,
            Predicate::Equal => a == b,
            Predicate::NotEqual => a != b,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Condition<'a> {
    predicate: Predicate,
    register: Register<'a>,
    value: i32,
}

impl<'a> Condition<'a> {
    pub fn eval(&self, register_value: i32) -> bool {
        self.predicate.eval(register_value, self.value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    condition: Condition<'a>,
    op_code: OpCode,
    register: Register<'a>,
    value: i32,
}

impl<'a> Instruction<'a> {
    pub fn from_str(input: &'a str) -> Instruction<'a> {
        let parse_result = parser::parse_instruction(input);
        println!("{:?}", parse_result);
        let (unused, result) = parse_result.unwrap();
        assert_eq!("", unused);

        result
    }

    pub fn eval(&self, target_register_value: i32, condition_register_value: i32) -> i32 {
        if self.condition.eval(condition_register_value) {
            match self.op_code {
                OpCode::Inc => target_register_value + self.value,
                OpCode::Dec => target_register_value - self.value,
            }
        } else {
            target_register_value
        }
    }
}

pub struct StateMachine<'a> {
    registers: HashMap<&'a str, i32>,
    highest_value_seen: i32,
}


impl<'a> StateMachine<'a> {
    pub fn new() -> Self {
        StateMachine {
            registers: HashMap::new(),
            highest_value_seen: 0,
        }
    }

    pub fn eval(&mut self, instr: &Instruction<'a>) {
        let condition_register_value: i32 =
            *self.registers.entry(instr.condition.register).or_insert(0);
        let target_register: &mut i32 = self.registers.entry(instr.register).or_insert(0);

        *target_register = instr.eval(*target_register, condition_register_value);

        self.highest_value_seen = cmp::max(self.highest_value_seen, *target_register);
    }

    pub fn register_value(&self, key: &str) -> i32 {
        *self.registers.get(key).unwrap_or(&0)
    }

    pub fn all_registers(&self) -> Vec<(&'a str, i32)> {
        self.registers
            .iter()
            .map(|(&name, &val)| (name, val))
            .collect()
    }
}


pub struct Day08<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Day<'a> for Day08<'a> {
    const NUM: u32 = 8;
    type Output1 = i32;
    type Output2 = i32;

    fn from_str(input: &'a str) -> Self {
        let (_, instructions) = parser::parse_many_instructions(input).unwrap();

        Day08 { instructions }
    }

    fn part_1(&self) -> i32 {
        let mut state_machine = StateMachine::new();

        for instr in self.instructions.iter() {
            state_machine.eval(instr);
        }

        state_machine
            .all_registers()
            .into_iter()
            .max_by(|&(_, val1), &(_, val2)| val1.cmp(&val2))
            .unwrap()
            .1
    }

    fn part_2(&self) -> i32 {
        let mut state_machine = StateMachine::new();

        for instr in self.instructions.iter() {
            state_machine.eval(instr);
        }

        state_machine.highest_value_seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let test_inputs = [
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ];
        let test_outputs = [
            Instruction {
                condition: Condition {
                    predicate: Predicate::GreaterThan,
                    register: "a",
                    value: 1,
                },
                op_code: OpCode::Inc,
                register: "b",
                value: 5,
            },
            Instruction {
                condition: Condition {
                    predicate: Predicate::LessThan,
                    register: "b",
                    value: 5,
                },
                op_code: OpCode::Inc,
                register: "a",
                value: 1,
            },
            Instruction {
                condition: Condition {
                    predicate: Predicate::GreaterThanOrEqual,
                    register: "a",
                    value: 1,
                },
                op_code: OpCode::Dec,
                register: "c",
                value: -10,
            },
            Instruction {
                condition: Condition {
                    predicate: Predicate::Equal,
                    register: "c",
                    value: 10,
                },
                op_code: OpCode::Inc,
                register: "c",
                value: -20,
            },
        ];

        for i in 0..4 {
            assert_eq!(test_outputs[i], Instruction::from_str(&test_inputs[i]));
        }
    }

    #[test]
    fn eval_predicate() {
        use Predicate::*;

        assert!(GreaterThan.eval(2, 1));
        assert!(!GreaterThan.eval(2, 2));
        assert!(!GreaterThan.eval(1, 2));

        assert!(GreaterThanOrEqual.eval(2, 1));
        assert!(GreaterThanOrEqual.eval(2, 2));
        assert!(!GreaterThanOrEqual.eval(1, 2));

        assert!(LessThan.eval(1, 2));
        assert!(!LessThan.eval(1, 1));
        assert!(!LessThan.eval(2, 1));

        assert!(LessThanOrEqual.eval(1, 2));
        assert!(LessThanOrEqual.eval(1, 1));
        assert!(!LessThanOrEqual.eval(2, 1));

        assert!(!Equal.eval(1, 2));
        assert!(Equal.eval(1, 1));
        assert!(!Equal.eval(2, 1));

        assert!(NotEqual.eval(1, 2));
        assert!(!NotEqual.eval(1, 1));
        assert!(NotEqual.eval(2, 1));
    }

    #[test]
    fn eval_instructions() {
        let inc_success = Instruction::from_str("a inc 10 if b >= 0");
        let inc_fail = Instruction::from_str("a inc 10 if b > 100");
        let dec_success = Instruction::from_str("a dec 10 if b >= 0");
        let dec_fail = Instruction::from_str("a dec 10 if b > 100");

        assert_eq!(10, inc_success.eval(0, 0));
        assert_eq!(-10, dec_success.eval(0, 40));
        assert_eq!(0, inc_fail.eval(0, -100));
        assert_eq!(-10, dec_fail.eval(-10, 99));
    }

    #[test]
    fn eval_state_machine() {
        let mut state_machine = StateMachine::new();

        let test_str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        let (_, instructions) = parser::parse_many_instructions(&test_str).unwrap();

        for instr in instructions.into_iter() {
            state_machine.eval(&instr);
        }

        assert_eq!(1, state_machine.register_value("a"));
        assert_eq!(0, state_machine.register_value("b"));
        assert_eq!(-10, state_machine.register_value("c"));
    }
}