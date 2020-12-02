use super::Error;

const INPUT: &str = include_str!("day1.txt");

pub fn part1() -> Result<String, Error> {
    for line in INPUT.lines() {
        let a: u32 = line.parse()?;

        for line in INPUT.lines() {
            let b: u32 = line.parse()?;

            if a + b == 2020 {
                return Ok(format!("{}", a * b))
            }
        }
    }

    Err(Error::NoSolution)
}

pub fn part2() -> Result<String, Error> {
    for line in INPUT.lines() {
        let a: u32 = line.parse()?;

        for line in INPUT.lines() {
            let b: u32 = line.parse()?;

            for line in INPUT.lines() {
                let c: u32 = line.parse()?;

                if a + b + c == 2020 {
                    return Ok(format!("{}", a * b * c))
                }
            }
        }
    }

    Err(Error::NoSolution)
}
