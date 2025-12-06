use std::{fs::File, io::Read};

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}
impl Operation {
    pub fn parse(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Mul,
            _ => panic!("Invalid operation"),
        }
    }
    pub fn initial(&self) -> i64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }

    pub fn apply(&self, value: &mut i64, rhs: i64) {
        match self {
            Self::Add => *value += rhs,
            Self::Mul => *value *= rhs,
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut lines = input.lines().rev();

    let operations_line = lines.next().unwrap();
    let line_len = operations_line.len();
    let operations = operations_line
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| Operation::parse(c));

    let mut numbers = Vec::new();
    let mut col = 0;
    numbers.push(vec![]);
    for i in (1..=line_len).rev() {
        let mut chars = Vec::new();
        for line in lines.clone() {
            let c = &line[i - 1..i];
            chars.push(c);
        }

        if chars.iter().all(|c| *c == " ") {
            col += 1;
            numbers.push(vec![]);
        } else {
            numbers[col].push(
                chars
                    .iter()
                    .rev()
                    .fold(String::new(), |v, c| v + if *c == " " { "" } else { c })
                    .parse::<i64>()
                    .unwrap(),
            );
        }
    }

    let mut result = 0;

    for (i, op) in operations.rev().enumerate() {
        let mut value = op.initial();
        for n in &numbers[i] {
            op.apply(&mut value, *n);
        }

        result += value;
    }

    println!("Total: {result}");
}
