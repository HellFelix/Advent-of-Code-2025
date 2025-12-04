use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut matrix = Vec::new();
    let mut m = 0;
    let mut n = 0;

    for line in input.lines() {
        let mut row = Vec::new();
        n = line.len();
        m += 1;
        for marker in line.chars() {
            row.push(match marker {
                '.' => false,
                '@' => true,
                _ => panic!("Invalid input"),
            });
        }
        matrix.push(row);
    }

    let mut available = 0;
    let mut removed = -1;
    while removed != 0 {
        removed = 0;
        // Super naive solution.
        // There's likely a smarter way to do this.
        for i in 0..m {
            for j in 0..n {
                let mut adjacent = 0;
                if i + 1 < m {
                    if j > 0 && matrix[i + 1][j - 1] {
                        adjacent += 1;
                    }
                    if matrix[i + 1][j] {
                        adjacent += 1;
                    }
                    if j + 1 < n && matrix[i + 1][j + 1] {
                        adjacent += 1;
                    }
                }

                if i > 0 {
                    if j > 0 && matrix[i - 1][j - 1] {
                        adjacent += 1;
                    }
                    if matrix[i - 1][j] {
                        adjacent += 1;
                    }
                    if j + 1 < n && matrix[i - 1][j + 1] {
                        adjacent += 1;
                    }
                }

                if j > 0 && matrix[i][j - 1] {
                    adjacent += 1;
                }
                if j + 1 < n && matrix[i][j + 1] {
                    adjacent += 1;
                }

                if matrix[i][j] && adjacent < 4 {
                    matrix[i][j] = false;
                    removed += 1;
                    available += 1;
                }
            }
        }
    }
    println!("{available}");
}
