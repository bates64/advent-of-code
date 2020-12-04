use super::Error;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use regex::Regex;

const INPUT: &str = include_str!("day4.txt");

type Passport = HashMap<String, String>;

fn passports() -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut cur_passport = Passport::new();

    let re = Regex::new(r"([a-z]{3}):([^\s]+)").unwrap();

    for line in INPUT.lines() {
        if line == "" {
            passports.push(cur_passport);
            cur_passport = Passport::new();
        }

        for caps in re.captures_iter(line) {
            cur_passport.insert(caps[1].to_owned(), caps[2].to_owned());
        }
    }

    passports.push(cur_passport);

    passports
}

pub fn part1() -> Result<String, Error> {
    let passports = passports();

    Ok(passports
        .into_iter()
        .filter(|passport| {
            for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
                if !passport.contains_key(field.to_owned()) {
                    return false;
                }
            }

            true
        })
        .count()
        .to_string())
}

macro_rules! unwrap {
    ($option:expr) => {
        if let Some(some) = $option {
            some
        } else {
            return false;
        }
    };
}

enum Height {
    Cm(i32),
    In(i32),
}

impl FromStr for Height {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_suffix("cm") {
            Ok(Height::Cm(s.parse().map_err(|_| ())?))
        } else if let Some(s) = s.strip_suffix("in") {
            Ok(Height::In(s.parse().map_err(|_| ())?))
        } else {
            Err(())
        }
    }
}

pub fn part2() -> Result<String, Error> {
    let passports = passports();

    let color_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let passport_id_re = Regex::new(r"^\d{9}$").unwrap();

    Ok(passports
        .into_iter()
        .filter(|passport| {
            let byr: i32 = unwrap!(unwrap!(passport.get("byr")).parse().ok());
            let iyr: i32 = unwrap!(unwrap!(passport.get("iyr")).parse().ok());
            let eyr: i32 = unwrap!(unwrap!(passport.get("eyr")).parse().ok());
            let hgt: Height = unwrap!(unwrap!(passport.get("hgt")).parse().ok());
            let hcl = unwrap!(passport.get("hcl"));
            let ecl = unwrap!(passport.get("ecl"));
            let pid = unwrap!(passport.get("pid"));

            (1920..=2002).contains(&byr) &&
            (2010..=2020).contains(&iyr) &&
            (2020..=2030).contains(&eyr) &&
            match hgt {
                Height::Cm(v) => (150..=193).contains(&v),
                Height::In(v) => (59..=76).contains(&v),
            } &&
            color_re.is_match(hcl) &&
            match ecl.as_str() {
                | "amb"
                | "blu"
                | "brn"
                | "gry"
                | "grn"
                | "hzl"
                | "oth" => true,
                _ => false,
            } &&
            passport_id_re.is_match(pid)
        })
        .count()
        .to_string())
}