use std::process::exit;
#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) kind: ExpressionKind,
}

#[derive(Debug)]
pub(crate) enum ExpressionKind {
    PushExpression(u64),
    JmpExpression(usize),
    JmpZeroExpression(usize),
    JmpEqualsExpression(usize),
    JmpGreaterExpression(usize),
    JmpLessExpression(usize),
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
                let value = self.tokens[self.pos + 1].parse::<u64>().unwrap();
                self.pos += 1;
                ExpressionKind::PushExpression(value)
            }
            "jmp" => {
                let value = self.tokens[self.pos + 1].parse::<usize>().unwrap();
                self.pos += 1;
                ExpressionKind::JmpExpression(value)
            }
            "jz" => {
                let value = self.tokens[self.pos + 1].parse::<usize>().unwrap();
                self.pos += 1;
                ExpressionKind::JmpZeroExpression(value)
            }
            "je" => {
                let value = self.tokens[self.pos + 1].parse::<usize>().unwrap();
                self.pos += 1;
                ExpressionKind::JmpEqualsExpression(value)
            }
            "jl" => {
                let value = self.tokens[self.pos + 1].parse::<usize>().unwrap();
                self.pos += 1;
                ExpressionKind::JmpLessExpression(value)
            }
            "jg" => {
                let value = self.tokens[self.pos + 1].parse::<usize>().unwrap();
                self.pos += 1;
                ExpressionKind::JmpGreaterExpression(value)
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
