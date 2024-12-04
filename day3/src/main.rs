use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", parse_and_sum(input));
    println!("Part 2: {}", parse_preprocessor_and_sum(input));
}

fn parse_and_sum(s: &str) -> u32 {
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in pattern.captures_iter(s) {
        let left = cap[1].parse::<u32>().unwrap();
        let right = cap[2].parse::<u32>().unwrap();
        sum += left * right;
    }
    sum
}

fn parse_preprocessor_and_sum(mut s: &str) -> u32 {
    let mut sum = 0;

    loop {
        // Find a closing instruction
        if let Some((left, right)) = s.split_once("don't()") {
            sum += parse_and_sum(left);
            s = right;
        } else {
            // No closing instruction found
            sum += parse_and_sum(s);
            break;
        }

        // Find an opening instruction
        if let Some((_, right)) = s.split_once("do()") {
            s = right;
        } else {
            // No opening instruction found
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = parse_and_sum(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let result = parse_preprocessor_and_sum(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, 48);
    }
}
