use crate::utils::read_input;
use std::cmp::Ordering;

#[derive(Clone)]
enum Order {
    INC,
    DEC,
}

pub fn part_one() -> i32 {
    let input: Vec<Vec<i32>> = input();
    let mut res: i32 = 0;

    for record in input {
        res += if record.len() < 2 {
            1
        } else {
            helper(&record, 0, 1, None, record.len())
        };
    }

    res
}

pub fn part_two() -> i32 {
    let input: Vec<Vec<i32>> = input();
    let mut res: i32 = 0;

    for record in input {
        if record.len() < 2 {
            res += 1;
        } else {
            for index in 0..record.len() {
                if helper(&record, 0, 1, None, index) == 1 {
                    res += 1;
                    break;
                }
            }
        };
    }

    res
}

fn helper(vec: &Vec<i32>, l: usize, r: usize, mut order: Option<Order>, index: usize) -> i32 {
    if r == vec.len() {
        return 1;
    }

    if l == index {
        return helper(vec, l + 1, r + 1, order, index);
    }

    if r == index {
        return helper(vec, l, r + 1, order, index);
    }

    if (vec[l] - vec[r]).abs() > 3 {
        return 0;
    }

    match &order {
        Some(ord) => match ord {
            Order::DEC => {
                if vec[l] <= vec[r] {
                    return 0;
                }
            }
            Order::INC => {
                if vec[l] >= vec[r] {
                    return 0;
                }
            }
        },
        None => match vec[l].cmp(&vec[r]) {
            Ordering::Less => order = Some(Order::INC),
            Ordering::Equal => return 0,
            Ordering::Greater => order = Some(Order::DEC),
        },
    }

    helper(vec, r, r + 1, order, index)
}

fn input() -> Vec<Vec<i32>> {
    let input: String = read_input::run(2).expect("Failed to read input!");

    let mut res: Vec<Vec<i32>> = Vec::new();

    for part in input.lines() {
        let report: Vec<i32> = part
            .split(" ")
            .filter_map(|char| char.parse().ok())
            .collect();

        res.push(report);
    }

    res
}
