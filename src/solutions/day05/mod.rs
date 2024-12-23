use crate::utils::read_input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one() -> i32 {
    let (mut mp, updates) = input();
    let mut res: i32 = 0;
    
    'outer: for entry in updates {
        
        let mut cache: HashSet<i32> = HashSet::new();

        for entry in &mut mp {
             entry.1.0 = false;
        }

        for update in &entry {
            if cache.contains(&update) {
                continue 'outer;
            }

            match mp.get_mut(&update) {
                Some((bool, set)) => {
                    if !*bool {
                        *bool = true;
                        cache.extend(set.iter());
                    }
                },
                _ => {
                    continue;
                }
            }

        }

        res += entry[entry.len()/2];
    }    

    res
}

fn input() -> (HashMap<i32, (bool, HashSet<i32>)>, Vec<Vec<i32>>) {
    let input: String = read_input::run(5).expect("Failed to read file!");

    let mut mp: HashMap<i32, (bool, HashSet<i32>)> = HashMap::new();
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut toggle: bool = true;

    for line in input.lines() {
        if line == "" {
            toggle = false;
            continue;
        }

        if toggle {
            let entry: Vec<i32> = line.split("|").filter_map(|val| val.parse().ok()).collect();
            
            match mp.get_mut(&entry[1]) {
                Some((_, set)) => {
                    set.insert(entry[0]);
                },
                None => {
                    mp.insert(entry[1], (false, HashSet::from([entry[0]])));
                }
            }
        
        } else {
            let entry: Vec<i32> = line.split(",").filter_map(|val| val.parse().ok()).collect();
            res.push(entry);
        }
    }

    (mp, res)
}
