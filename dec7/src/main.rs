use std::{
    fs::File,
    io::{Read, Write, stdout},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Token {
    None,
    Beam(i64),
    Splitter,
}
impl Token {
    pub fn parse(c: char) -> Self {
        match c {
            'S' => Self::Beam(1),
            '.' => Self::None,
            '^' => Self::Splitter,
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut matrix = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Token::parse(c));
        }
        matrix.push(row);
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let token = matrix[i][j];
            if let Token::Beam(n) = token {
                // propogate
                if i + 1 < matrix.len() {
                    if matrix[i + 1][j] == Token::Splitter {
                        // Split
                        if j > 0 {
                            if let Token::Beam(ref mut pre) = matrix[i + 1][j - 1] {
                                *pre += n;
                            } else {
                                matrix[i + 1][j - 1] = Token::Beam(n);
                            }
                        }
                        if j + 1 < matrix[i + 1].len() {
                            if let Token::Beam(ref mut pre) = matrix[i + 1][j + 1] {
                                *pre += n;
                            } else {
                                matrix[i + 1][j + 1] = Token::Beam(n);
                            }
                        }
                    } else if let Token::Beam(ref mut pre) = matrix[i + 1][j] {
                        // Straight down
                        *pre += n;
                    } else {
                        // Straight down
                        matrix[i + 1][j] = Token::Beam(n);
                    }
                }
            }
        }
    }

    for row in matrix.clone() {
        for token in row {
            match token {
                Token::None => print!("."),
                Token::Beam(n) => print!("{n}"),
                Token::Splitter => print!("^"),
            }
        }
        print!("\n");
    }
    stdout().flush().unwrap();

    let mut splits = 0;
    for token in matrix.last().unwrap() {
        if let Token::Beam(n) = token {
            splits += n;
        }
    }

    println!("Total splits: {splits}");
}
