use std::process::exit;
#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) kind: ExpressionKind,
}

#[derive(Debug, Clone)]
pub(crate) enum ExpressionKind {
    PushExpression(u64),
    JmpExpression(usize),
    JmpLabelExpression(String),
    JmpZeroExpression(usize),
    JmpLabelZeroExpression(String),
    JmpEqualsExpression(usize),
    JmpLabelEqualsExpression(String),
    JmpGreaterExpression(usize),
    JmpLabelGreaterExpression(String),
    JmpLessExpression(usize),
    JmpLabelLessExpression(String),
    AddExpression,
    SubExpression,
    MulExpression,
    DivExpression,
    HaltExpression,
    PrintExpression,
    Label(String),
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
                let value = self.tokens[self.pos + 1].parse::<usize>();
                if value.is_err() {
                    let value = self.tokens[self.pos + 1].parse::<String>().unwrap();
                    self.pos += 1;
                    ExpressionKind::JmpLabelExpression(value)
                } else {
                    self.pos += 1;
                    ExpressionKind::JmpExpression(value.unwrap())
                }
            }
            "jz" => {
                let value = self.tokens[self.pos + 1].parse::<usize>();
                if value.is_err() {
                    let value = self.tokens[self.pos + 1].parse::<String>().unwrap();
                    self.pos += 1;
                    ExpressionKind::JmpLabelZeroExpression(value)
                } else {
                    self.pos += 1;
                    ExpressionKind::JmpZeroExpression(value.unwrap())
                }
            }
            "je" => {
                let value = self.tokens[self.pos + 1].parse::<usize>();
                if value.is_err() {
                    let value = self.tokens[self.pos + 1].parse::<String>().unwrap();
                    self.pos += 1;
                    ExpressionKind::JmpLabelEqualsExpression(value)
                } else {
                    self.pos += 1;
                    ExpressionKind::JmpEqualsExpression(value.unwrap())
                }
            }
            "jl" => {
                let value = self.tokens[self.pos + 1].parse::<usize>();
                if value.is_err() {
                    let value = self.tokens[self.pos + 1].parse::<String>().unwrap();
                    self.pos += 1;
                    ExpressionKind::JmpLabelLessExpression(value)
                } else {
                    self.pos += 1;
                    ExpressionKind::JmpLessExpression(value.unwrap())
                }
            }
            "jg" => {
                let value = self.tokens[self.pos + 1].parse::<usize>();
                if value.is_err() {
                    let value = self.tokens[self.pos + 1].parse::<String>().unwrap();
                    self.pos += 1;
                    ExpressionKind::JmpLabelGreaterExpression(value)
                } else {
                    self.pos += 1;
                    ExpressionKind::JmpGreaterExpression(value.unwrap())
                }
            }
            "halt" => ExpressionKind::HaltExpression,
            "add" => ExpressionKind::AddExpression,
            "sub" => ExpressionKind::SubExpression,
            "mul" => ExpressionKind::MulExpression,
            "div" => ExpressionKind::DivExpression,
            "print" => ExpressionKind::PrintExpression,
            t => {
                if t.ends_with(":") {
                    let mut s = t.to_string();
                    s.pop();
                    let e = Some(Expression {
                        kind: ExpressionKind::Label(s),
                    });
                    self.pos += 1;
                    return e;
                } else {
                    eprintln!("unexpected token: {t}");
                    exit(1);
                }
            }
        };
        self.pos += 1;
        Some(Expression { kind })
    }
}
