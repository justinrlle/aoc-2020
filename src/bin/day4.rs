pub fn part_1(input: &str) -> usize {
    input.split("\n\n")
        .filter(|passport| {
            passport.find("byr").is_some()
                && passport.find("iyr").is_some()
                && passport.find("eyr").is_some()
                && passport.find("hgt").is_some()
                && passport.find("hcl").is_some()
                && passport.find("ecl").is_some()
                && passport.find("pid").is_some()
        })
        .count()
}

mod checks {
    #[inline]
    fn year(field: &str, lower: u16, upper: u16) -> bool {
        if field.len() != 8 {
            return false;
        }
        field[4..].parse::<u16>()
            .map(|year| year >= lower && year <= upper)
            .unwrap_or(false)
    }

    #[inline]
    pub fn byr(field: &str) -> bool {
        year(field, 1920, 2002)
    }
    #[inline]
    pub fn iyr(field: &str) -> bool {
        year(field, 2010, 2020)
    }
    #[inline]
    pub fn eyr(field: &str) -> bool {
        year(field, 2020, 2030)
    }

    #[inline]
    pub fn hgt(field: &str) -> bool {
        if field.len() != 8 && field.len() != 9 {
            return false;
        }
        let unit = &field[field.len() - 2..];
        let (lower, upper) = if unit == "cm" {
            (150, 193)
        } else if unit == "in" {
            (59, 76)
        } else {
            return false;
        };
        field[4..field.len() - 2].parse::<u16>()
            .map(|height| height >= lower && height <= upper)
            .unwrap_or(false)
    }

    #[inline]
    pub fn hcl(field: &str) -> bool {
        if field.len() != 11 || field.as_bytes()[4] != b'#' {
            return false;
        }
        field[5..].bytes()
            .all(|c| (c <= b'9' && c >= b'0') || (c <= b'f' && c >= b'a'))
    }

    #[inline]
    pub fn ecl(field: &str) -> bool {
        static COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        field.len() == 7 && COLORS.contains(&&field[4..])
    }

    #[inline]
    pub fn pid(field: &str) -> bool {
        field.len() == 13 && field[4..].bytes().all(|c| c <= b'9' && c >= b'0')
    }
}

pub fn part_2(input: &str) -> usize {
    input.split("\n\n")
        .filter(|passport| {
            passport.split(&['\n', ' '][..])
                .filter(|l| !l.is_empty())
                .try_fold(0u8, |mask, field| {
                    let valid = match &field[0..3] {
                        "byr" => if checks::byr(field) { 0b0000_0001 } else { return Err(()); },
                        "iyr" => if checks::iyr(field) { 0b0000_0010 } else { return Err(()); },
                        "eyr" => if checks::eyr(field) { 0b0000_0100 } else { return Err(()); },
                        "hgt" => if checks::hgt(field) { 0b0000_1000 } else { return Err(()); },
                        "hcl" => if checks::hcl(field) { 0b0001_0000 } else { return Err(()); },
                        "ecl" => if checks::ecl(field) { 0b0010_0000 } else { return Err(()); },
                        "pid" => if checks::pid(field) { 0b0100_0000 } else { return Err(()); },
                        "cid" => return Ok(mask), // cid is ignored, we skip him
                        _ => unreachable!(),
                    };
                    Ok(mask | valid)
                }).unwrap_or(0b0000_0000) == 0b0111_1111
        })
        .count()
}

#[test]
fn test_part_1() {
    let example = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    assert_eq!(part_1(example), 2);
}

#[test]
fn test_part_2() {
    let invalids = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    let valids = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(part_2(invalids), 0);
    assert_eq!(part_2(valids), 4);
}

pub fn main() {
    aoc_2020::day!(4, part_1, part_2);
}

