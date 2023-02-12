mod code_generator;
mod parser;
mod tokenizer;
mod transformer;

pub use code_generator::code_generator;
pub use parser::{parser, LispNode};
pub use tokenizer::{tokenizer, Token};
pub use transformer::{transformer, JsNode};

pub fn compiler(input: &str) -> String {
    let tokens = tokenizer(input);
    let ast = parser(&tokens);
    let new_ast = transformer(&ast);
    code_generator(&new_ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "(add 2 (subtract 4 2))";
    const OUTPUT: &str = "add(2, subtract(4, 2));";

    #[test]
    fn tokenizer_correct() {
        let tokens = vec![
            Token::LeftParen,
            Token::Name("add".to_string()),
            Token::Number("2".to_string()),
            Token::LeftParen,
            Token::Name("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::RightParen,
            Token::RightParen,
        ];

        assert_eq!(tokens, tokenizer(INPUT));
    }

    #[test]
    fn parser_correct() {
        let tokens = [
            Token::LeftParen,
            Token::Name("add".to_string()),
            Token::Number("2".to_string()),
            Token::LeftParen,
            Token::Name("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::RightParen,
            Token::RightParen,
        ];
        let ast = LispNode::Program(vec![LispNode::CallExpression {
            name: "add".to_string(),
            params: vec![
                LispNode::NumberLiteral("2".to_string()),
                LispNode::CallExpression {
                    name: "subtract".to_string(),
                    params: vec![
                        LispNode::NumberLiteral("4".to_string()),
                        LispNode::NumberLiteral("2".to_string()),
                    ],
                },
            ],
        }]);

        assert_eq!(ast, parser(&tokens));
    }

    #[test]
    fn transformer_correct() {
        let ast = LispNode::Program(vec![LispNode::CallExpression {
            name: "add".to_string(),
            params: vec![
                LispNode::NumberLiteral("2".to_string()),
                LispNode::CallExpression {
                    name: "subtract".to_string(),
                    params: vec![
                        LispNode::NumberLiteral("4".to_string()),
                        LispNode::NumberLiteral("2".to_string()),
                    ],
                },
            ],
        }]);
        let new_ast = JsNode::Program(vec![JsNode::ExpressionStatement(Box::new(
            JsNode::CallExpression {
                callee: Box::new(JsNode::Identifier("add".to_string())),
                arguments: vec![
                    JsNode::NumberLiteral("2".to_string()),
                    JsNode::CallExpression {
                        callee: Box::new(JsNode::Identifier("subtract".to_string())),
                        arguments: vec![
                            JsNode::NumberLiteral("4".to_string()),
                            JsNode::NumberLiteral("2".to_string()),
                        ],
                    },
                ],
            },
        ))]);

        assert_eq!(new_ast, transformer(&ast));
    }

    #[test]
    fn code_generator_correct() {
        let node = JsNode::Program(vec![JsNode::ExpressionStatement(Box::new(
            JsNode::CallExpression {
                callee: Box::new(JsNode::Identifier("add".to_string())),
                arguments: vec![
                    JsNode::NumberLiteral("2".to_string()),
                    JsNode::CallExpression {
                        callee: Box::new(JsNode::Identifier("subtract".to_string())),
                        arguments: vec![
                            JsNode::NumberLiteral("4".to_string()),
                            JsNode::NumberLiteral("2".to_string()),
                        ],
                    },
                ],
            },
        ))]);
        assert_eq!(OUTPUT, code_generator(&node));
    }

    #[test]
    fn compiler_correct() {
        assert_eq!(OUTPUT, compiler(INPUT));
    }
}
