use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    let mut adapters = aoc_2020::parse_all::<u32>(input)
        .collect::<Vec<_>>();
    adapters.sort_unstable();
    let (one_jolt, three_jolts, _) = adapters.iter()
        .fold((0, 1, 0), |(one_jolt, three_jolts, jolt), &adapter| {
            match adapter - jolt {
                1 => (one_jolt + 1, three_jolts, adapter),
                3 => (one_jolt, three_jolts + 1, adapter),
                _ => (one_jolt, three_jolts, adapter),
            }
        });
    one_jolt * three_jolts
}


pub fn part_2(input: &str) -> u128 {
    let mut adapters = aoc_2020::parse_all::<u64>(input)
        .collect::<Vec<_>>();
    adapters.sort_unstable();
    let mut dp
    part_2_inner(&adapters, 0, &mut cache)
}

fn main() {
    aoc_2020::day!(10, part_1, part_2);
}

