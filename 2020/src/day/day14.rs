use super::Error;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

const INPUT: &str = include_str!("day14.txt");

#[derive(Recap, Deserialize, Debug)]
#[recap(regex=r#"(?x)
    mem\[
    (?P<addr>\d+)
    \]\s=\s
    (?P<value>\d+)
"#)]
struct MemSet {
    addr: u64,
    value: u64,
}

#[derive(Debug, Clone, Copy)]
enum MaskBit {
    One,
    Zero,
    X,
}

pub fn part1() -> Result<String, Error> {
    let mut mask = [MaskBit::X; 36];
    let mut mem = HashMap::new();

    for line in INPUT.lines() {
        if line == "" {
            continue;
        }

        if let Some (mask_s) = line.strip_prefix("mask = ") {
            for (s, b) in mask_s.chars().zip(mask.iter_mut()) {
                *b = match s {
                    '0' => MaskBit::Zero,
                    '1' => MaskBit::One,
                    'X' => MaskBit::X,
                    _ => panic!("bad input"),
                };
            }
        } else {
            let mut set: MemSet = line.parse().unwrap();

            for (i, b) in mask.iter().enumerate() {
                match b {
                    MaskBit::One => set.value |= 0b100000000000000000000000000000000000 >> i,
                    MaskBit::Zero => set.value &= !(0b100000000000000000000000000000000000 >> i),
                    MaskBit::X => continue,
                }
            }

            mem.insert(set.addr, set.value);
        }
    }

    let sum: u64 = mem.values().sum();
    Ok(sum.to_string())
}

pub fn part2() -> Result<String, Error> {
    let mut mask = [MaskBit::X; 36];
    let mut mem = HashMap::new();

    for line in INPUT.lines() {
        if line == "" {
            continue;
        }

        if let Some (mask_s) = line.strip_prefix("mask = ") {
            for (s, b) in mask_s.chars().zip(mask.iter_mut()) {
                *b = match s {
                    '0' => MaskBit::Zero,
                    '1' => MaskBit::One,
                    'X' => MaskBit::X,
                    _ => panic!("bad input"),
                };
            }
        } else {
            let set: MemSet = line.parse().unwrap();

            for addr in apply_mask_v2(set.addr, &mask, 0) {
                mem.insert(addr, set.value);
            }
        }
    }

    let sum: u64 = mem.values().sum();
    Ok(sum.to_string())
}

fn apply_mask_v2(mut addr: u64, mask: &[MaskBit; 36], i: usize) -> Vec<u64> {
    for (i, b) in mask.iter().enumerate().skip(i) {
        match b {
            MaskBit::One => addr |= 0b100000000000000000000000000000000000 >> i,
            MaskBit::Zero => (),
            MaskBit::X => {
                // one
                let mut new_addr = addr.clone();
                new_addr |= 0b100000000000000000000000000000000000 >> i; // set
                let mut vec = apply_mask_v2(new_addr, &mask, i + 1);

                // zero
                let mut new_addr = addr.clone();
                new_addr &= !(0b100000000000000000000000000000000000 >> i); // unset
                vec.extend(apply_mask_v2(new_addr, &mask, i + 1));

                return vec;
            },
        };
    }

    vec![addr]
}
