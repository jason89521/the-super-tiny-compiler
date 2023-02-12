use crate::LispNode;

#[derive(Debug, PartialEq)]
pub enum JsNode {
    Program(Vec<JsNode>),
    ExpressionStatement(Box<JsNode>),
    CallExpression {
        callee: Box<JsNode>,
        arguments: Vec<JsNode>,
    },
    Identifier(String),
    NumberLiteral(String),
}

fn traverse_node(ctx: &mut Vec<JsNode>, node: &LispNode, parent: Option<&LispNode>) {
    match node {
        LispNode::Program(body) => {
            body.iter()
                .for_each(|child_node| traverse_node(ctx, child_node, Some(&node)));
        }
        LispNode::CallExpression { params, name } => {
            let mut arguments = vec![];
            params
                .iter()
                .for_each(|child_node| traverse_node(&mut arguments, child_node, Some(node)));
            if let Some(LispNode::CallExpression { .. }) = parent {
                ctx.push(JsNode::CallExpression {
                    callee: Box::new(JsNode::Identifier(name.clone())),
                    arguments,
                });
            } else {
                ctx.push(JsNode::ExpressionStatement(Box::new(
                    JsNode::CallExpression {
                        callee: Box::new(JsNode::Identifier(name.clone())),
                        arguments,
                    },
                )));
            }
        }
        LispNode::NumberLiteral(n) => {
            ctx.push(JsNode::NumberLiteral(n.clone()));
        }
    }
}

pub fn transformer(ast: &LispNode) -> JsNode {
    let mut context = vec![];
    traverse_node(&mut context, ast, None);

    JsNode::Program(context)
}
