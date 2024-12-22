use crate::utils::read_input;

pub fn part_one() -> i32 {
    let input: Vec<Vec<char>> = input();

    let mut res: i32 = 0;

    const MOVES: [(i32, i32); 8] = [(0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1), (1, -1), (-1, 1)];
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut char_at: usize = 0;
            if input[i][j] == WORD[char_at] {
                for dir in MOVES {
                    let mut a: i32 = i as i32;
                    let mut b: i32 = j as i32;
                    
                    while 0 <= a
                    && (a as usize) < input.len()
                    && 0 <= b
                    && (b as usize) < input[0].len()
                    && input[a as usize][b as usize] == WORD[char_at]
                    {
                        a += dir.0;
                        b += dir.1;
                        char_at += 1;
                        if char_at == WORD.len() {
                            res += 1;
                            break;
                        }
                    }

                    char_at = 0;
                }
            }
        }
    }

    res
}

fn input() -> Vec<Vec<char>> {
    let input: String = read_input::run(4).expect("Failed to read file!");

    let mut res: Vec<Vec<char>> = Vec::new();

    for part in input.lines() {
        let chars: Vec<char> = part.chars().collect();
        res.push(chars);
    }

    res
}
