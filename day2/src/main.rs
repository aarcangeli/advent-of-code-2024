fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<&str>>();
    println!("Total lines: {}", lines.len());

    let sum = lines.iter().filter(|x| is_safe1(x)).count();
    println!("Part 1: {}", sum);

    let sum = lines.iter().filter(|x| is_safe2(x)).count();
    println!("Part 2: {}", sum);
}

fn is_safe(values: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect();
    if !deltas.iter().all(|x| *x > 0) && !deltas.iter().all(|x| *x < 0) {
        // The levels are either all increasing or all decreasing.
        return false;
    }
    if deltas.iter().any(|x| *x == 0 || x.abs() > 3) {
        // Any two adjacent levels differ by at least one and at most three.
        return false;
    }
    true
}

fn is_safe1(value: &str) -> bool {
    let values = split_numbers(value);
    is_safe(&values)
}

fn split_numbers(value: &str) -> Vec<i32> {
    let values = value
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    values
}

fn is_safe2(value: &str) -> bool {
    let values = split_numbers(value);
    if is_safe(&values) {
        return true;
    }
    // try to remove one value and check if the rest is safe.
    for (i, _) in values.iter().enumerate() {
        let mut values = values.clone();
        values.remove(i);
        if is_safe(&values) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe1() {
        // From the spec.
        assert_eq!(is_safe1("7 6 4 2 1"), true);
        assert_eq!(is_safe1("1 2 7 8 9"), false);
        assert_eq!(is_safe1("9 7 6 2 1"), false);
        assert_eq!(is_safe1("8 6 4 4 1"), false);
        assert_eq!(is_safe1("1 3 6 7 9"), true);
        // Additional tests.
        assert_eq!(is_safe1("1 2 3"), true);
        assert_eq!(is_safe1("1 2 2"), false);
        assert_eq!(is_safe1("1 2 6"), false);
    }

    #[test]
    fn test_is_safe2() {
        // From the spec.
        assert_eq!(is_safe2("7 6 4 2 1"), true);
        assert_eq!(is_safe2("1 2 7 8 9"), false);
        assert_eq!(is_safe2("9 7 6 2 1"), false);
        assert_eq!(is_safe2("1 3 2 4 5"), true);
        assert_eq!(is_safe2("8 6 4 4 1"), true);
        assert_eq!(is_safe2("1 3 6 7 9"), true);
    }
}
