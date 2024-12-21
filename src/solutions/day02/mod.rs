use crate::utils::read_input;
use crate::utils::log::log;
use std::cmp::Ordering;

enum Order {
    INC,
    DEC
}

pub fn part_one() -> i32 {
    let input: Vec<Vec<i32>> = input();
    let mut res: i32 = 0;

    for record in input {
        res += if record.len() < 2 {1} else {helper(&record, 1, None)};
    }

    res
}

fn helper(vec: &Vec<i32>, index: usize, mut order: Option<Order>) -> i32 {
    if index == vec.len() {
        return 1
    }

    if (vec[index] - vec[index - 1]).abs() > 3 {return 0}
    
    match &order {
        Some(ord) => {
            match ord {
                Order::DEC => if vec[index] >= vec[index - 1] {return 0},
                Order::INC => if vec[index] <= vec[index - 1] {return 0}
            }
        } 
        None => {
            match vec[0].cmp(&vec[1]) {
                Ordering::Less => order = Some(Order::INC),
                Ordering::Equal => return 0,
                Ordering::Greater => order = Some(Order::DEC)
            }
        }
    }

    helper(vec, index + 1, order)
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