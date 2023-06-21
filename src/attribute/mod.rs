use crate::{constant_pool::ConstantPool, Error};

pub mod code;

pub enum Attribute {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        // exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>,
    },
}

impl Attribute {
    pub fn emit(self, constant_pool: &mut ConstantPool) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();

        match self {
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                // exception_table,
                attributes,
            } => {
                let name_index = constant_pool.insert_string("Code".to_string());
                bytes.extend_from_slice(&name_index.to_be_bytes());
                let attribute_length: u32 = (12 + code.len() + attributes.len()) as u32;
                bytes.extend_from_slice(&attribute_length.to_be_bytes());
                bytes.extend_from_slice(&max_stack.to_be_bytes());
                bytes.extend_from_slice(&max_locals.to_be_bytes());

                let code_length = code.len() as u32;
                bytes.extend_from_slice(&code_length.to_be_bytes());

                bytes.extend_from_slice(&code);

                bytes.extend_from_slice(&[0x00, 0x00]);
                bytes.extend_from_slice(&[0x00, 0x00]);
                // bytes.extend_from_slice(&attributes.len().to_be_bytes());

                // for attribute in attributes {
                // bytes.extend::<Vec<u8>>(attribute.emit(constant_pool));
                // }
            }
        }

        Ok(bytes)
    }
}
