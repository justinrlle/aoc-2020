pub fn part_1(input: &str) -> u32 {
    input.split("\n\n")
        .filter(|group| !group.is_empty())
        .map(str::as_bytes)
        .map(|group| {
            group.into_iter()
                .filter(|&&c| c >= b'a' && c <= b'z')
                .fold(0u32, |mask, &answer| mask | 1 << (answer - b'a'))
                .count_ones()
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input.split("\n\n")
        .filter(|group| !group.is_empty())
        .map(|group| {
            group.lines()
                .map::<u32, _>(|answer| {
                    answer.bytes()
                        .fold(0, |mask, answer| mask | 1 << (answer - b'a'))
                })
                .fold(u32::MAX, |answers, answer| answers & answer)
                .count_ones()
        })
        .sum()
}

#[test]
fn test_part_1() {
    let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(part_1(input), 11);
}

#[test]
fn test_part_2() {
    let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(part_2(input), 6);
}

pub fn main() {
    aoc_2020::day!(6, part_1, part_2);
}
