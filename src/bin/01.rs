use std::collections::HashMap;
const DELIMITER: &str = "   ";
advent_of_code::solution!(1);

fn parse_location_line(line: &str) -> (u32, u32) {
    let parts: Vec<&str> = line.split(DELIMITER).collect();
    let location1 = parts[0].parse::<u32>().unwrap();
    let location2 = parts[1].parse::<u32>().unwrap();
    (location1, location2)
}

fn load_locations(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (location_list1, location_list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .into_iter()
        .map(|line| parse_location_line(&line))
        .unzip();
    (location_list1, location_list2)
}

fn calculate_total_distance_delta(mut locations1: Vec<u32>, mut locations2: Vec<u32>) -> u32 {
    locations1.sort_unstable();
    locations2.sort_unstable();
    locations1
        .iter()
        .zip(&locations2)
        .map(|(l1, l2)| l1.abs_diff(*l2))
        .sum()
}

fn calculate_similarity_score(locations1: &[u32], locations2: &[u32]) -> u32 {
    let locations2_map = locations2.iter().fold(HashMap::new(), |mut acc, &loc| {
        *acc.entry(loc).or_insert(0) += 1;
        acc
    });
    locations1
        .iter()
        .filter_map(|&location| locations2_map.get(&location).map(|count| location * count))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let locations = load_locations(input);
    Some(calculate_total_distance_delta(locations.0, locations.1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let locations = load_locations(input);
    Some(calculate_similarity_score(&locations.0, &locations.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
