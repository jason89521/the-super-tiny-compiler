use super::Token;

#[derive(Debug, PartialEq)]
pub enum LispNode {
    CallExpression { name: String, params: Vec<LispNode> },
    NumberLiteral(String),
    Program(Vec<LispNode>),
}

pub fn parser(tokens: &[Token]) -> LispNode {
    let mut current = 0;

    let mut body = vec![];
    while current < tokens.len() {
        body.push(walk(&mut current, tokens));
    }

    LispNode::Program(body)
}

fn walk(current: &mut usize, tokens: &[Token]) -> LispNode {
    let token = &tokens[*current];
    match token {
        Token::Number(n) => {
            *current += 1;
            LispNode::NumberLiteral(n.clone())
        }
        Token::LeftParen => {
            // Increment `current` to skip the parenthesis since we don't care about it in our AST.
            *current += 1;
            let mut params = vec![];
            let token = &tokens[*current];
            if let Token::Name(name) = token {
                // To skip the name token.
                *current += 1;
                let mut token = &tokens[*current];
                while token != &Token::RightParen {
                    params.push(walk(current, tokens));
                    token = &tokens[*current];
                }

                // Finally we will increment `current` to skip the closing parenthesis.
                *current += 1;

                return LispNode::CallExpression {
                    name: name.clone(),
                    params,
                };
            } else {
                panic!("Didn't find a name token.");
            }
        }
        _ => {
            println!("{:#?}", token);
            panic!("Encounter an unexpected token.");
        }
    }
}
