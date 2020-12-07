use std::collections::HashMap;

fn parse_rule(rule: &str) -> (&str, Vec<(u32, &str)>) {
    let rule = &rule[..rule.len() - 1];  // remove the last point
    let mut split = rule.split(" bags contain ");
    let bag_color = split.next().unwrap();
    let sub_bags = split.next().unwrap();
    let sub_bags = sub_bags.split(", ")
        .flat_map(|sub_bag| {
            if sub_bag == "no other bags" {
                return None;
            }
            let split_idx = sub_bag.find(' ').unwrap();
            let number = sub_bag.split_at(split_idx).0.parse().unwrap();
            let trailing_size = if number == 1 { 4 } else { 5 };
            let sub_bag = &sub_bag[split_idx + 1..sub_bag.len() - trailing_size]; // remove the number, the space, and the ending " bag".
            Some((number, sub_bag))
        })
        .collect::<Vec<_>>();
    (bag_color, sub_bags)
}

#[test]
fn test_parse() {
    assert_eq!(
        parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        ("light red", vec![(1, "bright white"), (2, "muted yellow")])
    );
}

fn compute_sub_bags<'a>(bag_name: &'a str, bags: &'a HashMap<&'a str, Vec<(u32, &'a str)>>) -> u32 {
    let mut total = 0;
    for &(count, sub_bag) in bags[bag_name].iter() {
        let sub_total = match (count, sub_bag) {
            (count, "shiny gold") => count,
            (count, sub_bag) => count * compute_sub_bags(sub_bag, bags),
        };
        total += sub_total;
    }
    total
}

pub fn part_1(input: &str) -> usize {
    let bags = input.lines()
        .map(parse_rule)
        .collect::<HashMap<_, _>>();
    bags.iter()
        .filter(|(bag_name, _)| {
             compute_sub_bags(bag_name, &bags) > 0
        })
        .count()
}

fn total_sub_bag(bag_name: &str, bags: &HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    bags[bag_name].iter()
        .map(|&(count, sub_bag)| {
            count + count * total_sub_bag(sub_bag, bags)
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let bags = input.lines()
        .map(parse_rule)
        .collect::<HashMap<_, _>>();
    bags["shiny gold"].iter()
        .map(|(count, sub_bag)| {
            count + count * total_sub_bag(sub_bag, &bags)
        })
        .sum()

}

pub fn main() {
    aoc_2020::day!(7, part_1, part_2);
}
