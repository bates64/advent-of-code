use recap::Recap;
use serde::Deserialize;
use super::Error;
use std::collections::HashSet;

const INPUT: &str = include_str!("day8.txt");

#[derive(Recap, Deserialize, Debug, Clone)]
#[recap(regex=r#"(?x)
    (?P<op>\S+)
    \s
    (?P<arg>[+-]\d+)
"#)]
struct Instruction {
    op: String,
    arg: i32,
}

fn execute(program: Vec<Instruction>) -> (i32, bool) {
    let mut pos = 0i32;
    let mut seen = HashSet::new();
    let mut acc = 0;

    while seen.insert(pos) {
        if pos as usize >= program.len() {
            return (acc, true);
        }

        let inst = &program[pos as usize];

        match inst.op.as_ref() {
            "jmp" => pos += inst.arg,
            "acc" => {
                acc += inst.arg;
                pos += 1;
            },
            "nop" => pos += 1,
            _ => panic!(),
        }
    }

    (acc, false)
}

pub fn part1() -> Result<String, Error> {
    let program: Vec<_> = INPUT.lines()
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect();

    let (acc, terminated) = execute(program);
    assert!(!terminated);

    Ok(acc.to_string())
}

pub fn part2() -> Result<String, Error> {
    let program: Vec<_> = INPUT.lines()
    .filter_map(|line| line.parse::<Instruction>().ok())
    .collect();

    let mut pos = 0;

    loop {
        let mut new_program = program.clone();

        // Change a jmp instruction to nop
        loop {
            pos += 1;
            if new_program[pos].op == "jmp" {
                new_program[pos].op = "nop".to_owned();
                break;
            }
        }

        let (acc, terminated) = execute(new_program);

        if terminated {
            return Ok(acc.to_string());
        }
    }
}
