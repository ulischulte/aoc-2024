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
    let word_chars: Vec<char> = "XMAS".chars().collect();
    let mut occurrences = 0;

    for (row, col) in iproduct!(0..matrix.nrows(), 0..matrix.ncols()) {
        if matrix[[row, col]] == word_chars[0] {
            occurrences += check_all_directions(matrix, &word_chars, row, col);
        }
    }
    occurrences
}

fn check_all_directions(
    matrix: &Array2<char>,
    word_chars: &[char],
    start_row: usize,
    start_col: usize,
) -> u32 {
    DIRECTIONS
        .iter()
        .filter(|&&(dx, dy)| is_word_in_direction(matrix, word_chars, start_row, start_col, dx, dy))
        .count() as u32
}

fn is_word_in_direction(
    matrix: &Array2<char>,
    word_chars: &[char],
    start_row: usize,
    start_col: usize,
    dx: i32,
    dy: i32,
) -> bool {
    word_chars.iter().enumerate().all(|(i, &ch)| {
        let x = start_row as i32 + dx * i as i32;
        let y = start_col as i32 + dy * i as i32;
        is_position_in_bounds(x, y, matrix.nrows(), matrix.ncols())
            && matrix[[x as usize, y as usize]] == ch
    })
}

fn is_position_in_bounds(x: i32, y: i32, rows: usize, cols: usize) -> bool {
    (0..rows as i32).contains(&x) && (0..cols as i32).contains(&y)
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
    is_valid_x_mas_diagonal(matrix[[row - 1, col - 1]], matrix[[row + 1, col + 1]])
        && is_valid_x_mas_diagonal(matrix[[row - 1, col + 1]], matrix[[row + 1, col - 1]])
}

fn is_valid_x_mas_diagonal(first_char: char, last_char: char) -> bool {
    first_char == 'M' && last_char == 'S' || first_char == 'S' && last_char == 'M'
}

fn to_char_matrix(input: &str) -> Array2<char> {
    let rows: Vec<&str> = input.lines().collect();
    let row_count = rows.len();
    let col_count = rows[0].len();
    let chars: Vec<char> = rows.iter().flat_map(|line| line.chars()).collect();
    Array2::from_shape_vec((row_count, col_count), chars).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_xmas_in_matrix(&to_char_matrix(input)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_x_mas_in_matrix(&to_char_matrix(input)))
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