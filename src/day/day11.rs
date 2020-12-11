use super::Error;

const INPUT: &str = include_str!("day11.txt");

#[derive(Clone, Copy, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn get_seat(grid: &Vec<Vec<Seat>>, x: isize, y: isize) -> Option<Seat> {
    match grid.get(y as usize) {
        Some(row) => match row.get(x as usize) {
            Some(seat) => Some(seat.clone()),
            None => None,
        },
        None => None,
    }
}

fn print_grid(grid: &Vec<Vec<Seat>>) {
    for row in grid {
        for seat in row {
            print!("{}", match seat {
                Seat::Empty => "L",
                Seat::Occupied => "#",
                Seat::Floor => ".",
            });
        }
        print!("\n");
    }
}

pub fn part1() -> Result<String, Error> {
    let mut grid: Vec<Vec<Seat>> = INPUT.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _   => panic!("bad input"),
                })
                .collect()
        })
        .collect();

    loop {
        let mut changed = false;
        let mut next_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let adjacent = {
                    let x = x as isize;
                    let y = y as isize;
                    [
                        get_seat(&grid, x - 1, y - 1),
                        get_seat(&grid, x - 0, y - 1),
                        get_seat(&grid, x + 1, y - 1),

                        get_seat(&grid, x - 1, y),
                        get_seat(&grid, x + 1, y),

                        get_seat(&grid, x - 1, y + 1),
                        get_seat(&grid, x - 0, y + 1),
                        get_seat(&grid, x + 1, y + 1),
                    ]
                };

                let seat = grid[y][x];

                match seat {
                    Seat::Floor => (),
                    Seat::Empty => {
                        let mut occupied = false;
                        for s in adjacent.iter() {
                            if let Some(Seat::Occupied) = s {
                                occupied = true;
                                break;
                            }
                        }

                        if !occupied {
                            next_grid[y][x] = Seat::Occupied;
                            changed = true;
                        }
                    },
                    Seat::Occupied => {
                        let mut occupied = 0;
                        for s in adjacent.iter() {
                            if let Some(Seat::Occupied) = s {
                                occupied += 1;
                            }
                        }

                        if occupied >= 4 {
                            next_grid[y][x] = Seat::Empty;
                            changed = true;
                        }
                    },
                }
            }
        }

        grid = next_grid;

        //print!("\n");
        //print_grid(&grid);

        if !changed {
            break;
        }
    }

    let mut occupied = 0;
    for row in grid {
        for seat in row {
            if let Seat::Occupied = seat {
                occupied += 1;
            }
        }
    }

    Ok(occupied.to_string())
}

fn visible_seat(grid: &Vec<Vec<Seat>>, mut x: isize, mut y: isize, stride: (isize, isize)) -> Option<Seat> {
    loop {
        x += stride.0;
        y += stride.1;
        match get_seat(grid, x, y) {
            Some(Seat::Floor) => (),
            Some(seat) => return Some(seat),
            None => return None,
        }
    }
}

pub fn part2() -> Result<String, Error> {
    let mut grid: Vec<Vec<Seat>> = INPUT.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _   => panic!("bad input"),
                })
                .collect()
        })
        .collect();

    loop {
        let mut changed = false;
        let mut next_grid = grid.clone();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let adjacent = {
                    let x = x as isize;
                    let y = y as isize;
                    [
                        visible_seat(&grid, x, y, (-1, -1)),
                        visible_seat(&grid, x, y, ( 0, -1)),
                        visible_seat(&grid, x, y, ( 1, -1)),

                        visible_seat(&grid, x, y, (-1,  0)),
                        visible_seat(&grid, x, y, ( 1,  0)),

                        visible_seat(&grid, x, y, (-1,  1)),
                        visible_seat(&grid, x, y, ( 0,  1)),
                        visible_seat(&grid, x, y, ( 1,  1)),
                    ]
                };

                let seat = grid[y][x];

                match seat {
                    Seat::Floor => (),
                    Seat::Empty => {
                        let mut occupied = false;
                        for s in adjacent.iter() {
                            if let Some(Seat::Occupied) = s {
                                occupied = true;
                                break;
                            }
                        }

                        if !occupied {
                            next_grid[y][x] = Seat::Occupied;
                            changed = true;
                        }
                    },
                    Seat::Occupied => {
                        let mut occupied = 0;
                        for s in adjacent.iter() {
                            if let Some(Seat::Occupied) = s {
                                occupied += 1;
                            }
                        }

                        if occupied >= 5 {
                            next_grid[y][x] = Seat::Empty;
                            changed = true;
                        }
                    },
                }
            }
        }

        grid = next_grid;

        //print!("\n");
        //print_grid(&grid);

        if !changed {
            break;
        }
    }

    let mut occupied = 0;
    for row in grid {
        for seat in row {
            if let Seat::Occupied = seat {
                occupied += 1;
            }
        }
    }

    Ok(occupied.to_string())
}
