use std::iter::Peekable;

fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    // println!("Second part: {}", second_part(input));

}

#[derive(Debug, PartialEq)]
enum Token {
    Mul(char), // equals a char in mul
    LeftParen,
    Number(u32),
    RightParen,
    Comma,
    Invalid,
}

struct Tokenizer<'a> {
    chars: Peekable<std::str::Chars<'a>>,
}

impl Tokenizer<'_> {
    fn new(input: &str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;

        match c {
            c if c == 'm' || c == 'u' || c == 'l' => Some(Token::Mul(c)),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            ',' => Some(Token::Comma),
            c if c.is_digit(10) => match c.to_digit(10) {
                Some(d) => {
                    let mut digits = vec![d];

                    while digits.len() < 3 {
                        match self.chars.peek() {
                            Some(c) if c.is_digit(10) => {
                                let d = c.to_digit(10).unwrap();
                                digits.push(d);
                                self.chars.next();
                            }
                            _ => break,
                        }
                    }

                    let number = digits.iter().fold(0, |acc, num| acc * 10 + num);

                    Some(Token::Number(number))
                }
                None => Some(Token::Invalid),
            },
            _ => Some(Token::Invalid),
        }
    }
}

struct Multiplication(u32, u32);

impl From<Multiplication> for u64 {
    fn from(multiplication: Multiplication) -> u64 {
        multiplication.0 as u64 * multiplication.1 as u64
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    Tokenizer::new(input).collect()
}

fn parse(tokens: Vec<Token>) -> Vec<Multiplication> {
    const MAX_TOKENS: usize = 4;
    let mut multiplications = Vec::new();

    let start_idx = tokens
        .windows(8)
        .enumerate()
        .filter(|(_, window)| {
            window[0] == Token::Mul('m')
                && window[1] == Token::Mul('u')
                && window[2] == Token::Mul('l')
                && window[3] == Token::LeftParen
                && matches!(window[4], Token::Number(_))
                && window[5] == Token::Comma
                && matches!(window[4], Token::Number(_))
                && window[7] == Token::RightParen
        })
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let slices = start_idx
        .iter()
        .flat_map(|idx| tokens.get(*idx + 4..idx + 4 + MAX_TOKENS))
        .collect::<Vec<_>>();

    for slice in slices {
        let numbers = slice
            .iter()
            .filter_map(|token| match token {
                Token::Number(n) => Some(*n),
                _ => None,
            })
            .collect::<Vec<_>>();

        if numbers.len() == 2 {
            multiplications.push(Multiplication(numbers[0], numbers[1]));
        }
    }

    multiplications
}

fn first_part(input: &str) -> u64 {
    let tokens = tokenize(input);

    parse(tokens)
        .into_iter()
        .map(|multiplication| u64::from(multiplication))
        .sum()
}

fn second_part(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 161);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");

        assert_eq!(second_part(input), 48);
    }
}
