use crate::InstKind::{Nop, Jmp};
use InstKind::Acc;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum InstKind { Nop, Acc, Jmp }

pub fn part_1(input: &str) -> i32 {
    let stack = parse_stack(input);
    run_program(&stack).1
}

fn switch_inst(inst: &mut (InstKind, i32)) {
    match inst {
        (InstKind::Jmp, _) => inst.0 = InstKind::Nop,
        (InstKind::Nop, _) => inst.0 = InstKind::Jmp,
        _ => {},
    }
}

pub fn part_2(input: &str) -> i32 {
    let mut stack = parse_stack(input);
    for inst_pos in run_program(&stack).2 {
        if let InstKind::Jmp = stack[inst_pos].0 {
            switch_inst(&mut stack[inst_pos]);
            if let (true, acc, _) = run_program(&stack) {
                return acc;
            }
            switch_inst(&mut stack[inst_pos]);
        }
    }
    panic!("NOOOO");
}

fn parse_stack(input: &str) -> Vec<(InstKind, i32)> {
    let stack = input.lines()
        .map(|line| {
            let kind = match &line[..3] {
                "nop" => Nop,
                "acc" => Acc,
                "jmp" => Jmp,
                _ => unreachable!(),
            };
            let value = line[4..].parse::<i32>().unwrap();
            (kind, value)
        })
        .collect::<Vec<_>>();
    stack
}

fn run_program(stack: &[(InstKind, i32)]) -> (bool, i32, Vec<usize>) {
    let mut instructions_seen = Vec::with_capacity(stack.len());
    let mut stack_pointer = 0;
    let mut acc = 0;
    while stack_pointer < stack.len() {
        if instructions_seen.contains(&stack_pointer) {
            return (false, acc, instructions_seen);
        }
        instructions_seen.push(stack_pointer);
        let (kind, value) = stack[stack_pointer];
        match kind {
            Acc => {
                acc += value;
                stack_pointer += 1
            }
            Jmp => {
                if value < 0 {
                    stack_pointer -= value.abs() as usize;
                } else {
                    stack_pointer += value as usize;
                }
            }
            Nop => { stack_pointer += 1 }
        }
    }
    (true, acc, instructions_seen)
}
fn main() {
    aoc_2020::day!(8, part_1, part_2);
}

#[test]
fn test_parse_nb() {
    assert_eq!("+1".parse::<i32>(), Ok(1));
    assert_eq!("-1".parse::<i32>(), Ok(-1));
}

#[test]
fn test_example_part_1() {
    let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(part_1(input), 5);
}

#[test]
fn test_example_part_2() {
    let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(part_2(input), 8);
}
