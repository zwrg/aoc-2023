use regex::Regex;

pub fn run(input: &str) -> (u32, u32) {
    let lines = input.lines();
    let game_number_regex = Regex::new(r"Game (\d+):").unwrap();
    let red_regex = Regex::new(r"(?P<number>\d+) red").unwrap();
    let blue_regex = Regex::new(r"(?P<number>\d+) blue").unwrap();
    let green_regex = Regex::new(r"(?P<number>\d+) green").unwrap();

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for line in lines {
        let mut is_possible = true;
        let mut game_number = "";

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        if let Some(capture) = game_number_regex.captures(line) {
            game_number = capture.get(1).unwrap().as_str();
        }

        for capture in green_regex.captures_iter(line) {
            if let Some(group) = capture.name("number") {
                let number = group.as_str().parse::<u32>().unwrap();
                // println!("{:?}; {}; {}", capture, number, !(number > green_limit));
                if number > green_limit {
                    is_possible = false;
                }

                if max_green < number {
                    max_green = number;
                }
            }
        }

        for capture in red_regex.captures_iter(line) {
            if let Some(group) = capture.name("number") {
                let number = group.as_str().parse::<u32>().unwrap();
                // println!("{:?}; {}; {}", capture, number, !(number > green_limit));
                if number > red_limit {
                    is_possible = false;
                }

                if max_red < number {
                    max_red = number;
                }
            }
        }

        for capture in blue_regex.captures_iter(line) {
            if let Some(group) = capture.name("number") {
                let number = group.as_str().parse::<u32>().unwrap();
                // println!("{:?}; {}; {}", capture, number, !(number > green_limit));
                if number > blue_limit {
                    is_possible = false;
                }

                if max_blue < number {
                    max_blue = number;
                }
            }
        }

        if is_possible {
            sum_part1 += game_number.parse::<u32>().unwrap();
        }

        sum_part2 += max_blue * max_green * max_red;
    }

    return (sum_part1, sum_part2);
}
