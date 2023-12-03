use super::Error;

const INPUT: &str = include_str!("day2.txt");

pub fn part1() -> Result<String, Error> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;
    for line in INPUT.lines() {
        let mut is_game_valid = true;

        let colon = line.find(':').expect("line missing colon");
        let game = &line["Game ".len()..colon]
            .parse::<u32>()
            .expect("bad game number");

        for round in line[colon + 2..].split("; ") {
            for part in round.split(", ") {
                let space = part.find(' ').expect("part missing space");
                let count = part[..space].parse::<u32>().expect("bad count");
                let color = &part[space + 1..];

                let is_part_valid = match color {
                    "red" => count <= max_red,
                    "green" => count <= max_green,
                    "blue" => count <= max_blue,
                    _ => panic!("bad color {color}"),
                };
                if !is_part_valid {
                    is_game_valid = false;
                }
            }
        }

        if is_game_valid {
            sum += game;
        }
    }
    Ok(sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut sum = 0;
    for line in INPUT.lines() {
        let colon = line.find(':').expect("line missing colon");
        //let game = &line["Game ".len()..colon].parse::<u32>().expect("bad game number");

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for round in line[colon + 2..].split("; ") {
            for part in round.split(", ") {
                let space = part.find(' ').expect("part missing space");
                let count = part[..space].parse::<u32>().expect("bad count");
                let color = &part[space + 1..];

                match color {
                    "red" => max_red = max_red.max(count),
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    _ => panic!("bad color {color}"),
                };
            }
        }

        sum += max_red * max_green * max_blue;
    }
    Ok(sum.to_string())
}
