use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(3);

fn process_instructions(captures: regex::CaptureMatches) -> u32 {
    let mut enabled = true;
    captures
        .filter_map(|capture| {
            let part = &capture[0];
            match part {
                "do()" => {
                    enabled = true;
                    None
                }
                "don't()" => {
                    enabled = false;
                    None
                }
                _ if enabled => multiply_capture(&capture[2], &capture[3]),
                _ => None,
            }
        })
        .sum()
}

fn multiply_capture(x_cap: &str, y_cap: &str) -> Option<u32> {
    let x = x_cap.parse::<u32>().unwrap();
    let y = y_cap.parse::<u32>().unwrap();
    Some(x * y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        regex
            .captures(input)
            .par_iter()
            .filter_map(|capture| multiply_capture(&capture[1], &capture[2]))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();

    Some(process_instructions(regex.captures_iter(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
