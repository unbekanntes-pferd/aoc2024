use std::iter::Peekable;

fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

#[derive(Debug, PartialEq)]
enum Token {
    Start,
    LeftParen,
    Number(u32),
    RightParen,
    Comma,
    Invalid,
    Enabled,
    Disabled,
}

struct Tokenizer<'c> {
    chars: Peekable<std::str::Chars<'c>>,
}

impl Tokenizer<'_> {
    fn new(input: &str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

struct Parser<'t> {
    tokens: Peekable<std::slice::Iter<'t, Token>>,
    enabled: bool,
}

impl<'t> Parser<'t> {
    fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.iter().peekable(),
            enabled: true,
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = Multiplication;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Enabled => {
                    self.enabled = true;
                    continue;
                }
                Token::Disabled => {
                    self.enabled = false;
                    continue;
                }
                Token::Start if self.enabled => {
                    let mut num1 = None;
                    let mut num2 = None;

                    if let Some(_token @ Token::LeftParen) = self.tokens.peek() {
                        self.tokens.next();
                    } else {
                        continue;
                    };

                    if let Some(&Token::Number(num)) = self.tokens.peek() {
                        num1 = Some(*num);
                        self.tokens.next();
                    } else {
                        continue;
                    }

                    if let Some(_token @ Token::Comma) = self.tokens.peek() {
                        self.tokens.next();
                    } else {
                        continue;
                    }

                    if let Some(_token @ Token::Number(num)) = self.tokens.peek() {
                        num2 = Some(*num);
                        self.tokens.next();
                    } else {
                        continue;
                    }

                    if let Some(_token @ Token::RightParen) = self.tokens.peek() {
                        self.tokens.next();
                    } else {
                        continue;
                    }

                    if let (Some(num1), Some(num2)) = (num1, num2) {
                        return Some(Multiplication(num1, num2));
                    }
                }
                _ => continue,
            }
        }
        None
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.chars.next()?;

        match c {
            c if c == 'm' => match self.chars.peek() {
                Some('u') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('l') => {
                            self.chars.next();
                            Some(Token::Start)
                        }
                        _ => Some(Token::Invalid),
                    }
                }
                _ => Some(Token::Invalid),
            },
            c if c == 'd' => match self.chars.peek() {
                Some('o') => {
                    self.chars.next();

                    match self.chars.peek() {
                        Some('n') => {
                            self.chars.next();
                            let first = self.chars.next();
                            let second = self.chars.next();
                            let third = self.chars.next();
                            let fourth = self.chars.next();

                            match (first, second, third, fourth) {
                                (Some('\''), Some('t'), Some('('), Some(')')) => {
                                    Some(Token::Disabled)
                                }
                                _ => return Some(Token::Invalid),
                            }
                        }
                        Some('(') => {
                            self.chars.next();
                            match self.chars.peek() {
                                Some(')') => {
                                    self.chars.next();
                                    Some(Token::Enabled)
                                }
                                _ => return Some(Token::Invalid),
                            }
                        }
                        _ => Some(Token::Invalid),
                    }
                }
                _ => Some(Token::Invalid),
            },
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

#[derive(Debug, Copy, Clone, PartialEq)]
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
        .windows(6)
        .enumerate()
        .filter(|(_, window)| {
            window[0] == Token::Start
                && window[1] == Token::LeftParen
                && matches!(window[2], Token::Number(_))
                && window[3] == Token::Comma
                && matches!(window[4], Token::Number(_))
                && window[5] == Token::RightParen
        })
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let slices = start_idx
        .iter()
        .flat_map(|idx| tokens.get(*idx + 2..idx + 2 + MAX_TOKENS))
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
    let tokens = tokenize(input);

    Parser::new(&tokens)
        .filter_map(|multiplication| Some(u64::from(multiplication)))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{first_part, second_part, tokenize, Multiplication, Parser, Token};

    #[test]
    fn test_tokenizer() {
        let input = "mul(123, 456)";
        let expected = vec![
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Invalid,
            Token::Number(456),
            Token::RightParen,
        ];

        assert_eq!(tokenize(input), expected);

        let input = "do()373mul(123,456)don't()mul(789,101)";

        let expected = vec![
            Token::Enabled,
            Token::Number(373),
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Number(456),
            Token::RightParen,
            Token::Disabled,
            Token::Start,
            Token::LeftParen,
            Token::Number(789),
            Token::Comma,
            Token::Number(101),
            Token::RightParen,
        ];

        assert_eq!(tokenize(input), expected);

        let input = "do()123mul";

        let expected = vec![Token::Enabled, Token::Number(123), Token::Start];

        assert_eq!(tokenize(input), expected);

        let input = "don't()456mul";
        let expected = vec![Token::Disabled, Token::Number(456), Token::Start];

        assert_eq!(tokenize(input), expected);

        let input = "mul(do()1,2";

        let expected = vec![
            Token::Start,
            Token::LeftParen,
            Token::Enabled,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
        ];

        assert_eq!(tokenize(input), expected);

        let input = "1,don't()2)";

        let expected = vec![
            Token::Number(1),
            Token::Comma,
            Token::Disabled,
            Token::Number(2),
            Token::RightParen,
        ];

        assert_eq!(tokenize(input), expected);

        let inputs = [
            "d123",
            "do123",
            "do(123",
            "don123",
            "don't123",
            "don't(123",
            "mul(d,2)",
            "mul(do,2)",
            "mul(don't(,2)",
        ];

        for input in inputs {
            let tokens = tokenize(input);

            dbg!(&tokens);
            assert!(tokens.iter().all(|t| matches!(
                t,
                Token::Invalid
                    | Token::Number(_)
                    | Token::Start
                    | Token::Comma
                    | Token::LeftParen
                    | Token::RightParen
            )));
        }
    }

    #[test]
    fn test_tokenizer_malformed_state_changes() {
        let test_cases = [
            (
                "do(don't())",
                vec![Token::Invalid, Token::Disabled, Token::RightParen],
            ),
            ("do()don't()", vec![Token::Enabled, Token::Disabled]),
        ];

        for (input, expected) in test_cases {
            assert_eq!(tokenize(input), expected);
        }
    }

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::Enabled,
            Token::Number(373),
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Number(456),
            Token::RightParen,
            Token::Disabled,
            Token::Start,
            Token::LeftParen,
            Token::Number(789),
            Token::Comma,
            Token::Number(101),
            Token::RightParen,
        ];

        let parser = Parser::new(&tokens);

        let multiplications = parser.collect::<Vec<_>>();

        dbg!(&multiplications);

        let first = multiplications.first().unwrap();

        assert_eq!(u64::from(*first), 123 * 456);
    }

    #[test]
    fn test_parser_with_state_change_mid_pattern() {
        let tokens = vec![
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Enabled,
            Token::Number(456),
            Token::RightParen,
        ];

        let parser = Parser::new(&tokens);
        let multiplications = parser.collect::<Vec<_>>();

        assert!(multiplications.is_empty());
    }

    #[test]
    fn test_parser_state_change_effects() {
        let tokens = vec![
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Disabled, // State change mid-pattern
            Token::Enabled,  // State change mid-pattern
            Token::Disabled, // State change mid-pattern
            Token::Number(456),
            Token::RightParen,
            Token::Start, // Next multiplication
            Token::LeftParen,
            Token::Number(789),
            Token::Comma,
            Token::Number(101),
            Token::RightParen,
        ];

        let parser = Parser::new(&tokens);
        let multiplications = parser.collect::<Vec<_>>();

        dbg!(&multiplications);

        assert!(multiplications.is_empty());
    }

    #[test]
    fn test_parser_valid_state_changes() {
        let tokens = vec![
            Token::Start,
            Token::LeftParen,
            Token::Number(123),
            Token::Comma,
            Token::Number(456),
            Token::RightParen,
            Token::Disabled,
            Token::Start,
            Token::LeftParen,
            Token::Number(789),
            Token::Comma,
            Token::Number(101),
            Token::RightParen,
            Token::Enabled,
            Token::Start,
            Token::LeftParen,
            Token::Number(111),
            Token::Comma,
            Token::Number(333),
            Token::RightParen,
        ];

        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.next(), Some(Multiplication(123, 456))); // Enabled
        assert_eq!(parser.next(), Some(Multiplication(111, 333))); // Enabled
    }

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 161);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test2");

        assert_eq!(second_part(input), 48);
    }
}
