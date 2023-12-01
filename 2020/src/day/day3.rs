use super::Error;

const INPUT: &str = include_str!("day3.txt");

fn input_grid() -> Vec<Vec<char>> {
    INPUT.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn traverse(grid: &Vec<Vec<char>>, slope: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut tree_count = 0;

    loop {
        match grid.get(pos.1).map(|row| row.get(pos.0 % row.len())) {
            Some(Some(&cell)) => {
                if cell == '#' {
                    tree_count += 1;
                }

                pos.0 += slope.0;
                pos.1 += slope.1;
            },
            _ => return tree_count,
        }
    }
}

pub fn part1() -> Result<String, Error> {
    let grid = input_grid();
    Ok(traverse(&grid, (3, 1)).to_string())
}

pub fn part2() -> Result<String, Error> {
    let grid = input_grid();
    Ok((
        traverse(&grid, (1, 1)) *
        traverse(&grid, (3, 1)) *
        traverse(&grid, (5, 1)) *
        traverse(&grid, (7, 1)) *
        traverse(&grid, (1, 2))
    ).to_string())
}
