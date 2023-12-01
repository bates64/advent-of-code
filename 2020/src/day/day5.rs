use super::Error;

const INPUT: &str = include_str!("day5.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Seat(u32, u32);

impl Seat {
    pub fn from_boarding_pass(pass: &str) -> Self {
        let pass: Vec<char> = pass.chars().collect();
        assert_eq!(pass.len(), 10);

        let row = bsp(0, 127, 'F', 'B', &pass[0..7]);
        let col = bsp(0, 7, 'L', 'R', &pass[7..10]);

        Self(row, col)
    }

    pub fn id(&self) -> u32 {
        self.0 * 8 + self.1
    }
}

fn bsp(mut min: u32, mut max: u32, front: char, back: char, input: &[char]) -> u32 {
    for ch in input {
        let size: u32 = max - min + 1;

        if *ch == front {
            max -= size / 2;
        } else if *ch == back {
            min += size / 2;
        } else {
            panic!("invalid input char: {}", ch);
        }
    }

    assert_eq!(min, max);
    min
}

pub fn part1() -> Result<String, Error> {
    Ok(INPUT
        .lines()
        .map(|line| Seat::from_boarding_pass(line).id())
        .max()
        .unwrap()
        .to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut expected_id = None;
    for id in INPUT
        .lines()
        .map(|line| Seat::from_boarding_pass(line).id())
        .collect::<std::collections::BTreeSet<u32>>()
        .into_iter()
    {
        if let Some(expected_id) = expected_id {
            if id != expected_id {
                return Ok(expected_id.to_string());
            }
        }
        expected_id = Some(id + 1);
    }

    // Initial solution:
    /*
    let seat_ids: std::collections::BTreeSet<u32> = INPUT
        .lines()
        .map(|line| Seat::from_boarding_pass(line).id())
        .collect();

    let min = *seat_ids.iter().next().unwrap();
    let max = *seat_ids.iter().rev().next().unwrap();

    for i in min..max {
        if seat_ids.get(&i).is_none() {
            return Ok(i.to_string());
        }
    }
    */

    Err(Error::NoSolution)
}

#[test]
fn test_seat_calc() {
    assert_eq!(Seat::from_boarding_pass("FBFBBFFRLR"), Seat(44, 5));
    assert_eq!(Seat::from_boarding_pass("BFFFBBFRRR"), Seat(70, 7));
    assert_eq!(Seat::from_boarding_pass("FFFBBBFRRR"), Seat(14, 7));
    assert_eq!(Seat::from_boarding_pass("BBFFBBFRLL"), Seat(102, 4));
}
