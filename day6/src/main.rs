use bitflags::bitflags;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Cell {
    // An empty space
    Free,
    // An empty space reached by the guard
    Walked(DirectionField),
    // A wall
    Block,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DirectionField: u32 {
        const None = 0;
        const Up = 0x1;
        const Right = 0x2;
        const Down = 0x4;
        const Left = 0x8;
    }
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_field(&self) -> DirectionField {
        match self {
            Direction::Up => DirectionField::Up,
            Direction::Right => DirectionField::Right,
            Direction::Down => DirectionField::Down,
            Direction::Left => DirectionField::Left,
        }
    }
}

#[derive(Clone)]
struct State {
    map: Vec<Vec<Cell>>,
    guard_location: (i32, i32),
    guard_direction: Direction,
}

enum StepResult {
    Ok,
    Cycle,
    ReachedExit,
}

impl State {
    fn cell_at(&self, input: (i32, i32)) -> Cell {
        self.map[input.0 as usize][input.1 as usize]
    }

    fn set_cell_at(&mut self, input: (i32, i32), cell: Cell) {
        self.map[input.0 as usize][input.1 as usize] = cell
    }

    fn num_rows(&self) -> i32 {
        self.map.len() as i32
    }

    fn num_columns(&self) -> i32 {
        self.map[0].len() as i32
    }

    fn advance(&self, input: (i32, i32), direction: Direction) -> Option<(i32, i32)> {
        let (row, column) = input;
        match direction {
            Direction::Up => {
                if row > 0 {
                    Some((row - 1, column))
                } else {
                    None
                }
            }
            Direction::Right => {
                if column < self.num_columns() - 1 {
                    Some((row, column + 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row < self.num_rows() - 1 {
                    Some((row + 1, column))
                } else {
                    None
                }
            }
            Direction::Left => {
                if column > 0 {
                    Some((row, column - 1))
                } else {
                    None
                }
            }
        }
    }

    fn step(&mut self) -> StepResult {
        if let Some(next_position) = self.advance(self.guard_location, self.guard_direction) {
            // Update the guard location
            match self.cell_at(next_position) {
                Cell::Free | Cell::Walked(_) => self.guard_location = next_position,
                Cell::Block => self.guard_direction = self.guard_direction.rotate(),
            }
            // Mark as reached and detect a possible cycle
            if self.mark_reached() {
                StepResult::Cycle
            } else {
                StepResult::Ok
            }
        } else {
            StepResult::ReachedExit
        }
    }

    // Marks the cell as reached.
    // If a cycle is detected, true is returned
    fn mark_reached(&mut self) -> bool {
        let location = self.guard_location;
        let direction = self.guard_direction;
        match self.cell_at(location) {
            Cell::Free => {
                self.set_cell_at(location, Cell::Walked(self.guard_direction.to_field()));
                false
            }
            Cell::Walked(dir) => {
                if (dir & direction.to_field()) != DirectionField::None {
                    // We already walked this area
                    true
                } else {
                    self.set_cell_at(location, Cell::Walked(dir | direction.to_field()));
                    false
                }
            }
            Cell::Block => {
                panic!("The guard should not be on a block");
            }
        }
    }

    fn run(&mut self) {
        loop {
            match self.step() {
                StepResult::Ok => {}
                StepResult::ReachedExit => break,
                StepResult::Cycle => panic!("Cyclic maze"),
            }
        }
    }

    fn is_endless_loop(&self) -> bool {
        // work on a local copy
        let mut copy = self.clone();
        loop {
            match copy.step() {
                StepResult::Ok => {}
                StepResult::ReachedExit => return false,
                StepResult::Cycle => return true,
            }
        }
    }

    /// Count all walked map locations
    fn count_walked(&self) -> i32 {
        self.map
            .iter()
            .map(|x| x.iter().filter(|x| matches!(**x, Cell::Walked(_))).count() as i32)
            .sum()
    }
}

fn main() {
    let state = parse_file(include_str!("input.txt"));
    println!("Part 1: {}", count_walkable_cells(state.clone()));
    println!("Part 2: {}", count_obstacle_locations(state));
}

fn count_walkable_cells(mut state: State) -> i32 {
    state.run();
    state.count_walked()
}

fn count_obstacle_locations(mut state: State) -> i32 {
    let mut count = 0;
    for row in 0..state.num_rows() {
        for column in 0..state.num_columns() {
            if state.cell_at((row, column)) == Cell::Free {
                state.set_cell_at((row, column), Cell::Block);
                if state.is_endless_loop() {
                    count += 1;
                }
                state.set_cell_at((row, column), Cell::Free);
            }
        }
    }
    count
}

fn parse_file(file: &str) -> State {
    let mut map = vec![];
    let mut location: Option<_> = None;
    for (row, line) in file.trim().lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(column, el)| match el {
                    '#' => Cell::Block,
                    '.' => Cell::Free,
                    '^' => {
                        location = Some((row as i32, column as i32));
                        Cell::Walked(DirectionField::Up)
                    }
                    _ => panic!("Invalid cell"),
                })
                .collect(),
        );
    }
    State {
        map,
        guard_location: location.unwrap(),
        guard_direction: Direction::Up,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let state = parse_file(get_test_map());
        assert_eq!(state.map.len(), 10);
        for el in &state.map {
            assert_eq!(el.len(), 10);
        }
        assert_eq!(state.guard_location, (6, 4));
        assert_eq!(state.guard_direction, Direction::Up);
        assert_eq!(state.cell_at((0, 0)), Cell::Free);
        assert_eq!(state.cell_at((4, 0)), Cell::Free);
        assert_eq!(state.cell_at((0, 4)), Cell::Block);
        assert_eq!(state.cell_at((6, 4)), Cell::Walked(DirectionField::Up));

        // advance
        assert_eq!(state.advance((1, 1), Direction::Up), Some((0, 1)));
        assert_eq!(state.advance((0, 1), Direction::Up), None);
        assert_eq!(state.advance((1, 1), Direction::Left), Some((1, 0)));
        assert_eq!(state.advance((1, 0), Direction::Left), None);
        assert_eq!(state.advance((8, 8), Direction::Down), Some((9, 8)));
        assert_eq!(state.advance((9, 8), Direction::Down), None);
        assert_eq!(state.advance((8, 8), Direction::Right), Some((8, 9)));
        assert_eq!(state.advance((8, 9), Direction::Right), None);

        // Run
        assert_eq!(state.count_walked(), 1);
        assert_eq!(count_walkable_cells(state), 41);
    }

    #[test]
    fn test_part2() {
        let state = parse_file(get_test_map());
        assert_eq!(count_obstacle_locations(state), 6);
    }

    fn get_test_map() -> &'static str {
        r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .trim()
    }
}
