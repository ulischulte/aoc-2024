use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Default, Clone)]
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

#[derive(Copy, Clone)]
enum Cell {
    Obstruction,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (Guard, Vec<Vec<Cell>>) {
    let mut guard = Guard::default();

    let grid: Vec<Vec<Cell>> = input
        .trim_end()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '#' => Cell::Obstruction,
                    '.' => Cell::Empty,
                    '^' => {
                        guard.x = x;
                        guard.y = y;
                        Cell::Empty
                    }
                    _ => panic!("unknown character: {}", char),
                })
                .collect()
        })
        .collect();

    (guard, grid)
}

fn analyze_cell_ahead(guard: &Guard, grid: &[Vec<Cell>]) -> Option<Cell> {
    let out_of_bounds = match guard.direction {
        Direction::Left if guard.x == 0 => true,
        Direction::Up if guard.y == 0 => true,
        Direction::Right if guard.x == grid[0].len() - 1 => true,
        Direction::Down if guard.y == grid.len() - 1 => true,
        _ => false,
    };

    if out_of_bounds {
        return None;
    }

    let mut clone = guard.clone();
    clone.move_to_next_position();

    Some(grid[clone.y][clone.x])
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, grid) = parse(input);
    let mut visited = HashSet::new();

    visited.insert((guard.x, guard.y));

    while let Some(cell) = analyze_cell_ahead(&guard, &grid) {
        match cell {
            Cell::Obstruction => {
                guard.turn_right_90_degrees();
            }
            Cell::Empty => {}
        }
        guard.move_to_next_position();
        visited.insert((guard.x, guard.y));
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
