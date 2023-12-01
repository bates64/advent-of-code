use std::process::exit;
use yansi::Paint;

mod day;

fn main() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let day = std::env::args().skip(1).next();
    let range = match day.unwrap_or_default().as_ref() {
        // All days by default
        "" => 1..=day::NUM_DAYS,

        // Just a specific day
        day => match day.parse::<usize>() {
            Ok(day) => day..=day,
            Err(error) => {
                eprintln!("could not parse day number ({})", error);
                exit(1);
            },
        },
    };

    let mut fail = false;

    for i in range {
        print!("day {:<2} ...  ", i);

        let (part1, part2) = match day::ALL_DAYS.get(i - 1) {
            Some(pair) => pair,
            _ => {
                println!("{}", Paint::red("unknown day?").bold());
                fail = true;
                continue;
            },
        };

        match part1() {
            Ok(solution) => print!("{}", Paint::magenta(solution).bold()),
            Err(error) => {
                print!("{}", Paint::red(error));
                fail = true;
            },
        }

        print!("  ...  ");

        match part2() {
            Ok(solution) => print!("{}", Paint::magenta(solution).bold()),
            Err(error) => {
                print!("{}", Paint::red(error));
                fail = true;
            },
        }

        print!("\n");
    }

    if fail {
        exit(1);
    }
}
