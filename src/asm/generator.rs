use std::collections::HashMap;

use crate::vm::instructions::Instruction;

use super::parser::{Expression, ExpressionKind};

#[derive(Debug)]
pub struct Generator {
    expressions: Vec<Expression>,
    pos: usize,
    pub output: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl Generator {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self {
            expressions,
            pos: 0,
            output: vec![],
            labels: HashMap::new(),
        }
    }

    fn get_labels(&mut self) {
        let mut pos = 0;
        for expr in &self.expressions {
            match &expr.kind {
                ExpressionKind::Label(label) => {
                    self.labels.insert(label.clone(), pos);
                }
                _ => {}
            }
            pos += 1;
        }
    }

    pub fn generate_expression(&mut self) -> Option<()> {
        if self.pos >= self.expressions.len() {
            return None;
        }
        if self.labels.len() == 0 {
            self.get_labels();
        }
        self.output
            .push(match self.expressions[self.pos].kind.clone() {
                ExpressionKind::PushExpression(val) => Instruction::Push(val),
                ExpressionKind::JmpExpression(val) => Instruction::Jmp(val),
                ExpressionKind::JmpLabelExpression(label) => {
                    Instruction::Jmp(*self.labels.get(&label).unwrap())
                }
                ExpressionKind::JmpZeroExpression(val) => Instruction::JmpZero(val),
                ExpressionKind::JmpEqualsExpression(val) => Instruction::JmpEquals(val),
                ExpressionKind::JmpLessExpression(val) => Instruction::JmpLess(val),
                ExpressionKind::JmpGreaterExpression(val) => Instruction::JmpGreater(val),
                ExpressionKind::AddExpression => Instruction::AddStack,
                ExpressionKind::SubExpression => Instruction::SubStack,
                ExpressionKind::MulExpression => Instruction::MulStack,
                ExpressionKind::DivExpression => Instruction::DivStack,
                ExpressionKind::HaltExpression => Instruction::Halt,
                ExpressionKind::PrintExpression => Instruction::Print,
                ExpressionKind::Label(_) => Instruction::Nop,
                ExpressionKind::JmpLabelZeroExpression(label) => {
                    Instruction::JmpZero(*self.labels.get(&label).unwrap())
                }
                ExpressionKind::JmpLabelEqualsExpression(label) => {
                    Instruction::JmpEquals(*self.labels.get(&label).unwrap())
                }
                ExpressionKind::JmpLabelGreaterExpression(label) => {
                    Instruction::JmpGreater(*self.labels.get(&label).unwrap())
                }
                ExpressionKind::JmpLabelLessExpression(label) => {
                    Instruction::JmpLess(*self.labels.get(&label).unwrap())
                }
            });
        self.pos += 1;
        Some(())
    }
}
