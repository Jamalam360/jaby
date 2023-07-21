use crate::{constant_pool::ConstantPool, Error};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a JVM bytecode instruction.
/// See https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-6.html#jvms-6.5
pub enum Instruction {
    aload {
        index: u8,
    },
    iconst {
        value: i32,
    },
    iload {
        index: u8,
    },
    iadd,
    invokespecial {
        class: String,
        name: String,
        descriptor: String,
    },
    invokevirtual {
        class: String,
        name: String,
        descriptor: String,
    },
    invokestatic {
        class: String,
        name: String,
        descriptor: String,
    },
    getstatic {
        class: String,
        name: String,
        descriptor: String,
    },
    ireturn,
    r#return,
}

impl Instruction {
    /// Emits the instruction as a byte array.
    pub fn emit(self, constant_pool: &mut ConstantPool) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();

        match self {
            Instruction::aload { index } => {
                bytes.extend_from_slice(&[0x19]);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::iload { index } => {
                bytes.extend_from_slice(&[0x15]);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::iconst { value } => match value {
                -1 => bytes.extend_from_slice(&[0x02]),
                0 => bytes.extend_from_slice(&[0x03]),
                1 => bytes.extend_from_slice(&[0x04]),
                2 => bytes.extend_from_slice(&[0x05]),
                3 => bytes.extend_from_slice(&[0x06]),
                4 => bytes.extend_from_slice(&[0x07]),
                5 => bytes.extend_from_slice(&[0x08]),
                _ => Err(Error::InvalidIconst(value))?,
            },
            Instruction::iadd => {
                bytes.extend_from_slice(&[0x60]);
            }
            Instruction::invokespecial {
                class,
                name,
                descriptor,
            } => {
                bytes.extend_from_slice(&[0xb7]);
                let index = constant_pool.insert_method(class, name, descriptor);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::invokestatic {
                class,
                name,
                descriptor,
            } => {
                bytes.extend_from_slice(&[0xb8]);
                let index = constant_pool.insert_method(class, name, descriptor);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::invokevirtual {
                class,
                name,
                descriptor,
            } => {
                bytes.extend_from_slice(&[0xb6]);
                let index = constant_pool.insert_method(class, name, descriptor);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::getstatic {
                class,
                name,
                descriptor,
            } => {
                bytes.extend_from_slice(&[0xb2]);
                let index = constant_pool.insert_field(class, name, descriptor);
                bytes.extend_from_slice(&index.to_be_bytes());
            }
            Instruction::r#return => {
                bytes.extend_from_slice(&[0xb1]);
            }
            Instruction::ireturn => {
                bytes.extend_from_slice(&[0xac]);
            }
        }

        Ok(bytes)
    }
}

pub fn aload(index: u8) -> Instruction {
    Instruction::aload { index }
}

pub fn iconst(value: i32) -> Instruction {
    if value < -1 || value > 5 {
        panic!("Invalid iconst value: {}", value);
    }

    Instruction::iconst { value }
}

pub fn iload(index: u8) -> Instruction {
    Instruction::iload { index }
}

pub fn iadd() -> Instruction {
    Instruction::iadd
}

pub fn invokespecial(class: impl Into<String>, name: impl Into<String>, descriptor: impl Into<String>) -> Instruction {
    Instruction::invokespecial {
        class: class.into(),
        name: name.into(),
        descriptor: descriptor.into(),
    }
}

pub fn invokevirtual(class: impl Into<String>, name: impl Into<String>, descriptor: impl Into<String>) -> Instruction {
    Instruction::invokevirtual {
        class: class.into(),
        name: name.into(),
        descriptor: descriptor.into(),
    }
}

pub fn invokestatic(class: impl Into<String>, name: impl Into<String>, descriptor: impl Into<String>) -> Instruction {
    Instruction::invokestatic {
        class: class.into(),
        name: name.into(),
        descriptor: descriptor.into(),
    }
}

pub fn getstatic(class: impl Into<String>, name: impl Into<String>, descriptor: impl Into<String>) -> Instruction {
    Instruction::getstatic {
        class: class.into(),
        name: name.into(),
        descriptor: descriptor.into(),
    }
}

pub fn ireturn() -> Instruction {
    Instruction::ireturn
}

pub fn r#return() -> Instruction {
    Instruction::r#return
}
