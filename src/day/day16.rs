use recap::Recap;
use serde::Deserialize;
use super::Error;

const INPUT: &str = include_str!("day16.txt");

#[derive(Recap, Deserialize, Clone)]
#[recap(regex=r#"(?x)
    (?P<field>[^:]+)
    :\s
    (?P<lo_1>\d+)
    -
    (?P<hi_1>\d+)
    \sor\s
    (?P<lo_2>\d+)
    -
    (?P<hi_2>\d+)
"#)]
struct Requirement {
    field: String,

    lo_1: u32,
    hi_1: u32, // inclusive

    lo_2: u32,
    hi_2: u32,
}

fn is_valid(n: u32, requirements: &[Requirement]) -> bool {
    for req in requirements {
        let r1 = req.lo_1..=req.hi_1;
        let r2 = req.lo_2..=req.hi_2;


        if r1.contains(&n) || r2.contains(&n) {
            return true;
        }
    }

    false
}

pub fn part1() -> Result<String, Error> {
    let mut lines = INPUT.lines();

    let mut requirements: Vec<Requirement> = Vec::with_capacity(20);
    loop {
        let line = lines.next().unwrap();

        if line == "" {
            break;
        }

        requirements.push(line.parse().unwrap());
    }

    assert_eq!(lines.next(), Some("your ticket:"));

    lines.next(); // your ticket

    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));

    let invalid_sum: u32 = lines
        .map(|line| -> u32 {
            line
                .split(',')
                .map(|s| s.parse().unwrap())
                .map(|n| {
                    if is_valid(n, &requirements) {
                        0
                    } else {
                        n
                    }
                })
                .sum()
        })
        .sum();

    Ok(invalid_sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut lines = INPUT.lines();

    let mut requirements: Vec<Requirement> = Vec::with_capacity(20);
    loop {
        let line = lines.next().unwrap();

        if line == "" {
            break;
        }

        requirements.push(line.parse().unwrap());
    }

    assert_eq!(lines.next(), Some("your ticket:"));

    let your_ticket: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));

    let tickets: Vec<Vec<u32>> = lines
        .map(|line| -> Vec<u32> {
            line
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .filter(|ticket| {
            // Drop tickets with invalid fields
            ticket
                .iter()
                .find(|n| !is_valid(**n, &requirements))
                .is_none()
        })
        .collect();

    let mut fields: Vec<Vec<Requirement>> = vec![requirements; your_ticket.len()];

    loop {
        let (unknown, known): (Vec<_>, Vec<_>) = fields
            .iter_mut()
            .enumerate()
            .partition(|(_, reqs)| reqs.len() > 1);

        if unknown.len() == 0 {
            break;
        }

        let known: Vec<&Requirement> = known
            .into_iter()
            .map(|(_, reqs)| &reqs[0])
            .collect();

        for (idx, reqs) in unknown {
            reqs.retain(|req| {
                // Drop if known
                if known.iter().find(|k| k.field == req.field).is_some() {
                    return false;
                }

                for ticket in &tickets {
                    let n = ticket[idx];

                    let r1 = req.lo_1..=req.hi_1;
                    let r2 = req.lo_2..=req.hi_2;

                    if !r1.contains(&n) && !r2.contains(&n) {
                        return false;
                    }
                }

                true
            });
        }
    }

    Ok(fields
        .iter()
        .enumerate()
        .map(|(idx, reqs)| (idx, &reqs[0]))
        .filter(|(_, req)| req.field.starts_with("departure "))
        .map(|(idx, _)| your_ticket[idx] as u64)
        .product::<u64>()
        .to_string())
}
