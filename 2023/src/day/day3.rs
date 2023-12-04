use std::collections::{HashMap, HashSet};

use super::Error;

const INPUT: &str = include_str!("day3.txt");

pub fn part1() -> Result<String, Error> {
    // a symbol is any char other than a number or a period
    let symbol_positions: HashSet<(usize, usize)> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .filter(|(_, c)| *c != '.' && !c.is_ascii_digit())
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut sum = 0;
    for (y, line) in INPUT.lines().enumerate() {
        let mut value = 0;
        let mut has_symbol_neighbour = false;

        for (x, ch) in line.char_indices() {
            if let Some(digit) = ch.to_digit(10) {
                value = value * 10 + digit;

                // search 8 neighbours for symbols
                let xi = x as isize;
                let yi = y as isize;
                let found_symbol_neighbour = [
                    (xi + 1, yi),
                    (xi + 1, yi + 1),
                    (xi, yi + 1),
                    (xi - 1, yi + 1),
                    (xi - 1, yi),
                    (xi - 1, yi - 1),
                    (xi, yi - 1),
                    (xi + 1, yi - 1),
                ]
                .into_iter()
                .filter_map(|(x, y)| {
                    // exclude out of bounds
                    let x: usize = x.try_into().ok()?;
                    let y: usize = y.try_into().ok()?;
                    Some((x, y))
                })
                .fold(false, |acc, (x, y)| {
                    // check for 8-way neighbours
                    acc || symbol_positions.contains(&(x, y))
                });

                if found_symbol_neighbour {
                    has_symbol_neighbour = true;
                }
            } else {
                // not a digit; reset
                if has_symbol_neighbour {
                    sum += value;
                }
                value = 0;
                has_symbol_neighbour = false;
                continue;
            }
        }

        if has_symbol_neighbour {
            sum += value;
        }
    }
    Ok(sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let gear_positions: HashSet<(usize, usize)> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.char_indices()
                .filter(|(_, c)| *c == '*')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    // maps position of gear to neighbour number value(s)
    let mut gears: HashMap<(usize, usize), HashSet<u32>> = HashMap::new();

    for (y, line) in INPUT.lines().enumerate() {
        let mut value = 0;
        let mut gear_neighbours = HashSet::new();

        for (x, ch) in line.char_indices() {
            if let Some(digit) = ch.to_digit(10) {
                value = value * 10 + digit;

                // search 8 neighbours for gears
                let xi = x as isize;
                let yi = y as isize;
                for (x, y) in [
                    (xi + 1, yi),
                    (xi + 1, yi + 1),
                    (xi, yi + 1),
                    (xi - 1, yi + 1),
                    (xi - 1, yi),
                    (xi - 1, yi - 1),
                    (xi, yi - 1),
                    (xi + 1, yi - 1),
                ]
                .into_iter()
                .filter_map(|(x, y)| {
                    // exclude out of bounds
                    let x: usize = x.try_into().ok()?;
                    let y: usize = y.try_into().ok()?;
                    Some((x, y))
                }) {
                    if gear_positions.contains(&(x, y)) {
                        gear_neighbours.insert((x, y));
                    }
                }
            } else {
                // not a digit; reset
                for gear in gear_neighbours.drain() {
                    gears.entry(gear).or_default().insert(value);
                }
                value = 0;
                continue;
            }
        }

        for gear in gear_neighbours {
            gears.entry(gear).or_default().insert(value);
        }
    }

    // sum of valid gears
    let mut sum = 0;
    for (_, neighbours) in gears {
        if neighbours.len() == 2 {
            sum += neighbours.into_iter().product::<u32>();
        }
    }

    Ok(sum.to_string())
}
