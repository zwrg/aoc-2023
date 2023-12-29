use std::{env, fs};

pub mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (2..=2).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };

    for day in &days {
        println!("Day: {}", day);
        let path = format!("inputs/day{:02}", day);
        let file = format!("{0}.txt", path);
        let test_file = format!("{0}-{1}.txt", path, "test");

        let paths = [file, test_file];

        for path in paths.iter() {
            // println!("Index: {}, Value: {}", index, path);
            let input = fs::read_to_string(path);

            if let Ok(input) = input {
                let input = input.trim_end();
                let day_fn = match day {
                    1 => days::day01::run,
                    2 => days::day02::run,
                    _ => unreachable!(),
                };
                let result = day_fn(input);
                println!("{},{:?}", path, result);
            } else {
                println!("ERROR: no data");
            }
        }
    }
}
