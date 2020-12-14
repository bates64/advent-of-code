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
    // idk lol
    Err(Error::Unimplemented)
}
