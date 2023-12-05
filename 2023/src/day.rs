#![allow(dead_code)]

use thiserror::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[derive(Error, Debug)]
pub enum Error {
    #[error("todo")]
    Unimplemented,

    #[error("no solution")]
    NoSolution,

    #[error("bad input: {0}")]
    BadInput(#[from] std::num::ParseIntError),

    #[error("{0}")]
    Io(#[from] std::io::Error),
}

type Solution = fn() -> Result<String, Error>;

fn todo() -> Result<String, Error> {
    Err(Error::Unimplemented)
}

pub const NUM_DAYS: usize = 5;

pub const ALL_DAYS: [(Solution, Solution); NUM_DAYS] = [
    (day1::part1, day1::part2),
    (day2::part1, day2::part2),
    (day3::part1, day3::part2),
    (day4::part1, day4::part2),
    (day5::part1, day5::part2),
];
