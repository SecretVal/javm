use std::process::exit;
#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) kind: ExpressionKind,
}

#[derive(Debug)]
pub(crate) enum ExpressionKind {
    PushExpression(u8),
    AddExpression,
    SubExpression,
    MulExpression,
    DivExpression,
    HaltExpression,
    PrintExpression,
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let kind = match self.tokens[self.pos].as_str() {
            "push" => {
                let value = self.tokens[self.pos + 1].parse::<u8>().unwrap();
                self.pos += 1;
                ExpressionKind::PushExpression(value)
            }
            "halt" => ExpressionKind::HaltExpression,
            "add" => ExpressionKind::AddExpression,
            "sub" => ExpressionKind::SubExpression,
            "mul" => ExpressionKind::MulExpression,
            "div" => ExpressionKind::DivExpression,
            "print" => ExpressionKind::PrintExpression,
            _ => {
                eprintln!("unexpected token");
                exit(1);
            }
        };
        self.pos += 1;
        Some(Expression { kind })
    }
}
