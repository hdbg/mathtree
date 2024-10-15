use std::{any::Any, i128, iter::Peekable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Parenthesis(Direction),
    Div,
    Mul,
    Sub,
    Add,
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Parenthesis(Direction::Open)),
            ')' => Ok(Self::Parenthesis(Direction::Close)),
            '/' => Ok(Self::Div),
            '*' => Ok(Self::Mul),
            '-' => Ok(Self::Sub),
            '+' => Ok(Self::Add),
            _ => Err(()),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Open,
    Close,
}
#[derive(Debug, strum::EnumIs)]
pub enum Token {
    Literal(i128),
    Operator(Operator),
    Variable(String),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenizerState {
    None,
    NumberParsing { is_negative: bool, chars: Vec<char> },
    VariableParsing { chars: Vec<char> },
}

fn parse_while(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    start: char,
    cond: impl Fn(char) -> bool,
) -> String {
    let mut buffer = String::from(start);

    loop {
        let is_next_alphabetic = iter.peek().filter(|x| cond(**x)).is_some();
        if !is_next_alphabetic {
            break;
        }

        let next_char = iter.next().unwrap();

        buffer.push(next_char);
    }
    buffer
}

fn fix_tokens(tokens: &mut Vec<Token>) {
    let mut curr_index = 0;

    let mut indexes_to_remove = Vec::new();

    while curr_index < tokens.len() {
        let has_previous_sub =
            curr_index > 1 && matches!(&tokens[curr_index - 1], Token::Operator(Operator::Sub));
        let has_2previous_op = curr_index > 2 && (&tokens[curr_index - 2]).is_operator();
        let curr_item = &mut tokens[curr_index];

        if has_2previous_op && has_previous_sub {
            if let Token::Literal(ref mut number) = curr_item {
                *number = -*number;
                indexes_to_remove.push(curr_index - 1);
            }
        }

        curr_index += 1;
    }

    for index in indexes_to_remove {
        tokens.remove(index);
    }
}

fn tokenize(data: &str) -> Vec<Token> {
    // add whitespace filter as we don't care
    let mut chunks = data.chars().filter(|x| !char::is_whitespace(*x)).peekable();

    let mut result = Vec::new();

    let mut state = TokenizerState::None;

    while let Some(curr_token) = chunks.next() {
        if curr_token.is_alphabetic() {
            let mut new_name: String = parse_while(&mut chunks, curr_token, |x| x.is_alphabetic());
            result.push(Token::Variable(new_name));
            continue;
        }

        if curr_token.is_digit(10) {
            let mut new_name: String = parse_while(&mut chunks, curr_token, |x| x.is_digit(10));
            let number = new_name.as_str().parse().unwrap();
            result.push(Token::Literal(number));
            continue;
        }

        let Ok(operator) = Operator::try_from(curr_token) else {
            panic!("invalid input");
        };

        result.push(Token::Operator(operator));
    }

    fix_tokens(&mut result);

    result
}

fn main() {
    println!("{:#?}", tokenize("1 + 2 - (4 * 5 + -20 - 40) / -60"));
}
