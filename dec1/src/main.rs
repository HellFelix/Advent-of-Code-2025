use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut file = File::open("input").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut current_point: i32 = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let direction = match &line[0..1] {
            "R" => 1,
            "L" => -1,
            _ => panic!("Invalid input"),
        };

        let steps = line[1..].parse::<i32>().unwrap();

        let was_positive = current_point > 0; // strictly greater!!
        current_point += direction * steps;
        zeroes += (current_point / 100).abs();
        if current_point == 0 {
            zeroes += 1;
        }
        if was_positive && current_point < 0 {
            // we have wrapped from positive to negative
            zeroes += 1;
        }
        current_point = current_point.rem_euclid(100);
    }

    println!("Found {zeroes} zeroes");
}
