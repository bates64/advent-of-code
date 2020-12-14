use super::Error;

const INPUT: &str = include_str!("day13.txt");

pub fn part1() -> Result<String, Error> {
    let mut lines = INPUT.lines();

    let time: u64 = lines.next().unwrap().parse().unwrap();

    let (id, delta) = lines.next().unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .map(|id| (id, id - (time % id)))
        .min_by_key(|(_id, delta)| *delta)
        .unwrap();

    Ok((id * delta).to_string())
}

pub fn part2() -> Result<String, Error> {
    let buses: Vec<(usize, usize)> = INPUT.lines()
        .skip(1)
        .next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(delta, s)| (delta, s.parse().unwrap()))
        .collect();

    let mut time = 0;
    let mut stride = 1;

    for (delta, id) in buses {
        loop {
            if (time + delta) % id == 0 {
                stride = lcm(id, stride);
                break;
            }

            time += stride;
        }
    }

    Ok(time.to_string())
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}
