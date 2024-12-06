use std::collections::HashSet;

struct InputFile {
    ordering: HashSet<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

fn main() {
    let input = parse_file(include_str!("input.txt"));
    println!("Part 1: {}", do_part1(&input));
    println!("Part 2: {}", do_part2(&input));
}

fn do_part1(input: &InputFile) -> i32 {
    let mut sum = 0;
    for (i, update) in input.updates.iter().enumerate() {
        if is_correct_order(&input, i) {
            sum += get_middle_point(update);
        }
    }
    sum
}

fn do_part2(input: &InputFile) -> i32 {
    let mut sum = 0;
    for (i, _) in input.updates.iter().enumerate() {
        if !is_correct_order(&input, i) {
            let correct_update = fix_order(&input, i);
            sum += get_middle_point(&correct_update);
        }
    }
    sum
}

fn get_middle_point(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

fn parse_file(input: &str) -> InputFile {
    let mut iterator = input.lines().into_iter();

    let mut ordering = HashSet::new();
    loop {
        let line = iterator.next().expect("Unexpected EOF").trim();
        if line.is_empty() {
            break;
        }
        let mut pair = line.split("|").map(|x| x.parse::<i32>().unwrap());
        let left = pair.next().expect("Expected value");
        let right = pair.next().expect("Expected value");
        ordering.insert((left, right));
    }

    let mut updates = vec![];
    loop {
        let line = iterator.next();
        if line.is_none() {
            break;
        }
        updates.push(
            line.unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }

    InputFile { ordering, updates }
}

fn is_correct_order(file: &InputFile, update_id: usize) -> bool {
    assert!(update_id < file.updates.len());

    let update = &file.updates[update_id];
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            // If we have a constraint in the opposite direction, return false
            if file.ordering.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }

    true
}

fn fix_order(file: &InputFile, update_id: usize) -> Vec<i32> {
    assert!(update_id < file.updates.len());

    let update = &file.updates[update_id];
    let mut correct_update = vec![];

    for i in 0..update.len() {
        let page = update[i];
        if correct_update.len() == 0 {
            correct_update.push(page);
        } else {
            let index = correct_update
                .iter()
                .enumerate()
                .find(|(_, value)| {
                    // can I put `page` before the current value?
                    let pair = (page, **value);
                    return file.ordering.contains(&pair);
                })
                .map(|(j, _)| j)
                .unwrap_or(correct_update.len());
            correct_update.insert(index, page);
        }
    }

    correct_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let test_file = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            .trim();
        let input = parse_file(test_file);
        assert_eq!(input.ordering.len(), 21);
        assert!(input.ordering.contains(&(47, 53)));
        assert!(input.ordering.contains(&(53, 13)));

        assert_eq!(input.updates.len(), 6);
        assert_eq!(input.updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(input.updates[5], vec![97, 13, 75, 29, 47]);

        assert_eq!(is_correct_order(&input, 0), true);
        assert_eq!(is_correct_order(&input, 1), true);
        assert_eq!(is_correct_order(&input, 2), true);
        assert_eq!(is_correct_order(&input, 3), false);
        assert_eq!(is_correct_order(&input, 4), false);
        assert_eq!(is_correct_order(&input, 5), false);

        assert_eq!(do_part1(&input), 143);

        assert_eq!(fix_order(&input, 3), vec![97, 75, 47, 61, 53]);
        assert_eq!(fix_order(&input, 4), vec![61, 29, 13]);
        assert_eq!(fix_order(&input, 5), vec![97, 75, 47, 29, 13]);

        assert_eq!(do_part2(&input), 123);
    }
}
