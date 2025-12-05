use std::{fs::File, io::Read};

fn check_merger(lower_bound: usize, upper_bound: usize, range: &mut (usize, usize)) -> bool {
    let mut overlaps = false;
    if lower_bound <= range.0 && upper_bound >= range.0 {
        range.0 = lower_bound;
        overlaps = true;
    }

    if upper_bound >= range.1 && lower_bound <= range.1 {
        range.1 = upper_bound;
        overlaps = true;
    }

    if lower_bound >= range.0 && upper_bound <= range.1 {
        overlaps = true;
    }

    overlaps
}

fn main() {
    let mut input = String::new();
    let mut input_file = File::open("input").unwrap();
    input_file.read_to_string(&mut input).unwrap();

    let mut fresh_ranges: Vec<(usize, usize)> = Vec::new();
    // let mut is_range = true;

    // let mut fresh_ids = 0;
    for (n, line) in input.lines().enumerate() {
        println!("{line}");
        if line.is_empty() {
            // is_range = false;
            break;
        } else {
            let mut bounds = line.split("-");
            let lower_bound = bounds.next().unwrap().parse::<usize>().unwrap();
            let upper_bound = bounds.next().unwrap().parse::<usize>().unwrap();

            let mut overlaps = false;

            for range in &mut fresh_ranges {
                if lower_bound <= range.0 && upper_bound >= range.0 {
                    range.0 = lower_bound;
                    overlaps = true;
                }

                if upper_bound >= range.1 && lower_bound <= range.1 {
                    range.1 = upper_bound;
                    overlaps = true;
                }

                if lower_bound >= range.0 && upper_bound <= range.1 {
                    overlaps = true;
                }
            }

            // Check for merger
            if overlaps {
                let mut merged = true;
                while merged {
                    merged = false;
                    'outer: for i in 0..fresh_ranges.len() {
                        for j in i + 1..fresh_ranges.len() {
                            if check_merger(
                                fresh_ranges[i].0,
                                fresh_ranges[i].1,
                                &mut fresh_ranges[j],
                            ) {
                                fresh_ranges.remove(i);
                                merged = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if !overlaps {
                fresh_ranges.push((lower_bound, upper_bound));
            }
        }

        // if is_range {
        //     let mut bounds = line.split("-");
        //     let lower_bound = bounds.next().unwrap().parse::<usize>().unwrap();
        //     let upper_bound = bounds.next().unwrap().parse::<usize>().unwrap();
        //     fresh_ranges.push((lower_bound, upper_bound));
        // } else {
        //     let mut is_fresh = false;
        //     let id = line.trim().parse::<usize>().unwrap();
        //     'outer: for range in &fresh_ranges {
        //         if id >= range.0 && id <= range.1 {
        //             is_fresh = true;
        //             break 'outer;
        //         }
        //     }
        //
        //     if is_fresh {
        //         fresh_ids += 1;
        //     }
        // }
    }

    let mut total_ids = 0;
    for range in fresh_ranges {
        println!("{}-{}", range.0, range.1);
        total_ids += range.1 - range.0 + 1;
    }

    println!("Total: {total_ids}");
}
