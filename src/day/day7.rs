use super::Error;
use std::collections::HashMap;
use std::cell::Cell;

const INPUT: &str = include_str!("day7.txt");

pub fn part1() -> Result<String, Error> {
    #[derive(Debug)]
    struct BagDesc {
        pub color: String,
        pub inside: HashMap<String, u32>,
        has_gold: Cell<Option<bool>>,
    }

    impl BagDesc {
        fn resolve_has_gold(&self, bags: &HashMap<String, BagDesc>) {
            if self.has_gold.get().is_some() {
                
            } else {
                for (color, _) in &self.inside {
                    match bags.get(color).unwrap().has_gold.get() {
                        Some(true) => {
                            self.has_gold.set(Some(true));
                            return; // It's a yes
                        },
                        Some(false) => (),
                        None => return, // Dependencies not resolved
                    }
                }

                // No dependency has gold
                self.has_gold.set(Some(false));
            }
        }
    }

    let mut bags = HashMap::new();

    // This is gross
    for line in INPUT.lines() {
        let (color, within) = line.split_at(line.find(" bags contain ").unwrap());
        let color = color.to_owned();
        let within = within
            .strip_prefix(" bags contain ").unwrap()
            .strip_suffix(".").unwrap()
            .to_owned();

        bags.insert(color.to_owned(), BagDesc {
            color: color.to_owned(),
            has_gold: if color == "shiny gold" {
                Cell::new(Some(true))
            } else {
                Cell::new(None)
            },
            inside: if within == "no other bags" {
                HashMap::new()
            } else {
                within.split(", ")
                    .map(|color| {
                        let (n, color) = color
                            .strip_suffix("s").unwrap_or(color)
                            .strip_suffix(" bag").unwrap()
                            .split_at(color.find(" ").unwrap());

                        (
                            color
                                .strip_prefix(" ")
                                .unwrap()
                                .to_owned(),
                            n.parse().unwrap(),
                        )
                    })
                    .collect()
            },
        });
    }

    while {
        bags.iter()
            .find(|(_, bag)| bag.has_gold.get().is_none())
            .is_some()
    } {
        for (_, bag) in &bags {
            bag.resolve_has_gold(&bags);
        }
    }

    Ok(
        bags.into_iter()
            .filter(|(_, bag)| bag.color != "shiny gold" && bag.has_gold.get().unwrap() == true)
            .count()
            .to_string()
    )
}

pub fn part2() -> Result<String, Error> {
    #[derive(Debug)]
    struct BagDesc {
        pub color: String,
        pub inside: HashMap<String, u64>,
        inside_count: Cell<Option<u64>>,
    }

    impl BagDesc {
        fn resolve_inside_count(&self, bags: &HashMap<String, BagDesc>) {
            if self.inside_count.get().is_some() {
                
            } else {
                let mut count = 1; // include self

                for (color, multi) in &self.inside {
                    match bags.get(color).unwrap().inside_count.get() {
                        Some(n) => count += multi * n,
                        None => return, // Dependencies not resolved
                    }
                }

                self.inside_count.set(Some(count));
            }
        }
    }

    let mut bags = HashMap::new();

    // This is gross
    for line in INPUT.lines() {
        let (color, within) = line.split_at(line.find(" bags contain ").unwrap());
        let color = color.to_owned();
        let within = within
            .strip_prefix(" bags contain ").unwrap()
            .strip_suffix(".").unwrap()
            .to_owned();

        bags.insert(color.to_owned(), BagDesc {
            color: color.to_owned(),
            inside_count: Cell::new(None),
            inside: if within == "no other bags" {
                HashMap::new()
            } else {
                within.split(", ")
                    .map(|color| {
                        let (n, color) = color
                            .strip_suffix("s").unwrap_or(color)
                            .strip_suffix(" bag").unwrap()
                            .split_at(color.find(" ").unwrap());

                        (
                            color
                                .strip_prefix(" ")
                                .unwrap()
                                .to_owned(),
                            n.parse().unwrap(),
                        )
                    })
                    .collect()
            },
        });
    }

    while {
        bags.iter()
            .find(|(_, bag)| bag.inside_count.get().is_none())
            .is_some()
    } {
        for (_, bag) in &bags {
            bag.resolve_inside_count(&bags);
        }
    }

    Ok(
        (bags
            .get("shiny gold").unwrap()
            .inside_count.get().unwrap() - 1)
            .to_string()
    )
}
