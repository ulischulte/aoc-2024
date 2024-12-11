use regex::{Regex, Captures};

advent_of_code::solution!(3);

fn process_line(captures: regex::CaptureMatches, enabled: &mut bool) -> u32 {
    captures
        .filter_map(|capture| {
            let part = &capture[0];
            match part {
                "do()" => {
                    *enabled = true;
                    None
                }
                "don't()" => {
                    *enabled = false;
                    None
                }
                _ if *enabled => calculate_multiplication(&capture[2], &capture[3]),
                _ => None,
            }
        })
        .sum()
}

fn calculate_multiplication(x_cap: &str, y_cap: &str) -> Option<u32> {
    let x = x_cap.parse::<u32>().unwrap();
    let y = y_cap.parse::<u32>().unwrap();
    Some(x * y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(re.captures_iter(input).map(|mul| {
        calculate_multiplication(&mul[1], &mul[2]).unwrap()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();
    let mut enabled = true;
    let mul_total = process_line(regex.captures_iter(input), &mut enabled);
    Some(mul_total)
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