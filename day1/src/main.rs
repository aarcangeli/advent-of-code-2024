use regex::Regex;

fn main() {
    let (mut left_vec, mut right_vec) = read_inputs();

    left_vec.sort();
    right_vec.sort();

    // Part 1
    let mut sum = 0;
    for i in 0..left_vec.len() {
        let left = left_vec[i];
        let right = right_vec[i];
        let difference = (left - right).abs();
        sum += difference;
    }
    println!("Part 1: {}", sum);

    // In the second part the input should not be sorted, but the result is not influenced by the order.
    // ¯\_(ツ)_/¯

    // Part 2
    let mut sum: i64 = 0;
    for i in left_vec {
        let count = right_vec.iter().filter(|&&j| i == j).count();
        sum += i as i64 * count as i64;
    }
    println!("Part 2: {}", sum);
}

/// Read the input file and return the left and right values in two vectors
fn read_inputs() -> (Vec<i32>, Vec<i32>) {
    let source = include_str!("input.txt");
    let seperator = Regex::new(r" +").expect("Invalid regex");

    let mut left_vec = vec![];
    let mut right_vec = vec![];
    for row in source.lines() {
        let split: Vec<&str> = seperator.split(row).collect();
        if split.len() == 2 {
            left_vec.push(split[0].parse::<i32>().unwrap());
            right_vec.push(split[1].parse::<i32>().unwrap());
        } else {
            println!("Invalid row: '{}'", row);
        }
    }

    assert_eq!(left_vec.len(), 1000, "Invalid count");
    (left_vec, right_vec)
}
