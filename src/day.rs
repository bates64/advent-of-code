#![allow(dead_code)]

use thiserror::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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

pub const NUM_DAYS: usize = 14;

pub const ALL_DAYS: [(Solution, Solution); NUM_DAYS] = [
    (day1::part1, day1::part2),
    (day2::part1, day2::part2),
    (day3::part1, day3::part2),
    (day4::part1, day4::part2),
    (day5::part1, day5::part2),
    (day6::part1, day6::part2),
    (day7::part1, day7::part2),
    (day8::part1, day8::part2),
    (day9::part1, day9::part2),
    (day10::part1, day10::part2),
    (day11::part1, day11::part2),
    (day12::part1, day12::part2),
    (day13::part1, day13::part2),
    (day14::part1, day14::part2),
];
