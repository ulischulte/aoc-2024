use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Clone)]
struct OrderingRule {
    left: u32,
    right: u32,
}

struct Manual {
    ordering_rules: Vec<OrderingRule>,
    pages: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Manual {
    let mut sections = input.split("\n\n");

    let ordering_rules = parse_ordering_rules(sections.next().unwrap_or(""));
    let pages = parse_pages(sections.next().unwrap_or(""));
    Manual {
        ordering_rules,
        pages,
    }
}

fn parse_ordering_rules(rules_section: &str) -> Vec<OrderingRule> {
    fn parse_ordering_rule(rule_str: &str) -> OrderingRule {
        let rule_parts: Vec<u32> = rule_str.split('|').filter_map(|s| s.parse().ok()).collect();
        OrderingRule {
            left: rule_parts[0],
            right: rule_parts[1],
        }
    }
    rules_section.par_lines().map(parse_ordering_rule).collect()
}

fn parse_pages(pages_section: &str) -> Vec<Vec<u32>> {
    pages_section
        .par_lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

fn extract_pages<F>(manual: &Manual, condition: F) -> Vec<Vec<u32>>
where
    F: Sync + Fn(&[u32], &[OrderingRule]) -> bool,
{
    manual
        .pages
        .par_iter()
        .filter(|&page| condition(page, &manual.ordering_rules))
        .cloned()
        .collect()
}

fn extract_pages_in_right_order(manual: &Manual) -> Vec<Vec<u32>> {
    extract_pages(manual, |page, rules| {
        filter_rules_for_page(page, rules)
            .par_iter()
            .all(|rule| is_in_correct_order(&preprocess_indexes(&page), &rule))
    })
}

fn extract_pages_in_wrong_order(manual: &Manual) -> Vec<Vec<u32>> {
    extract_pages(manual, |page, rules| {
        filter_rules_for_page(page, rules)
            .par_iter()
            .any(|rule| !is_in_correct_order(&preprocess_indexes(&page), &rule))
    })
}

fn is_in_correct_order(index_map: &HashMap<u32, usize>, rule: &OrderingRule) -> bool {
    match (index_map.get(&rule.left), index_map.get(&rule.right)) {
        (Some(&left_idx), Some(&right_idx)) => left_idx < right_idx,
        _ => true,
    }
}

fn filter_rules_for_page(page: &[u32], rules: &[OrderingRule]) -> Vec<OrderingRule> {
    rules
        .iter()
        .filter(|rule| page.contains(&rule.left) && page.contains(&rule.right))
        .cloned()
        .collect()
}

fn preprocess_indexes(page: &[u32]) -> HashMap<u32, usize> {
    page.iter().enumerate().map(|(i, &val)| (val, i)).collect()
}

fn reorder_pages(rules: &[OrderingRule], pages: &mut [Vec<u32>]) {
    pages
        .par_iter_mut()
        .for_each(|page| reorder_page(rules, page));
}

fn reorder_page(rules: &[OrderingRule], page: &mut Vec<u32>) {
    page.sort_unstable_by(|&a, &b| {
        if rules.iter().any(|rule| rule.left == a && rule.right == b) {
            std::cmp::Ordering::Less
        } else if rules.iter().any(|rule| rule.left == b && rule.right == a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

fn get_middle_page_sum(pages: &[Vec<u32>]) -> u32 {
    pages
        .par_iter()
        .filter_map(|page| page.get(page.len() / 2))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pages = parse_input(input);
    let ordered_pages = extract_pages_in_right_order(&pages);
    Some(get_middle_page_sum(&ordered_pages))
}

pub fn part_two(input: &str) -> Option<u32> {
    let manual = parse_input(input);
    let mut pages = extract_pages_in_wrong_order(&manual);
    reorder_pages(&manual.ordering_rules, &mut pages);
    Some(get_middle_page_sum(&pages))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
