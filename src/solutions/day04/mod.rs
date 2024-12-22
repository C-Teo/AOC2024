use crate::utils::read_input;

pub fn part_one() -> i32 {
    let input: Vec<Vec<char>> = input();

    let mut res: i32 = 0;

    const MOVES: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (1, 1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
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

pub fn part_two() -> i32 {
    let input: Vec<Vec<char>> = input();

    const MOVES: [(i32, i32); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

    let mut res: i32 = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i as usize][j as usize] == 'A' {
                let mut ct: i8 = 0;

                for dir in MOVES {
                    let a: i32 = i as i32;
                    let b: i32 = j as i32;

                    if (a + dir.0) >= 0
                        && ((a + dir.0) as usize) < input.len()
                        && (b + dir.1) >= 0
                        && ((b + dir.1) as usize) < input[0].len()
                        && (a - dir.0) >= 0
                        && ((a - dir.0) as usize) < input.len()
                        && (b - dir.1) >= 0
                        && ((b - dir.1) as usize) < input[0].len()
                    {
                        if input[(a + dir.0) as usize][(b + dir.1) as usize] == 'M'
                            && input[(a - dir.0) as usize][(b - dir.1) as usize] == 'S'
                        {
                            ct += 1;
                        }
                    }
                }

                if ct > 1 {
                    res += 1;
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
