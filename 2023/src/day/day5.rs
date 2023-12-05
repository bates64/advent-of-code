use super::Error;
use rayon::prelude::*;

const INPUT: &str = include_str!("day5.txt");

pub fn part1() -> Result<String, Error> {
    let mut lines = INPUT.lines();
    let mut seeds: Vec<usize> = lines.next().expect("seeds")["seeds: ".len()..]
        .split_whitespace()
        .map(|x| x.parse().expect("bad seed"))
        .collect();

    // proceed in rounds
    let mut next_seeds = seeds.clone();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            seeds = next_seeds.clone();
            lines.next(); // "x-to-y map:"
            continue;
        }

        let mut split = line.split_whitespace().map(|x| x.parse().expect("bad int"));
        let dest_range_start = split.next().expect("dest range");
        let source_range_start = split.next().expect("source range");
        let range_len = split.next().expect("range len");

        // update seeds that are in the range
        seeds
            .par_iter()
            .zip(next_seeds.par_iter_mut())
            .for_each(|(prev, next)| {
                if *prev >= source_range_start && *prev < source_range_start + range_len {
                    *next = dest_range_start + (*prev - source_range_start);
                }
            });
    }

    let lowest = next_seeds.iter().min().expect("min");
    Ok(lowest.to_string())
}

// slow af
pub fn part2() -> Result<String, Error> {
    let mut lines = INPUT.lines();
    let mut seeds: Vec<usize> = lines.next().expect("seeds")["seeds: ".len()..]
        .split_whitespace()
        .map(|x| x.parse().expect("bad seed"))
        .collect::<Vec<usize>>()
        .chunks(2)
        .flat_map(|s| {
            let start = s[0];
            let length = s[1];
            (start..start + length).collect::<Vec<usize>>()
        })
        .collect();

    // proceed in rounds
    let mut next_seeds = seeds.clone();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            seeds = next_seeds.clone();
            lines.next(); // "x-to-y map:"
            continue;
        }

        let mut split = line.split_whitespace().map(|x| x.parse().expect("bad int"));
        let dest_range_start = split.next().expect("dest range");
        let source_range_start = split.next().expect("source range");
        let range_len = split.next().expect("range len");

        // update seeds that are in the range
        seeds
            .par_iter()
            .zip(next_seeds.par_iter_mut())
            .for_each(|(prev, next)| {
                if *prev >= source_range_start && *prev < source_range_start + range_len {
                    *next = dest_range_start + (*prev - source_range_start);
                }
            });
    }

    let lowest = next_seeds.iter().min().expect("min");
    Ok(lowest.to_string())
}
