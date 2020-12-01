fn part_1(input: &str) -> u32 {
    let mut entries = Vec::with_capacity(input.len() / 4); // ~= 4.9 chars per line on average, including NLs
    for entry in input.lines() {
        let entry = entry.parse::<u32>().expect("invalid number");
        for prev_entry in entries.iter() {
            if entry + prev_entry == 2020 {
                return entry * prev_entry;
            }
        }
        entries.push(entry);
    }
    unreachable!("not found");
}

fn part_2(input: &str) -> u32 {
    let mut entries = Vec::with_capacity(input.len() / 4); // ~= 4.9 chars per line on average, including NLs
    for entry in input.lines() {
        let entry = entry.parse::<u32>().expect("invalid number");
        for idx in 0..entries.len() {
            let first = entries[idx];
            for second in entries.iter().enumerate().filter(|&(i, _)| i != idx).map(|(_, i)| i) {
                if first + second + entry == 2020 {
                    return first * second * entry;
                }
            }
        }
        entries.push(entry);
    }
    unreachable!("not found");
}


pub fn main() {
    aoc_2020::day!(1, part_1, part_2);
}
