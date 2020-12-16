use std::fmt::Display;
use std::str::FromStr;

pub fn day_input(day: u8) -> String {
    let path = format!("./days/{}", day);
    match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => panic!("failed to read {}: {}", path, e),
    }
}

pub fn parse_all<'a, T: FromStr>(input: &'a str) -> impl Iterator<Item=T> + 'a
    where <T as FromStr>::Err: Display {
    input.lines()
        .enumerate()
        .map(|(idx, l)| {
            l.parse::<T>()
                .map_err(|e|
                    format!("failed to parse {} (line {})\n  could not parse into {}: {}",
                            l,
                            idx,
                            std::any::type_name::<T>(),
                            e))
                .unwrap()
        })
}

#[macro_export]
macro_rules! day {
    ($day:expr, $part_1:ident) => {
        {
            let input = $crate::day_input($day);
            println!("day {}:", $day);
            println!(" - part 1 ({}): {}", stringify!($part_1), $part_1(&input));
        }
    };
    ($day:expr, $part_1:ident, $part_2:ident) => {
         {
            let input = $crate::day_input($day);
            println!("day {}:", $day);
            println!(" - part 1 ({}): {}", stringify!($part_1), $part_1(&input));
            println!(" - part 2 ({}): {}", stringify!($part_2), $part_2(&input));
        }
    };
}

