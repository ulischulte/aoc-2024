use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Default, Clone, Copy)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn move_to_next_position(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }

    fn turn_right_90_degrees(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Obstruction,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (Guard, Vec<Vec<Cell>>) {
    let mut guard = Guard::default();

    let grid = input
        .trim_end()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '^' => {
                        guard.x = x;
                        guard.y = y;
                        Cell::Empty
                    }
                    '.' => Cell::Empty,
                    '#' => Cell::Obstruction,
                    _ => panic!("unknown character: {}", char),
                })
                .collect()
        })
        .collect();

    (guard, grid)
}

fn analyze_cell_ahead(guard: &Guard, grid: &[Vec<Cell>]) -> Option<Cell> {
    let (next_x, next_y) = match guard.direction {
        Direction::Left => (guard.x.checked_sub(1)?, guard.y),
        Direction::Up => (guard.x, guard.y.checked_sub(1)?),
        Direction::Right => (guard.x + 1, guard.y),
        Direction::Down => (guard.x, guard.y + 1),
    };

    // include out-of-bounds-check for next_y and next_x
    grid.get(next_y).and_then(|row| row.get(next_x)).copied()
}

fn extract_visited_positions(guard: &mut Guard, grid: &Vec<Vec<Cell>>) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert((guard.x, guard.y));

    while let Some(cell) = analyze_cell_ahead(guard, grid) {
        if cell == Cell::Obstruction {
            guard.turn_right_90_degrees();
        }
        guard.move_to_next_position();
        visited.insert((guard.x, guard.y));
    }
    visited
}

fn is_cycling(mut guard: Guard, grid: &[Vec<Cell>]) -> bool {
    let mut visited = HashSet::new();
    visited.insert((guard.x, guard.y, guard.direction));

    loop {
        match analyze_cell_ahead(&guard, grid) {
            Some(Cell::Obstruction) => guard.turn_right_90_degrees(),
            Some(Cell::Empty) => guard.move_to_next_position(),
            None => return false,
        }

        if !visited.insert((guard.x, guard.y, guard.direction)) {
            return true;
        }
    }
}

fn count_cycle_causing_positions(
    initial_guard: &Guard,
    original_grid: &[Vec<Cell>],
    visited_positions: &HashSet<(usize, usize)>
) -> u32 {
    visited_positions
        .par_iter()
        .filter(|&&pos| pos != (initial_guard.x, initial_guard.y))
        .map(|&pos| creates_cycle(pos, initial_guard, original_grid))
        .filter(|&creates_cycle| creates_cycle)
        .count() as u32
}

fn creates_cycle(
    (x, y): (usize, usize),
    initial_guard: &Guard,
    original_grid: &[Vec<Cell>]
) -> bool {
    let mut modified_grid = original_grid.to_vec();
    modified_grid[y][x] = Cell::Obstruction;
    is_cycling(*initial_guard, &modified_grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, grid) = parse(input);
    let visited = extract_visited_positions(&mut guard, &grid);
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (initial_guard, original_grid) = parse(input);
    let visited_positions = extract_visited_positions(&mut initial_guard.clone(), &original_grid);

    let cycle_causing_positions = count_cycle_causing_positions(
        &initial_guard,
        &original_grid,
        &visited_positions
    );

    Some(cycle_causing_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
