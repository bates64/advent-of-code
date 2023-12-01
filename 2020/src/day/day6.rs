use super::Error;
use std::collections::{HashSet, HashMap};

const INPUT: &str = include_str!("day6.txt");

pub fn part1() -> Result<String, Error> {
    let mut questions = HashSet::new();
    let mut count = 0;

    for line in INPUT.lines() {
        if line == "" {
            count += questions.len();
            questions = HashSet::new();
        } else {
            for ch in line.chars() {
                questions.insert(ch);
            }
        }
    }

    count += questions.len();

    Ok(count.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut questions: HashMap<char, u32> = HashMap::new();
    let mut num_people = 0;
    let mut count = 0;

    for line in INPUT.lines() {
        if line == "" {
            count += questions
                .into_iter()
                .filter(|(_, u)| *u == num_people)
                .count();
            questions = HashMap::new();
            num_people = 0;
        } else {
            num_people += 1;
            for ch in line.chars() {
                *questions.entry(ch).or_default() += 1;
            }
        }
    }

    count += questions
        .into_iter()
        .filter(|(_, u)| *u == num_people)
        .count();

    Ok(count.to_string())
}
