use access_flag::AccessFlag;
use constant_pool::ConstantPool;
use method::MethodBuilder;
use thiserror::Error;

pub mod access_flag;
pub mod attribute;
pub mod constant_pool;
pub mod consts;
pub mod instruction;
pub mod method;

#[cfg(test)]
mod functional_tests;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid iconst value (must be -1 to 5 inc.): {0}")]
    InvalidIconst(i32),
}

pub struct ClassFileBuilder {
    access_flags: Vec<AccessFlag>,
    constant_pool: ConstantPool,
    class_name: Option<String>,
    super_class: Option<String>,
    methods: Vec<MethodBuilder>,
}

impl ClassFileBuilder {
    pub fn new() -> Self {
        Self {
            access_flags: Vec::new(),
            constant_pool: ConstantPool::new(),
            class_name: None,
            super_class: None,
            methods: Vec::new(),
        }
    }

    pub fn access_flag(mut self, flag: AccessFlag) -> Self {
        self.access_flags.push(flag);
        self
    }

    pub fn class_name<S>(mut self, class_name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_name = Some(class_name.into());
        self
    }

    pub fn super_class<S>(mut self, super_class: S) -> Self
    where
        S: Into<String>,
    {
        self.super_class = Some(super_class.into());
        self
    }

    pub fn method(mut self, method: MethodBuilder) -> Self {
        self.methods.push(method);
        self
    }

    pub fn emit(mut self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&consts::MAGIC.to_be_bytes());
        bytes.extend_from_slice(&consts::MINOR_VERSION.to_be_bytes());
        bytes.extend_from_slice(&consts::MAJOR_VERSION.to_be_bytes());

        let method_count = self.methods.len();
        let method_bytes: Vec<u8> = self
            .methods
            .into_iter()
            .map(|m| m.emit(&mut self.constant_pool))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        let self_class = self.constant_pool.insert_class(self.class_name.unwrap());
        let super_class = self.constant_pool.insert_class(
            self.super_class
                .unwrap_or(consts::OBJECT_CLASS_NAME.to_string()),
        );

        bytes.extend::<Vec<u8>>(self.constant_pool.emit()?);

        bytes.extend_from_slice(
            &(self
                .access_flags
                .into_iter()
                .map(<AccessFlag as Into<u16>>::into)
                .sum::<u16>())
            .to_be_bytes(),
        );

        bytes.extend(self_class.to_be_bytes());
        bytes.extend(super_class.to_be_bytes());
        bytes.extend_from_slice(&[0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00]);

        bytes.extend_from_slice(&(method_count as u16).to_be_bytes());
        bytes.extend(method_bytes);

        bytes.extend_from_slice(&[0x00, 0x00]);
        Ok(bytes)
    }
}

impl Default for ClassFileBuilder {
    fn default() -> Self {
        Self::new()
    }
}
