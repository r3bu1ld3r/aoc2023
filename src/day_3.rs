use std::char;
use std::collections::{HashSet, VecDeque};

const SIZE: usize = 140;

fn main() {
    let input = include_str!("../input/3.txt");
    let mut matrix = vec![vec![0i32; 140]; 140];

    for (i, line) in input.trim().split('\n').enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => matrix[i][j] = 0,
                c if c.is_ascii_digit() => {
                    if matrix[i][j] == 0 {
                        let s = line.as_bytes();
                        let mut ptr = j;
                        let mut number: i32 = 0;
                        while ptr < SIZE && char::is_ascii_digit(&(s[ptr] as char)) {
                            let digit = s[ptr] - '0' as u8;
                            number = number * 10 + digit as i32;
                            ptr += 1;
                        }
                        let mut filler = j;
                        while filler != ptr {
                            matrix[i][filler] = number as i32;
                            filler += 1
                        }
                    } else {
                        continue;
                    }
                }
                _ => matrix[i][j] = -1,
            }
        }
    }

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back((0, 0));
    let mut useful = 0;
    let mut visited = HashSet::<(usize, usize)>::new();

    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if visited.contains(&(i, j)) {
            continue;
        };
        visited.insert((i, j));
        if i + 1 < SIZE && !visited.contains(&(i + 1, j)) {
            queue.push_back((i + 1, j));
        };
        if j + 1 < SIZE && !visited.contains(&(i, j + 1)) {
            queue.push_back((i, j + 1));
        };
        if i > 0 && !visited.contains(&(i - 1, j)) {
            queue.push_back((i - 1, j));
        };
        if j > 0 && !visited.contains(&(i, j - 1)) {
            queue.push_back((i, j - 1));
        };
        if i + 1 < SIZE && j + 1 < SIZE && !visited.contains(&(i + 1, j + 1)) {
            queue.push_back((i + 1, j + 1));
        };
        if i + 1 < SIZE && j > 0 && !visited.contains(&(i + 1, j - 1)) {
            queue.push_back((i + 1, j - 1));
        };
        if i > 0 && j + 1 < SIZE && !visited.contains(&(i - 1, j + 1)) {
            queue.push_back((i - 1, j + 1));
        };
        if i > 0 && j > 0 && !visited.contains(&(i - 1, j - 1)) {
            queue.push_back((i - 1, j - 1));
        };
        match matrix[i][j] {
            -1 => {
                let mut set = HashSet::<i32>::new();
                if i + 1 < SIZE && matrix[i + 1][j] > 0 {
                    set.insert(matrix[i + 1][j]);
                };
                if i > 0 && matrix[i - 1][j] > 0 {
                    set.insert(matrix[i - 1][j]);
                };
                if i + 1 < SIZE && j + 1 < SIZE && matrix[i + 1][j + 1] > 0 {
                    set.insert(matrix[i + 1][j + 1]);
                };
                if i + 1 < SIZE && j > 0 && matrix[i + 1][j - 1] > 0 {
                    set.insert(matrix[i + 1][j - 1]);
                };
                if j + 1 < SIZE && matrix[i][j + 1] > 0 {
                    set.insert(matrix[i][j + 1]);
                };
                if j > 0 && matrix[i][j - 1] > 0 {
                    set.insert(matrix[i][j - 1]);
                };
                if i > 0 && j + 1 < SIZE && matrix[i - 1][j + 1] > 0 {
                    set.insert(matrix[i - 1][j + 1]);
                };
                if i > 0 && j > 0 && matrix[i - 1][j - 1] > 0 {
                    set.insert(matrix[i - 1][j - 1]);
                };
                useful += set.iter().sum::<i32>();
            }
            _ => {}
        }
    }
    println!("{}", useful);
}
