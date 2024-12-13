use itertools::iproduct;
use ndarray::Array2;

advent_of_code::solution!(4);

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_xmas_in_matrix(matrix: &Array2<char>) -> u32 {
    let word_chars: &[char] = &['X', 'M', 'A', 'S'];
    let mut occurrences = 0;

    for (row, col) in iproduct!(0..matrix.nrows(), 0..matrix.ncols()) {
        if matrix[[row, col]] == word_chars[0] {
            occurrences += DIRECTIONS
                .iter()
                .filter(|&&(dx, dy)| is_word_in_direction(matrix, word_chars, row, col, dx, dy))
                .count() as u32;
        }
    }
    occurrences
}

fn is_word_in_direction(
    matrix: &Array2<char>,
    word_chars: &[char],
    start_row: usize,
    start_col: usize,
    dx: i32,
    dy: i32,
) -> bool {
    fn is_position_in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
        x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32
    }
    word_chars.iter().enumerate().all(|(i, &ch)| {
        let x = start_row as i32 + dx * i as i32;
        let y = start_col as i32 + dy * i as i32;
        is_position_in_bounds(x, y, matrix.nrows(), matrix.ncols())
            && matrix[[x as usize, y as usize]] == ch
    })
}

fn count_x_mas_in_matrix(matrix: &Array2<char>) -> u32 {
    let mut occurrences = 0;

    for (row, col) in iproduct!(1..matrix.nrows() - 1, 1..matrix.ncols() - 1) {
        if matrix[[row, col]] == 'A' && is_mas_x(matrix, row, col) {
            occurrences += 1;
        }
    }
    occurrences
}

fn is_mas_x(matrix: &Array2<char>, row: usize, col: usize) -> bool {
    fn is_valid_x_mas_diagonal(first_char: char, last_char: char) -> bool {
        first_char == 'M' && last_char == 'S' || first_char == 'S' && last_char == 'M'
    }

    is_valid_x_mas_diagonal(matrix[[row - 1, col - 1]], matrix[[row + 1, col + 1]])
        && is_valid_x_mas_diagonal(matrix[[row - 1, col + 1]], matrix[[row + 1, col - 1]])
}

fn to_char_matrix(input: &str) -> Array2<char> {
    let rows = input.lines();
    let row_count = rows.clone().count();
    let col_count = rows.clone().next().map_or(0, |line| line.len());

    // Reserve the storage space directly to avoid reallocating
    let mut chars = Vec::with_capacity(row_count * col_count);
    for line in rows {
        chars.extend(line.chars());
    }

    Array2::from_shape_vec((row_count, col_count), chars).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let char_matrix = to_char_matrix(input);
    Some(count_xmas_in_matrix(&char_matrix))
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_matrix = to_char_matrix(input);
    Some(count_x_mas_in_matrix(&char_matrix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
