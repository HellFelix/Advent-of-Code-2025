use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut values = Vec::new();
    for line in input.lines() {
        let mut total = 0;

        let mut index = 0;
        for i in 0..12 {
            let (leading, found_index) = find_leading(&line[index..(line.len() - 11 + i)]);
            total += leading * 10_i64.pow(11 - i as u32);
            index += found_index + 1;
        }

        values.push(total);
    }

    println!("Total: {}", values.iter().sum::<i64>());
}

fn find_leading(input: &str) -> (i64, usize) {
    let mut largest = 0;
    let mut index = 0;
    for i in 0..input.len() {
        let number = input[i..i + 1].parse::<i64>().unwrap();
        if number > largest {
            largest = number;
            index = i;
        }
    }
    (largest, index)
}
