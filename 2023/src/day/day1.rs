use super::Error;

const INPUT: &str = include_str!("day1.txt");

pub fn part1() -> Result<String, Error> {
    let mut sum = 0;
    for line in INPUT.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10)).peekable();
        let first_digit = *digits.peek().ok_or(Error::NoSolution)?; // peek because first may also be last
        let last_digit = digits.last().ok_or(Error::NoSolution)?;
        sum += first_digit * 10 + last_digit;
    }
    Ok(sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut sum = 0;
    for line in INPUT.lines() {
        let mut state = String::new();
        let mut first_digit = None;
        let mut last_digit = None;

        for c in line.chars() {
            let mut n = c.to_digit(10);

            state.push(c);
            if n.is_none() {
                n = if state.ends_with("one") {
                    Some(1)
                } else if state.ends_with("two") {
                    Some(2)
                } else if state.ends_with("three") {
                    Some(3)
                } else if state.ends_with("four") {
                    Some(4)
                } else if state.ends_with("five") {
                    Some(5)
                } else if state.ends_with("six") {
                    Some(6)
                } else if state.ends_with("seven") {
                    Some(7)
                } else if state.ends_with("eight") {
                    Some(8)
                } else if state.ends_with("nine") {
                    Some(9)
                } else if state.ends_with("zero") {
                    Some(0)
                } else {
                    None
                };
            }

            if let Some(n) = n {
                if first_digit.is_none() {
                    first_digit = Some(n);
                }
                last_digit = Some(n);
            }
        }

        let first_digit = first_digit.ok_or(Error::NoSolution)?;
        let last_digit = last_digit.ok_or(Error::NoSolution)?;
        sum += first_digit * 10 + last_digit;
    }
    Ok(sum.to_string())
}
