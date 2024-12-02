fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn build_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .flat_map(|line| {
            let items = line.split("   ").collect::<Vec<_>>();
            let left = items.first();
            let right = items.last();

            if let (Some(left), Some(right)) = (left, right) {
                if let (Ok(left), Ok(right)) = (left.parse::<u64>(), right.parse::<u64>()) {
                    Some((left, right))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unzip()
}

fn first_part(input: &str) -> u64 {
    let (mut left, mut right) = build_lists(input);

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn second_part(input: &str) -> u64 {
    let (left, right) = build_lists(input);
    left.iter()
        .map(|left_num| {
            let count = right
                .iter()
                .filter(|&right_num| right_num == left_num)
                .count();
            count as u64 * left_num
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 11);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");
        assert_eq!(second_part(input), 31);
    }
}
