fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

fn build_reports(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .flat_map(|num| num.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .collect()
}

trait SafeReport {
    fn is_safe(&self) -> bool;
}

impl SafeReport for Vec<u64> {
    fn is_safe(&self) -> bool {
        let all_increasing = self.windows(2).all(|window| window[0] < window[1]);
        let all_decreasing = self.windows(2).all(|window| window[0] > window[1]);
        let max_diff = self
            .windows(2)
            .map(|window| window[0].abs_diff(window[1]))
            .max()
            .unwrap_or(0);

        (all_increasing || all_decreasing) && max_diff <= 3
    }
}

fn first_part(input: &str) -> u64 {
    let reports = build_reports(input);

    reports.iter().fold(0, |mut acc, report| {
        if report.is_safe() {
            acc += 1;
        }

        acc
    })
}

fn second_part(input: &str) -> u64 {
    let reports = build_reports(input);
    reports.iter().fold(0, |mut acc, report| {
        if report.is_safe() {
            return acc + 1;
        }

        let report_copy = report.clone();

        for (idx, _) in report.into_iter().enumerate() {
            let mut report_copy = report_copy.clone();
            report_copy.remove(idx);

            if report_copy.is_safe() {
                acc += 1;
                break;
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 2);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");

        assert_eq!(second_part(input), 4);
    }
}
