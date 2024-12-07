use std::rc::Rc;

fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

enum Operator {
    Add,
    Multiply,
    Concat
}

const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Multiply];
const EXTENDED_OPERATORS: [Operator; 3] = [Operator::Add, Operator::Multiply, Operator::Concat];

#[derive(Debug)]
enum Expression {
    Number(u32),
    Combined(Rc<Expression>, u32),
    Empty,
}

impl Expression {
    fn evaluate(&self, operators: &[Operator]) -> Vec<u64> {
        let mut results = Vec::new();
        match self {
            Self::Empty => (),
            Self::Number(num) => {
                results.push(*num as u64);
            }
            Self::Combined(exp, number) => {
                let exp_results = exp.evaluate(operators);
                for exp_result in exp_results {
                    for op in operators {
                        let result = match op {
                            Operator::Add => exp_result + *number as u64,
                            Operator::Multiply => exp_result * *number as u64,
                            Operator::Concat => format!("{exp_result}{number}").parse::<u64>().expect("malformed number")
                        };
                        results.push(result);
                    }
                }
            }
        }
        results
    }
}

fn first_part(input: &str) -> u64 {
    let expressions = parse(input);
    expressions
        .iter()
        .filter(|(expected, expression)| {
            let results = expression.evaluate(&OPERATORS);
            results.contains(expected)
        })
        .map(|(num, _)| {
            num
        })
        .sum()
}

fn second_part(input: &str) -> u64 {
    let expressions = parse(input);
    expressions
        .iter()
        .filter(|(expected, expression)| {
            let results = expression.evaluate(&EXTENDED_OPERATORS);
            results.contains(expected)
        })
        .map(|(num, _)| {
            num
        })
        .sum()
}

impl From<Vec<u32>> for Expression {
    fn from(nums: Vec<u32>) -> Self {
        nums.into_iter()
            .fold(Expression::Empty, |acc, num| match acc {
                Expression::Empty => Expression::Number(num),
                exp => Expression::Combined(Rc::new(exp), num),
            })
    }
}

fn parse(input: &str) -> Vec<(u64, Expression)> {
    input
        .lines()
        .flat_map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();

            let test_value = parts.first()?.parse::<u64>().ok()?;

            let equation_nums = parts
                .last()?
                .split(' ')
                .flat_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>();
            Some((test_value, equation_nums.into()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 3749);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");
        assert_eq!(second_part(input), 11387);
    }
}
