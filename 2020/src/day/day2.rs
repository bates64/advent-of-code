use recap::Recap;
use serde::Deserialize;
use super::Error;

const INPUT: &str = include_str!("day2.txt");

#[derive(Recap, Deserialize)]
#[recap(regex=r#"(?x)
    (?P<at_least>\d+)
    -
    (?P<at_most>\d+)
    \s+
    (?P<ch>\S)
    :
    \s+
    (?P<password>\S+)
"#)]
struct Input {
    at_least: usize,
    at_most: usize,
    ch: char,
    password: String,
}

pub fn part1() -> Result<String, Error> {
    Ok(INPUT.lines()
        .filter_map(|line| line.parse::<Input>().ok())
        .filter(|Input { at_least, at_most, ch, password }| {
            let count = password
                .chars()
                .filter(|pass_ch| pass_ch == ch)
                .count();

            count >= *at_least && count <= *at_most
        })
        .count()
        .to_string())
}

pub fn part2() -> Result<String, Error> {
    Ok(INPUT.lines()
        .filter_map(|line| line.parse::<Input>().ok())
        .filter(|Input { at_least: idx_a, at_most: idx_b, ch, password }| {
            let a = password.chars().nth(*idx_a - 1).unwrap_or_default() == *ch;
            let b = password.chars().nth(*idx_b - 1).unwrap_or_default() == *ch;

            a ^ b
        })
        .count()
        .to_string())
}
