use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

#[derive(Clone)]
struct Rule(u64, u64);

fn check_valid_update(update: &Vec<u64>, rules: &Vec<Rule>) -> bool {
    let index_list = update
        .iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx))
        .collect::<HashMap<u64, usize>>();

    rules.iter().all(|rule| {
        let first_idx = index_list.get(&rule.0);
        let second_idx = index_list.get(&rule.1);

        match (first_idx, second_idx) {
            (Some(first), Some(second)) => first < second,
            _ => true,
        }
    })
}

fn fix_invalid_update(update: &Vec<u64>, rules: &Vec<Rule>) -> Vec<u64> {
    let mut root_nums = update
        .iter()
        .filter(|num| {
            !rules.iter().any(|rule| &rule.1 == *num)
                || rules
                    .iter()
                    .filter(|rule| (rule.1 == **num))
                    .all(|rule| !update.contains(&rule.0))
        })
        .map(|num| *num)
        .collect::<Vec<_>>();

    let mut bound_nums = update
        .iter()
        .filter(|num| rules.iter().any(|rule| &rule.1 == *num))
        .map(|num| *num)
        .collect::<Vec<_>>();

    let mut idx = 0;

    loop {
        if idx >= bound_nums.len() {
            idx = 0;
            continue;
        }

        let Some(next) = bound_nums.get(idx) else {
            break;
        };

        let nums_before = rules
            .iter()
            .filter(|rule| rule.1 == *next && update.contains(&rule.0))
            .map(|rule| rule.0)
            .collect::<Vec<_>>();

        if !nums_before.iter().all(|num| root_nums.contains(num)) {
            idx += 1;
            continue;
        }

        if !root_nums.contains(next) {
            root_nums.push(*next);
        }

        bound_nums.remove(idx);

        if bound_nums.is_empty() {
            break;
        }
    }

    root_nums
}

fn get_rules_and_updates(input: &str) -> (Vec<Rule>, Vec<Vec<u64>>) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().expect("malformed input");
    let updates = parts.next().expect("malformed input");

    let rules = rules
        .lines()
        .flat_map(|line| {
            let mut parts = line.split('|');
            let first = parts.next()?.parse::<u64>().ok()?;
            let second = parts.next()?.parse::<u64>().ok()?;

            Some(Rule(first, second))
        })
        .collect::<Vec<_>>();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|num| num.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

fn first_part(input: &str) -> u64 {
    let (rules, updates) = get_rules_and_updates(input);

    updates
        .iter()
        .filter(|update| check_valid_update(update, &rules))
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum()
}

fn second_part(input: &str) -> u64 {
    let (rules, updates) = get_rules_and_updates(input);

    updates
        .iter()
        .filter(|update| !check_valid_update(update, &rules))
        .map(|update| fix_invalid_update(update, &rules))
        .map(|update| {
            if !update.is_empty() {
                let mid = update.len() / 2;
                update[mid]
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_updates() {
        let input = include_str!("input_test");

        let (rules, updates) = get_rules_and_updates(input);

        let count_valid = updates
            .iter()
            .filter(|update| check_valid_update(update, &rules))
            .count();

        assert_eq!(count_valid, 3);
    }

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");
        assert_eq!(first_part(input), 143);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");
        assert_eq!(second_part(input), 123);
    }
}
