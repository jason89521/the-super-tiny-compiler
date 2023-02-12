use crate::JsNode;

pub fn code_generator(node: &JsNode) -> String {
    match node {
        JsNode::Program(body) => body
            .iter()
            .map(code_generator)
            .collect::<Vec<String>>()
            .join("\n"),
        JsNode::ExpressionStatement(expression) => code_generator(expression) + ";",
        JsNode::CallExpression { callee, arguments } => {
            let name = code_generator(callee);
            let arguments = arguments
                .iter()
                .map(code_generator)
                .collect::<Vec<String>>()
                .join(", ");
            name + "(" + &arguments + ")"
        }
        JsNode::Identifier(name) => name.clone(),
        JsNode::NumberLiteral(num) => num.clone(),
    }
}
