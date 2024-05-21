use crate::vm::instructions::Instruction;

use super::parser::{Expression, ExpressionKind};

#[derive(Debug)]
pub struct Generator {
    expressions: Vec<Expression>,
    pos: usize,
    pub output: Vec<Instruction>,
}

impl Generator {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self {
            expressions,
            pos: 0,
            output: vec![],
        }
    }

    pub fn generate_expression(&mut self) -> Option<()> {
        if self.pos >= self.expressions.len() {
            return None;
        }
        self.output.push(match self.expressions[self.pos].kind {
            ExpressionKind::PushExpression(val) => Instruction::Push(val),
            ExpressionKind::JmpExpression(val) => Instruction::Jmp(val),
            ExpressionKind::JmpZeroExpression(val) => Instruction::JmpZero(val),
            ExpressionKind::JmpEqualsExpression(val) => Instruction::JmpEquals(val),
            ExpressionKind::AddExpression => Instruction::AddStack,
            ExpressionKind::SubExpression => Instruction::SubStack,
            ExpressionKind::MulExpression => Instruction::MulStack,
            ExpressionKind::DivExpression => Instruction::DivStack,
            ExpressionKind::HaltExpression => Instruction::Halt,
            ExpressionKind::PrintExpression => Instruction::Print,
        });
        self.pos += 1;
        Some(())
    }
}
