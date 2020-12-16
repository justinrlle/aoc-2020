use std::collections::{BTreeSet, HashMap};


fn part_1_inner(input: &str, preamble_size: usize) -> u64 {
    let nbs = aoc_2020::parse_all(input)
        .collect::<Vec<_>>();

    fn is_nb_invalid(nb: u64, preamble: &BTreeSet<u64>) -> bool {
        for &i in preamble.iter() {
            if i > nb {
                continue
            }
            if preamble.contains(&(nb - i)) {
                return false;
            }
        }
        true
    }

    let mut preamble = BTreeSet::new();
    for nbs in nbs.windows(preamble_size + 1) {
        preamble.clear();
        preamble.extend(nbs[..preamble_size].iter().copied());
        let nb = nbs[preamble_size];
        if is_nb_invalid(nb, &preamble) {
            return nb;
        }
    }
    panic!("nb not found");
}

#[test]
fn test_part_1() {
    let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(part_1_inner(input, 5), 127);
}

pub fn part_1(input: &str) -> u64 {
    part_1_inner(input, 25)
}

pub fn part_2_inner(input: &str, invalid_nb: u64) -> u64 {
    let mut guesses = HashMap::new();
    let nbs = input.lines()
        .enumerate()
        .map(|(idx, l)| {
            l.parse::<u64>()
                .map_err(|e| format!("failed to parse {} (line {}) into a number: {}", l, idx, e))
                .unwrap()
        });
    for nb in nbs {
        for (&key, (count, min, max)) in guesses.iter_mut() {
            if *count + nb == invalid_nb {
                dbg!(key, nb, *min, *max);
                return *min + *max;
            }
            *count += nb;
            if nb < *min {
                *min = nb;
            } else if nb > *max {
                *max = nb;
            }
        }
        guesses.insert(nb, (nb, nb, nb));
    }
    panic!("No nb found");
}

#[test]
fn test_part_2() {
    let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(part_2_inner(input, 127), 62);
}

pub fn part_2(input: &str) -> u64 {
    part_2_inner(input, 556543474)
}

fn main() {
    aoc_2020::day!(9, part_1, part_2);
}
