use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug)]
struct PasswordPolicy {
    first: u8,
    second: u8,
    letter: char,
}

#[derive(Debug)]
struct PasswordPolicyBytes {
    first: u8,
    second: u8,
    letter: u8,
}

fn parse_policy_bytes(input: &[u8]) -> PasswordPolicyBytes {
    let letter = *(input.last().unwrap());
    let input = &input[..input.len() - 2];
    let mut iter = input.split(|&b| b == b'-');
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let first = u8::from_str(unsafe { std::str::from_utf8_unchecked(first) }).unwrap();
    let second = u8::from_str(unsafe { std::str::from_utf8_unchecked(second) }).unwrap();
    PasswordPolicyBytes {
        first,
        second,
        letter,
    }
}

fn parse_policy(input: &str) -> PasswordPolicy {
    let split_idx = input.find('-').unwrap();
    let first = input[..split_idx].parse().unwrap();
    let second = input[split_idx + 1..input.len() - 2].parse().unwrap();
    let letter = input[input.len() - 1..].chars().next().unwrap();
    PasswordPolicy {
        first,
        second,
        letter,
    }
}

fn parse_line(line: &str) -> (PasswordPolicy, &str) {
    let split_idx = line.find(':').unwrap();
    let policy = parse_policy(&line[..split_idx]);
    let password = &line[split_idx + 2..];
    (policy, password)
}

fn parse_line_bytes(line: &[u8]) -> (PasswordPolicyBytes, &[u8]) {
    let mut iter = line.split(|&b| b == b':');
    let policy = parse_policy_bytes(iter.next().unwrap());
    let password = &iter.next().unwrap()[1..];
    (policy, password)
}

fn part_1(input: &str) -> usize {
    input.lines()
        .map(parse_line)
        .filter(|(policy, password)| {
            let count: u8 = (password.split(policy.letter).count() - 1).try_into().unwrap();
            count <= policy.second && count >= policy.first
        }).count()
}

fn part_1_bytes(input: &str) -> usize {
    input.lines()
        .map(str::as_bytes)
        .map(parse_line_bytes)
        .filter(|(policy, password)| {
            let count: u8 = (password.split(|&b| b == policy.letter).count() - 1).try_into().unwrap();
            count <= policy.second && count >= policy.first
        }).count()
}

fn part_2(input: &str) -> usize {
    input.lines()
        .map(parse_line)
        .filter(|(policy, password)| {
            let first = usize::from(policy.first);
            let second = usize::from(policy.second);
            let first = password[first - 1..second].chars().next().unwrap();
            let second = password[second - 1..second].chars().next().unwrap();
            first != second && (first == policy.letter || second == policy.letter)
        }).count()
}

fn part_2_bytes(input: &str) -> usize {
    input.lines()
        .map(str::as_bytes)
        .map(parse_line_bytes)
        .filter(|(policy, password)| unsafe {
            let first = *password.get_unchecked(usize::from(policy.first) - 1);
            let second = *password.get_unchecked(usize::from(policy.second) - 1);
            first != second && (first == policy.letter || second == policy.letter)
        }).count()
}

pub fn main() {
    aoc_2020::day!(2, part_1, part_2);
    aoc_2020::day!(2, part_1_bytes, part_2_bytes);
}
