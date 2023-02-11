#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Number(String),
    Name(String),
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut current = 0;
    let mut tokens = vec![];
    let input = input.chars().collect::<Vec<char>>();

    while current < input.len() {
        let char = input[current];
        match char {
            char if char == '(' => {
                tokens.push(Token::LeftParen);
                current += 1;
            }
            char if char == ')' => {
                tokens.push(Token::RightParen);
                current += 1;
            }
            char if char.is_whitespace() => {
                current += 1;
                continue;
            }
            char if char.is_digit(10) => {
                let mut char = char;
                let mut number = vec![];
                while char.is_digit(10) {
                    number.push(char);
                    current += 1;
                    char = input[current];
                }
                tokens.push(Token::Number(number.iter().collect()));
            }
            char if char.is_alphabetic() => {
                let mut char = char;
                let mut name = vec![];
                while char.is_alphabetic() {
                    name.push(char);
                    current += 1;
                    char = input[current];
                }
                tokens.push(Token::Name(name.iter().collect()));
            }
            _ => {
                panic!("Unknown character when tokenizing")
            }
        }
    }

    tokens
}
