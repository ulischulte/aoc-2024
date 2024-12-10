advent_of_code::solution!(2);

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn is_sorted_sequence(numbers: &[i32]) -> bool {
    let is_increasing = numbers.windows(2).all(|w| is_window_valid(w, true));
    let is_decreasing = numbers.windows(2).all(|w| is_window_valid(w, false));
    is_increasing || is_decreasing
}

fn is_near_sorted_sequence(numbers: &[i32]) -> bool {
    is_sorted_sequence(numbers)
        || subsequences_omitting_one(numbers)
        .iter()
        .any(|seq| is_sorted_sequence(seq))
}

fn subsequences_omitting_one(report: &[i32]) -> Vec<Vec<i32>> {
    (0..report.len())
        .map(|i| [&report[..i], &report[i + 1..]].concat())
        .collect()
}

fn is_window_valid(window: &[i32], increasing: bool) -> bool {
    if let [first, second] = window {
        if increasing {
            matches!(second - first, 1..=3)
        } else {
            matches!(first - second, 1..=3)
        }
    } else {
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| is_sorted_sequence(&parse_numbers(line)))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| is_near_sorted_sequence(&parse_numbers(line)))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}