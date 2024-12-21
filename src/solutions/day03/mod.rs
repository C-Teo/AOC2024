use crate::utils::read_input;

pub fn part_one() -> i32 {
    let input: String = input();

    let mut res: i32 = 0;
    let mut index: usize = 0;

    while index < input.len() {
        if index + 4 < input.len() {
            if &input[index..index + 4] != "mul(" {
                index += 1;
                continue;
            }
        } else {
            break;
        }
        index += 4;

        let val1: i32;
        match parse_num(&input, index) {
            Ok((value, idx)) => {
                val1 = value;
                index = idx;
            }
            Err(idx) => {
                index = idx;
                continue;
            }
        }

        if &input[index..=index] != "," {
            continue;
        }
        index += 1;

        let val2: i32;
        match parse_num(&input, index) {
            Ok((value, idx)) => {
                val2 = value;
                index = idx;
            }
            Err(idx) => {
                index = idx;
                continue;
            }
        }

        if &input[index..=index] != ")" {
            continue;
        }
        index += 1;

        res += val1 * val2;
    }

    res
}

fn parse_num(input: &str, mut index: usize) -> Result<(i32, usize), usize> {
    let idx: usize = index;

    if !input[idx..=idx].parse::<i32>().is_ok() {
        return Err(idx);
    }

    while (&input)[idx..=index].parse::<i32>().is_ok() {
        index += 1;
        if index == input.len() {
            return Err(index);
        }
    }

    return Ok((input[idx..index].parse().unwrap(), index));
}

fn input() -> String {
    let val: String = read_input::run(3).expect("Failed to read file!");
    return val;
}
