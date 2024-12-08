struct Equation {
    result: u64,
    values: Vec<u64>,
}

fn main() {
    let state = parse_file(include_str!("input.txt"));
    println!("Part 1: {}", count_valid(&state, part1_operators));
    println!("Part 2: {}", count_valid(&state, part2_operators));
}

fn count_valid(equations: &Vec<Equation>, operators: fn(u64, u64) -> Vec<u64>) -> u64 {
    let mut sum = 0;
    for equation in equations {
        if may_be_valid(equation, operators) {
            sum += equation.result;
        }
    }
    sum
}

fn part1_operators(a: u64, b: u64) -> Vec<u64> {
    vec![a + b, a * b]
}

fn part2_operators(a: u64, b: u64) -> Vec<u64> {
    vec![a + b, a * b, int_concat(a, b)]
}

fn int_concat(a: u64, b: u64) -> u64 {
    let exponent = b.ilog(10);
    let multiplier = 10_u64.pow(exponent + 1);
    a * multiplier + b
}

fn may_be_valid(equation: &Equation, operators: fn(u64, u64) -> Vec<u64>) -> bool {
    // This method is only valid if all values are positive
    assert!(!equation.values.contains(&0));
    assert!(!equation.values.is_empty());

    fn validate(
        equation: &Equation,
        operators: fn(u64, u64) -> Vec<u64>,
        index: usize,
        accumulator: u64,
    ) -> bool {
        if accumulator > equation.result {
            // Optimization: this path cannot be the result
            return false;
        }
        let el = equation.values[index];
        let values = operators(accumulator, el);
        if index + 1 == equation.values.len() {
            // This is the last element
            values.contains(&equation.result)
        } else {
            values
                .iter()
                .any(|x| validate(equation, operators, index + 1, *x))
        }
    }

    validate(equation, operators, 0, 0)
}

fn parse_file(file: &str) -> Vec<Equation> {
    let mut result = vec![];
    for line in file.trim().lines() {
        result.push(
            line.split_once(':')
                .map(|(result, values)| Equation {
                    result: result.parse().expect("Invalid input"),
                    values: values
                        .trim()
                        .split(' ')
                        .map(|t| t.parse().expect("Invalid input"))
                        .collect(),
                })
                .expect("Expecting ':'"),
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn tests() {
        let equations = parse_file(TEST_INPUT);
        assert_eq!(equations.len(), 9);
        assert_eq!(equations[0].result, 190);
        assert_eq!(equations[0].values, vec![10, 19]);

        // may_be_valid part 1
        let valid_equations = vec![0, 1, 8];
        for (i, equation) in equations.iter().enumerate() {
            let expected_result = valid_equations.contains(&i);
            assert_eq!(
                may_be_valid(equation, part1_operators),
                expected_result
            );
        }

        // may_be_valid part 2
        let valid_equations = vec![0, 1, 3, 4, 6, 8];
        for (i, equation) in equations.iter().enumerate() {
            let expected_result = valid_equations.contains(&i);
            assert_eq!(
                may_be_valid(equation, part2_operators),
                expected_result
            );
        }
    }

    #[test]
    fn concat_tests() {
        assert_eq!(int_concat(1, 1), 11);
        assert_eq!(int_concat(1, 9), 19);
        assert_eq!(int_concat(1, 10), 110);
        assert_eq!(int_concat(11, 10), 1110);
        assert_eq!(int_concat(123, 123), 123123);
    }
}
