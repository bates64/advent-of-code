use super::Error;
use std::collections::HashMap;

const INPUT: &str = include_str!("day15.txt");

fn spoken_at(turn_number: u64) -> u64 {
    let nums: Vec<u64> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();

    let start = 1 + nums.len() as u64;
    let mut previous = nums[nums.len() - 1];

    let mut when: HashMap<_, _> = nums.into_iter()
        .enumerate()
        .map(|(idx, n)| (n, 1 + idx as u64))
        .collect();
    when.remove(&previous);

    for turn in start..=turn_number {
        let spoken = match when.get(&previous) {
            None => 0, // it's new!
            Some(last_spoken) => turn - 1 - *last_spoken,
        };
        when.insert(previous, turn - 1);

        previous = spoken;
    }

    previous
}

pub fn part1() -> Result<String, Error> {
    Ok(spoken_at(2020).to_string())
}

pub fn part2() -> Result<String, Error> {
    Ok(spoken_at(30000000).to_string())
}
