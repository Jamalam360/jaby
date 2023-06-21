use crate::{constant_pool::ConstantPool, instruction::Instruction, Error};

use super::Attribute;

pub struct CodeBuilder {
    max_stack: u16,
    max_locals: u16,
    code: Vec<Instruction>,
    // exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Attribute>,
}

impl CodeBuilder {
    pub fn new() -> Self {
        Self {
            max_stack: 0,
            max_locals: 0,
            code: Vec::new(),
            // exception_table: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn max_stack(mut self, max_stack: u16) -> Self {
        self.max_stack = max_stack;
        self
    }

    pub fn max_locals(mut self, max_locals: u16) -> Self {
        self.max_locals = max_locals;
        self
    }

    pub fn instructions<I>(mut self, instructions: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        self.code.extend(instructions);
        self
    }

    pub fn build(self, constant_pool: &mut ConstantPool) -> Result<Attribute, Error> {
        let max_stack = if self.max_stack == 0 {
            self.calculate_max_stack()
        } else {
            self.max_stack
        };

        Ok(Attribute::Code {
            max_stack,
            max_locals: self.max_locals,
            code: self
                .code
                .into_iter()
                .map(|instruction| instruction.emit(constant_pool))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
            // exception_table: self.exception_table,
            attributes: self.attributes,
        })
    }

    fn calculate_max_stack(&self) -> u16 {
        let mut max_stack = 0i32;
        let mut stack = 0;

        for code in self.code.iter() {
            match code {
                Instruction::aload { .. } => stack += 1,
                Instruction::getstatic { .. } => stack += 1,
                Instruction::iadd => stack -= 1,
                Instruction::iconst { .. } => stack += 1,
                Instruction::iload { .. } => stack += 1,
                Instruction::invokespecial { .. } => stack -= 1,
                Instruction::invokestatic { .. } => stack -= 1,
                Instruction::invokevirtual { .. } => stack -= 1,
                Instruction::ireturn => stack -= 1,
                Instruction::r#return => stack -= 1,
            }

            if stack > max_stack {
                max_stack = stack;
            }
        }

        max_stack.try_into().unwrap_or(0)
    }
}

impl Default for CodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
