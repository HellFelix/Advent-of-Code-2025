use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut valid_ids = Vec::new();
    for range in input.split(",") {
        let mut bounds = range.split("-");
        let lower_bound = bounds.next().unwrap().trim();
        let upper_bound = bounds.next().unwrap().trim();

        // I'm not really happy with this, but I can't think of a more efficient way of doing it
        // right now.
        for i in lower_bound.parse::<i64>().unwrap()..=upper_bound.parse::<i64>().unwrap() {
            let string_repr = i.to_string();
            let string_len = string_repr.len();
            let factors = list_factors(string_len);
            'iter_factors: for sub_len in factors {
                let mut sub_strings = Vec::new();
                let substring_len = string_len / sub_len;
                for k in 0..sub_len {
                    sub_strings.push(&string_repr[k * substring_len..(k + 1) * substring_len]);
                }
                if sub_strings.len() > 1 && sub_strings.iter().min() == sub_strings.iter().max() {
                    valid_ids.push(string_repr.parse::<i64>().unwrap());
                    break 'iter_factors;
                }
            }
        }
    }

    println!("Total: {}", valid_ids.iter().sum::<i64>());
}

fn list_factors(number: usize) -> Vec<usize> {
    let mut factors = Vec::new();

    let mut i = 1;

    while i * i <= number {
        if number % i == 0 {
            factors.push(i);

            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }

    factors.sort();

    return factors;
}
