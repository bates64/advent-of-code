use super::Error;
use std::collections::VecDeque;

const INPUT: &str = include_str!("day9.txt");

fn is_sum(line: u64, nums: &VecDeque<u64>) -> bool {
    for a in nums.iter() {
        for b in nums.iter() {
            if a != b && a + b == line {
                return true;
            }
        }
    }

    false
}

pub fn part1() -> Result<String, Error> {
    let mut nums = VecDeque::with_capacity(25);

    for line in INPUT.lines() {
        let line: u64 = line.parse()?;

        if nums.len() == 25 {
            if is_sum(line, &nums) == false {
                return Ok(line.to_string());
            }
        }

        nums.push_front(line);
        nums.truncate(25);
    }

    Err(Error::NoSolution)
}

pub fn part2() -> Result<String, Error> {
    let target: u64 = part1()?.parse().unwrap();
    let lines: Vec<u64> = INPUT.lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let mut pos = 0;
    loop {
        let mut i = pos;
        let mut sum = 0;

        while sum < target {
            match lines.get(i) {
                Some(n) => sum += n,
                None => break,
            }
            i += 1;
        }

        if sum == target {
            let range = &lines[pos..i];
            let max = range.iter().max().unwrap();
            let min = range.iter().min().unwrap();

            return Ok((max + min).to_string())
        } else {
            pos += 1;

            if pos >= lines.len() {
                return Err(Error::NoSolution);
            }
        }
    }
}
