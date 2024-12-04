type Matrix = Vec<Vec<char>>;

fn main() {
    let input = parse_matrix(include_str!("input.txt"));
    assert!(is_valid_matrix(&input), "Invalid matrix");
    println!("Part1: {:?}", count_part1(&input));
    println!("Part2: {:?}", count_part2(&input));
}

fn parse_matrix(input: &str) -> Matrix {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    matrix
}

/// Transform a string "str" into "str-rts"
fn get_double_backward(mut input: String) -> String {
    input.reserve(input.len() + 1);
    let reversed = input.chars().rev().collect::<String>();
    input.push('-');
    input.push_str(&reversed);
    input
}

fn count_part1(matrix: &Matrix) -> u64 {
    let size = matrix.len();
    let mut sum = 0;
    for i in 0..size {
        sum += count_xmas_in_string(&get_double_backward(get_row(&matrix, i)));
        sum += count_xmas_in_string(&get_double_backward(get_column(&matrix, i)));
        sum += count_xmas_in_string(&get_double_backward(get_diagonal(&matrix, 0, i)));
        sum += count_xmas_in_string(&get_double_backward(get_diagonal_reverse(&matrix, i, 0)));
        if i > 0 {
            sum += count_xmas_in_string(&get_double_backward(get_diagonal(&matrix, i, 0)));
            sum += count_xmas_in_string(&get_double_backward(get_diagonal_reverse(
                &matrix,
                size - 1,
                i,
            )));
        }
    }
    sum
}

fn count_part2(matrix: &Matrix) -> u64 {
    fn check_ms(c1: char, c2: char) -> bool {
        c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M'
    }

    let size = matrix.len();
    let mut sum = 0;
    for row in 1..size - 1 {
        for column in 1..size - 1 {
            if matrix[row][column] == 'A' {
                if check_ms(matrix[row - 1][column - 1], matrix[row + 1][column + 1]) {
                    if check_ms(matrix[row - 1][column + 1], matrix[row + 1][column - 1]) {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn is_valid_matrix(matrix: &Matrix) -> bool {
    for row in matrix.iter() {
        if row.len() != matrix.len() {
            return false;
        }
    }
    true
}

fn get_row(matrix: &Matrix, row: usize) -> String {
    assert!(row < matrix.len());
    matrix[row].iter().collect()
}

fn get_column(matrix: &Matrix, column: usize) -> String {
    assert!(column < matrix.len());
    matrix.iter().map(|row| row[column]).collect()
}

fn get_diagonal(matrix: &Matrix, mut row: usize, mut column: usize) -> String {
    assert!(row < matrix.len());
    assert!(column < matrix.len());
    let mut diagonal = String::new();
    while row < matrix.len() && column < matrix.len() {
        diagonal.push(matrix[row][column]);
        row += 1;
        column += 1;
    }
    diagonal
}

fn get_diagonal_reverse(matrix: &Matrix, mut row: usize, mut column: usize) -> String {
    assert!(row < matrix.len());
    assert!(column < matrix.len());

    let mut diagonal = String::new();
    while row > 0 && column < matrix.len() {
        diagonal.push(matrix[row][column]);
        row -= 1;
        column += 1;
    }
    if column < matrix.len() {
        diagonal.push(matrix[row][column]);
    }
    diagonal
}

fn count_xmas_in_string(value: &str) -> u64 {
    (value.split("XMAS").count() - 1) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(count_xmas_in_string(""), 0);
        assert_eq!(count_xmas_in_string("aa"), 0);
        assert_eq!(count_xmas_in_string("XMAS"), 1);
        assert_eq!(count_xmas_in_string("aXMASa"), 1);
        assert_eq!(count_xmas_in_string("XMASXMAS"), 2);
    }

    #[test]
    fn test_double_backward() {
        assert_eq!(get_double_backward("str".to_owned()), "str-rts");
    }

    #[test]
    fn test_spec() {
        let str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
            .trim();
        let matrix = parse_matrix(str);
        // assert_eq!(count_part1(&matrix), 18);
        assert_eq!(count_part2(&matrix), 9);
    }
}
