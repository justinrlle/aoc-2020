fn find_pos(part: &[u8], min: u8, max: u8, go_down: u8, go_up: u8) -> u8 {
    let last = *part.last().unwrap();
    let part = part.iter()
        .fold((min, max), |(min, max), &letter| {
            let delta = (max - min) / 2;
            if letter == go_down {
                (min, min + delta)
            } else if letter == go_up {
                (max - delta, max)
            } else {
                unreachable!();
            }
        });
    if last == go_down { part.0 } else { part.1 }
}
fn seat_id(seat: &[u8]) -> usize {
    let row_part = &seat[..7];
    let row = find_pos(row_part, 0, 127, b'F', b'B');
    let col_part = &seat[7..];
    let col = find_pos(col_part, 0, 7, b'L', b'R');
    ((row as usize) * 8) + (col as usize)
}

pub fn part_1(input: &str) -> usize {
    input.lines()
        .map(str::as_bytes)
        .map(seat_id)
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut ids = input.lines()
        .map(str::as_bytes)
        .map(seat_id)
        .collect::<Vec<_>>();
    ids.sort_unstable();
    let gap = ids.windows(2).find(|&pair| {
        let prev = pair[0];
        let next = pair[1];
        prev + 1 != next
    }).unwrap();
    gap[0] + 1
}


pub fn main() {
    aoc_2020::day!(5, part_1, part_2);
}

#[test]
fn test_part_1() {
    let example = "FBFBBFFRLR";
    assert_eq!(find_pos(&example.as_bytes()[..7], 0, 127, b'F', b'B'), 44);
    assert_eq!(find_pos(&example.as_bytes()[7..], 0, 7, b'L', b'R'), 5);
    assert_eq!(part_1(example), 357);
    let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n";
    assert_eq!(part_1("BFFFBBFRRR"), 567);
    assert_eq!(part_1("FFFBBBFRRR"), 119);
    assert_eq!(part_1(input), 820);
}
