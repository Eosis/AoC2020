use std::fs;

pub fn solve_part_1() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day18.txt").unwrap());
    println!("{}", part_1(input));
    Ok(())
}

pub fn solve_part_2() -> Result<(), ()> {
    let input = parse_input(&fs::read_to_string("./inputs/day18.txt").unwrap());
    println!("{}", part_2(input));
    Ok(())
}

fn part_1(input: Vec<Vec<Token>>) -> u64 {
    input
        .into_iter()
        .map(|line_tokens| get_value_from_expr_part_1(&mut line_tokens.into_iter()))
        .sum()
}

fn part_2(input: Vec<Vec<Token>>) -> u64 {
    input
        .into_iter()
        .map(|line_tokens| get_value_from_expr_part_2(&mut line_tokens.into_iter()))
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input.split('\n').map(|line| parse_line(line)).collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Token {
    OParen,
    CParen,
    Multiply,
    Add,
    Number(u64),
}

fn parse_line(mut input: &str) -> Vec<Token> {
    input = input.trim_start();
    let mut tokens = vec![];
    while !input.is_empty() {
        let next = input.chars().next().unwrap();
        tokens.push(match next {
            '(' => {
                input = &input[1..];
                Token::OParen
            }
            ')' => {
                input = &input[1..];
                Token::CParen
            }
            '*' => {
                input = &input[1..];
                Token::Multiply
            }
            '+' => {
                input = &input[1..];
                Token::Add
            }
            _ => {
                let number = input.split(&[' ', ')'][..]).next().unwrap();
                input = &input[number.len()..];
                Token::Number(number.parse::<u64>().unwrap())
            }
        });
        input = input.trim_start();
    }
    tokens
}

fn get_value_from_expr_part_2<T: Iterator<Item = Token>>(iter: &mut T) -> u64 {
    // let input = "1 + (2 * 3) + (4 * (5 + 6))";
    let mut first_pass: Vec<Token> = vec![];
    // First Pass, resolve all the parens, get the tokens
    while let Some(x) = iter.next() {
        match x {
            Token::Number(x) => first_pass.push(Token::Number(x)),
            Token::Multiply => first_pass.push(Token::Multiply),
            Token::Add => first_pass.push(Token::Add),
            Token::OParen => first_pass.push(Token::Number(get_value_from_expr_part_2(iter))),
            Token::CParen => break,
        }
    }

    // Second Pass, resolve all the adds
    let mut second_pass_result: Vec<Token> = vec![];
    let mut lhs: Option<u64> = None;
    let mut rhs: Option<u64> = None;

    for item in first_pass.iter() {
        match item {
            Token::Number(x) => {
                if lhs.is_none() {
                    lhs = Some(*x);
                } else {
                    rhs = Some(*x);
                }
            }
            Token::Multiply => {
                second_pass_result.push(Token::Number(lhs.take().unwrap()));
                second_pass_result.push(Token::Multiply);
            }
            _ => (),
        }

        if lhs.is_some() && rhs.is_some() {
            lhs = Some(lhs.unwrap() + rhs.unwrap());
            rhs = None;
        }
    }

    second_pass_result.push(Token::Number(lhs.take().unwrap()));

    // Only multiplies left:
    second_pass_result
        .into_iter()
        .filter_map(|token| if let Token::Number(x) = token { Some(x) } else { None })
        .product()
}

fn get_value_from_expr_part_1<T: Iterator<Item = Token>>(iter: &mut T) -> u64 {
    let mut lhs = None;
    let mut rhs = None;
    let mut op = None;
    loop {
        match iter.next() {
            Some(Token::Number(x)) => {
                if lhs.is_none() {
                    lhs = Some(x);
                } else {
                    rhs = Some(x);
                }
            }
            Some(Token::Multiply) => op = Some(Token::Multiply),
            Some(Token::Add) => op = Some(Token::Add),
            Some(Token::OParen) => {
                if lhs.is_none() {
                    lhs = Some(get_value_from_expr_part_1(iter))
                } else {
                    rhs = Some(get_value_from_expr_part_1(iter))
                }
            }
            Some(Token::CParen) | None => return lhs.unwrap(),
        }
        if lhs.is_some() && rhs.is_some() {
            match op {
                Some(Token::Multiply) => {
                    lhs = Some(lhs.unwrap() * rhs.unwrap());
                }
                Some(Token::Add) => {
                    lhs = Some(lhs.unwrap() + rhs.unwrap());
                }
                _ => panic!("Got a LHS and RHS without an OP!"),
            }
            rhs = None;
            op = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashbrown::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_tokenizing() {
        let input = "(1 * 2 + 3 * ( 4 + 5) )";
        let tokens = parse_line(input);
        let expected = vec![
            Token::OParen,
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::Add,
            Token::Number(3),
            Token::Multiply,
            Token::OParen,
            Token::Number(4),
            Token::Add,
            Token::Number(5),
            Token::CParen,
            Token::CParen,
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_results() {
        let input = "(1 * 2 + 3 * ( 4 + 5) )";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_1(&mut iter), 45);

        let input = "2 * 3 + (4 * 5)";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_1(&mut iter), 26);

        let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_1(&mut iter), 437);

        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_1(&mut iter), 12240);

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_1(&mut iter), 13632);
    }

    #[test]
    fn test_get_value_from_expr_part_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let tokens = parse_line(input);
        let mut iter = tokens.into_iter();
        assert_eq!(get_value_from_expr_part_2(&mut iter), 51);

        // let input = "2 * 3 + (4 * 5)";
        // let tokens = parse_line(input);
        // let mut iter = tokens.into_iter();
        // assert_eq!(get_value_from_expr_part_2(&mut iter), 46);
        //
        // let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        // let tokens = parse_line(input);
        // let mut iter = tokens.into_iter();
        // assert_eq!(get_value_from_expr_part_2(&mut iter), 669060);
    }
}
