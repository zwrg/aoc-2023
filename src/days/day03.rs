use std::vec;

fn is_engine_char(ch: &char) -> bool {
    if *ch == '.' {
        return false;
    } else {
        return ch.to_digit(10).is_none();
    }
}

fn is_engine_gear(ch: &char) -> bool {
    return *ch == '*';
}

fn find_gear_vector(
    coords_vec: &mut Vec<((usize, usize), Vec<u32>)>,
    target_coords: (usize, usize),
) -> Option<&mut Vec<u32>> {
    for (coords, numbers) in coords_vec.iter_mut() {
        if *coords == target_coords {
            return Some(numbers);
        }
    }
    None
}

fn is_part_of_schematic(
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    m: usize,
    n: usize,
) -> (bool, (usize, usize)) {
    if i != 0 && j != 0 && is_engine_char(&matrix[i - 1][j - 1]) {
        return (true, (i - 1, j - 1));
    } else if i != 0 && is_engine_char(&matrix[i - 1][j]) {
        return (true, (i - 1, j));
    } else if i != 0 && j < n - 1 && is_engine_char(&matrix[i - 1][j + 1]) {
        return (true, (i - 1, j + 1));
    } else if j != 0 && is_engine_char(&matrix[i][j - 1]) {
        return (true, (i, j - 1));
    } else if j < n - 1 && is_engine_char(&matrix[i][j + 1]) {
        return (true, (i, j + 1));
    } else if i < m - 1 && j != 0 && is_engine_char(&matrix[i + 1][j - 1]) {
        return (true, (i + 1, j - 1));
    } else if i < m - 1 && is_engine_char(&matrix[i + 1][j]) {
        return (true, (i + 1, j));
    } else if i < m - 1 && j < n - 1 && is_engine_char(&matrix[i + 1][j + 1]) {
        return (true, (i + 1, j + 1));
    } else {
        return (false, (0, 0));
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut v: Vec<char> = Vec::new();
        for ch in line.chars() {
            v.push(ch);
        }
        matrix.push(v);
    }

    let mut sum = 0;
    let mut gear_info: Vec<((usize, usize), Vec<u32>)> = Vec::new();

    for (i, v) in matrix.iter().enumerate() {
        let mut append_to_number = false;
        let mut is_engine_part = false;
        let mut number = 0;
        let mut current_gear_coords: (usize, usize) = (0, 0);

        print!("[{},] ", i + 1);
        for (j, ch) in v.iter().enumerate() {
            match ch.to_digit(10) {
                Some(value) => {
                    if append_to_number {
                        number = number * 10 + value;
                    } else {
                        number = value;
                    }
                    append_to_number = true;

                    let schema = is_part_of_schematic(&matrix, i, j, matrix.len(), matrix[i].len());
                    if !is_engine_part && schema.0 {
                        is_engine_part = true;
                    }

                    let is_gear = is_engine_gear(&matrix[schema.1 .0][schema.1 .1]);
                    if is_gear {
                        current_gear_coords = (schema.1 .0, schema.1 .1)
                    }

                    if j == matrix[i].len() - 1 {
                        if is_engine_part {
                            sum += number;
                            print!("{number}/{sum} # ");
                        }

                        if let Some(numbers) = find_gear_vector(&mut gear_info, current_gear_coords)
                        {
                            if number != 0 {
                                numbers.push(number)
                            }
                        } else {
                            if current_gear_coords != (0, 0) {
                                gear_info.push((current_gear_coords, vec![number]))
                            }
                        }

                        number = 0;
                        append_to_number = false;
                        is_engine_part = false;
                        current_gear_coords = (0, 0);
                        if j == matrix[i].len() - 1 {
                            println!("; => {}", sum);
                        }
                    }
                }
                None => {
                    // dot_char or engine_char
                    if is_engine_part {
                        sum += number;
                        print!("{number}/{sum} # ");
                    }

                    if let Some(numbers) = find_gear_vector(&mut gear_info, current_gear_coords) {
                        if number != 0 {
                            numbers.push(number)
                        }
                    } else {
                        if current_gear_coords != (0, 0) {
                            gear_info.push((current_gear_coords, vec![number]))
                        }
                    }

                    number = 0;
                    append_to_number = false;
                    is_engine_part = false;
                    current_gear_coords = (0, 0);
                    if j == matrix[i].len() - 1 {
                        println!("; => {}", sum);
                    }
                    continue;
                }
            };

            if j == matrix[i].len() - 1 {
                println!("; == {}", sum);
            }
        }
    }

    let mut sum_2 = 0;
    println!("{:?}", gear_info);
    for (coords, gear) in gear_info {
        if gear.len() == 2 {
            println!("{:?}, {:?}", coords, gear);
            let mut product = 1;
            for number in gear.iter() {
                product *= number;
            }
            sum_2 += product;
        }
    }

    return (sum, sum_2);
}
