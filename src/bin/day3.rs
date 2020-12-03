pub fn part_1(input: &str) -> u32 {
    input.lines()
        .map(str::as_bytes)
        .enumerate()
        .fold(0, |mut count, (idx, line)| {
            if check(line, idx * 3) {
                count += 1;
            }
            count
        })
}

#[inline]
fn check(line: &[u8], x: usize) -> bool {
    line[x % line.len()] == b'#'
}

pub fn part_2(input: &str) -> usize {
    let (c1, c2, c3, c4, c5) = input.lines()
        .map(str::as_bytes)
        .enumerate()
        .fold((0, 0, 0, 0, 0), |(mut c1, mut c2, mut c3, mut c4, mut c5), (idx, line)| {
            if check(line, idx) {
                c1 += 1;
            }
            if check(line, idx * 3) {
                c2 += 1;
            }
            if check(line, idx * 5) {
                c3 += 1
            }
            if check(line, idx * 7) {
                c4 += 1;
            }
            if idx % 2 == 0 && check(line, idx / 2) {
                c5 += 1;
            }
            (c1, c2, c3, c4, c5)
        });
    c1 * c2 * c3 * c4 * c5
}

pub fn main() {
    aoc_2020::day!(3, part_1, part_2);
}
