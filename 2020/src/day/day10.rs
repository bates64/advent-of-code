use super::Error;
use std::collections::BTreeSet;
use cached::proc_macro::cached;

const INPUT: &str = include_str!("day10.txt");

pub fn part1() -> Result<String, Error> {
    let lines: BTreeSet<u64> = INPUT.lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut ones = 0;
    let mut threes = 1;

    let mut prev = 0;
    for n in lines {
        let delta = n - prev;
        match delta {
            0 => (),
            1 => ones += 1,
            2 => (),
            3 => threes += 1,
            _ => panic!(),
        }

        prev = n;
    }

    Ok((ones * threes).to_string())
}

pub fn part2() -> Result<String, Error> {
    let lines: BTreeSet<u64> = INPUT.lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let p = count_permutations(lines, 0);

    Ok(p.to_string())
}

#[cached]
fn count_permutations(lines: BTreeSet<u64>, from: u64) -> u64 {
    if lines.is_empty() {
        return 1;
    }

    let mut choices = lines.clone();
    choices.split_off(&(from + 4));

    choices
        .into_iter()
        .map(|choice| count_permutations(lines.clone().split_off(&(choice + 1)), choice))
        .sum()
}