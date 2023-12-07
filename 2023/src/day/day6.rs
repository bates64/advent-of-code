use super::Error;

const INPUT: &str = include_str!("day6.txt");

pub fn part1() -> Result<String, Error> {
    let data: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let product: u32 = data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(&time, &record_distance)| {
            /*
            let mut wins = 0;
            for speed in 1..time {
                let distance = speed * (time - speed);
                if distance > record_distance {
                    wins += 1;
                }
            }
            wins
            */
            let (x1, x2) = solve_quadratic(time as f64, record_distance as f64);
            x1 as u32 - x2 as u32
        })
        .product();
    Ok(product.to_string())
}

pub fn part2() -> Result<String, Error> {
    let data: Vec<u64> = INPUT
        .lines()
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .replace(" ", "")
                .parse()
                .unwrap()
        })
        .collect();
    let t = data[0];
    let y = data[1];

    let (x1, x2) = solve_quadratic(t as f64, y as f64);

    let difference = x1 as u64 - x2 as u64;
    Ok(difference.to_string())
}

// https://www.desmos.com/calculator/csqxghr3nm
// we have the quadratic y=x(t-x)
// in standard form, this is x^2 - tx + y = 0
fn solve_quadratic(t: f64, y: f64) -> (f64, f64) {
    let discriminant = t.powi(2) - 4.0 * y;
    if discriminant < 0.0 {
        panic!("no real solutions");
    }
    let sqrt_discriminant = discriminant.sqrt();
    let x1 = (t + sqrt_discriminant) / 2.0;
    let x2 = (t - sqrt_discriminant) / 2.0;
    (x1, x2)
}
