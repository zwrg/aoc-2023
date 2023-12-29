use indexmap::IndexMap;

pub fn run(input: &str) -> (u32, u32) {
    let dict: IndexMap<&str, &str> = IndexMap::from([
        ("eightwo", "8two"),
        ("eighthree", "8three"),
        ("nineight", "9eight"),
        ("zerone", "0one"),
        ("fiveight", "5eight"),
        ("threeight", "3eight"),
        ("twone", "2one"),
        ("oneight", "1eight"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let lines = input.lines();
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for line1 in lines.clone() {
        let mut first_digit = 0;
        let mut last_digit = 0;

        for (_, c) in line1.chars().enumerate() {
            match c.to_digit(10) {
                Some(x) => {
                    if first_digit == 0 {
                        first_digit = x;
                    }
                    last_digit = x;
                }
                None => {}
            }
        }
        sum_p1 += first_digit * 10 + last_digit;
    }

    for line2 in lines.clone() {
        let mut first_digit = 0;
        let mut last_digit = 0;

        let mut result = String::from(line2);
        for (key, value) in dict.iter() {
          result = result.replace(key, value);
        }

        for (_, c) in result.chars().enumerate() {
            match c.to_digit(10) {
                Some(x) => {
                    if first_digit == 0 {
                        first_digit = x;
                    }
                    last_digit = x;
                }
                None => {}
            }
        }
        sum_p2 += first_digit * 10 + last_digit;
    }

    return (sum_p1, sum_p2);
}
