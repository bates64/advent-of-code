use std::collections::{HashMap, HashSet};

use super::Error;

const INPUT: &str = include_str!("day4.txt");

pub fn part1() -> Result<String, Error> {
    let mut sum = 0;
    for line in INPUT.lines() {
        let colon = line.find(':').expect("line missing colon");

        let mut split = line[colon + 1..].split("|");
        let winners_str = split.next().unwrap();
        let actual_str = split.next().unwrap();

        let winners: HashSet<u32> = winners_str
            .split_whitespace()
            .map(|x| x.parse().expect("bad winner number"))
            .collect();

        let wins = actual_str
            .split_whitespace()
            .map(|x| x.parse().expect("bad actual number"))
            .filter(|x| winners.contains(x))
            .count();

        let mut score = 0;
        for _ in 0..wins {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        sum += score;
    }
    Ok(sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut card_count = HashMap::new();

    for line in INPUT.lines() {
        let colon = line.find(':').expect("line missing colon");
        let card = line["Card ".len()..colon]
            .trim()
            .parse::<u32>()
            .expect("bad game number");

        let copies = *card_count.entry(card).or_insert(1);

        let mut split = line[colon + 1..].split("|");
        let winners_str = split.next().unwrap();
        let actual_str = split.next().unwrap();

        let winners: HashSet<u32> = winners_str
            .split_whitespace()
            .map(|x| x.parse().expect("bad winner number"))
            .collect();

        let wins = actual_str
            .split_whitespace()
            .map(|x| x.parse().expect("bad actual number"))
            .filter(|x| winners.contains(x))
            .count() as u32;

        for i in 0..wins {
            *card_count.entry(card + 1 + i).or_insert(1) += copies;
        }
    }

    Ok(card_count.values().sum::<u32>().to_string())
}
