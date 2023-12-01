use super::Error;
use recap::Recap;
use serde::Deserialize;

const INPUT: &str = include_str!("day12.txt");

#[derive(Recap, Deserialize)]
#[recap(regex=r#"(?x)
    (?P<command>\S)
    (?P<value>\d+)
"#)]
struct Line {
    command: char,
    value: i32,
}

pub fn part1() -> Result<String, Error> {
    #[derive(Debug)]
    struct Ship {
        x: i32,
        y: i32,
        dir: Dir,
    }

    impl Ship {
        fn distance(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }
    }

    #[derive(Debug)]
    enum Dir {
        N, E, S, W,
}

    Ok(INPUT.lines()
        .filter_map(|line| line.parse::<Line>().ok())
        .fold(Ship { x: 0, y: 0, dir: Dir::E }, |mut ship, Line { command, value }| {
            match command {
                'N' => ship.y += value,
                'S' => ship.y -= value,
                'E' => ship.x += value,
                'W' => ship.x -= value,
                'L' => {
                    assert_eq!(value % 90, 0);
                    for _ in 0..(value / 90) {
                        ship.dir = match ship.dir {
                            Dir::N => Dir::W,
                            Dir::W => Dir::S,
                            Dir::S => Dir::E,
                            Dir::E => Dir::N,
                        }
                    }
                },
                'R' => {
                    assert_eq!(value % 90, 0);
                    for _ in 0..(value / 90) {
                        ship.dir = match ship.dir {
                            Dir::W => Dir::N,
                            Dir::S => Dir::W,
                            Dir::E => Dir::S,
                            Dir::N => Dir::E,
                        }
                    }
                },
                'F' => match ship.dir {
                    Dir::E => ship.x += value,
                    Dir::S => ship.y -= value,
                    Dir::W => ship.x -= value,
                    Dir::N => ship.y += value,
                },
                _ => panic!("bad input"),
            }

            ship
        })
        .distance()
        .to_string())
}

pub fn part2() -> Result<String, Error> {
    #[derive(Debug)]
    struct Ship {
        x: i32,
        y: i32,
        wx: i32,
        wy: i32,
    }

    impl Ship {
        fn distance(&self) -> i32 {
            self.x.abs() + self.y.abs()
        }
    }

    Ok(INPUT.lines()
        .filter_map(|line| line.parse::<Line>().ok())
        .fold(Ship { x: 0, y: 0, wx: 10, wy: 1 }, |mut ship, Line { command, value }| {
            match command {
                'N' => ship.wy += value,
                'S' => ship.wy -= value,
                'E' => ship.wx += value,
                'W' => ship.wx -= value,
                'L' => {
                    assert_eq!(value % 90, 0);
                    for _ in 0..(value / 90) {
                        // (1, 2) -> (-2, 1)
                        let Ship { wx, wy, .. } = ship;
                        ship.wx = -wy;
                        ship.wy = wx;
                    }
                },
                'R' => {
                    assert_eq!(value % 90, 0);
                    for _ in 0..(value / 90) {
                        // (-4, 3) -> (3, 4)
                        let Ship { wx, wy, .. } = ship;
                        ship.wx = wy;
                        ship.wy = -wx;
                    }
                },
                'F' => {
                    ship.x += ship.wx * value;
                    ship.y += ship.wy * value;
                },
                _ => panic!("bad input"),
            }

            ship
        })
        .distance()
        .to_string())
}
