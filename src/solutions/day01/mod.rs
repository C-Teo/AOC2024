use crate::utils::read_input;
use std::collections::HashMap;

pub fn part_one() -> i32 {
    let (mut vec_one, mut vec_two) = input();

    vec_one.sort();
    vec_two.sort();

    let mut res: i32 = 0;

    for index in 0..vec_one.len() {
        res += (vec_one[index] - vec_two[index]).abs();
    }

    res
}

pub fn part_two() -> i32 {
    let (vec_one, vec_two) = input();

    let mut map:HashMap<i32, i32> = HashMap::new();

    vec_two.iter().for_each(|val| *(map.entry(*val).or_insert(0)) += 1);    

    let mut res:i32 = 0;

    for val in vec_one {
        res += val * map.get(&val).unwrap_or(&0);
    }

    res
}

fn input() -> (Vec<i32>, Vec<i32>) {
    let input: String = read_input::run(1).expect("Failed to read input!");
    
    let mut vec_one: Vec<i32> = Vec::new();
    let mut vec_two: Vec<i32> = Vec::new();
    
    for part in input.lines() {
        let mut iter = part.split("   ");
        
        if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
            if let (Ok(parsed_one), Ok(parsed_two)) = (first.trim().parse::<i32>(), second.trim().parse::<i32>()) {
                vec_one.push(parsed_one);
                vec_two.push(parsed_two);
            }
        }
    }
    
    (vec_one, vec_two)
}