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
/// Most errors in Jaby are exposed through this enum.
pub enum Error {
    #[error("Invalid iconst value (must be -1 to 5 inc.): {0}")]
    InvalidIconst(i32),
}

/// A builder for an individual class file.
/// The output bytes of [ClassFileBuilder] corresponds to the contents of one `.class` file.
pub struct ClassFileBuilder {
    access_flags: Vec<AccessFlag>,
    constant_pool: ConstantPool,
    class_name: Option<String>,
    super_class: Option<String>,
    methods: Vec<MethodBuilder>,
}

impl ClassFileBuilder {
    /// Creates a new [ClassFileBuilder].
    pub fn new() -> Self {
        Self {
            access_flags: Vec::new(),
            constant_pool: ConstantPool::new(),
            class_name: None,
            super_class: None,
            methods: Vec::new(),
        }
    }

    /// Adds an access flag to the class.
    /// 'Access' flags also include modifiers such as `final` or `abstract`.
    pub fn access_flag(mut self, flag: AccessFlag) -> Self {
        self.access_flags.push(flag);
        self
    }

    /// Sets the name of the class.
    pub fn class_name(mut self, class_name: impl Into<String>) -> Self
    {
        self.class_name = Some(class_name.into());
        self
    }

    /// Sets the name of the super class.
    /// This is in the internal format, `Lcom/example/ExampleClass`.
    pub fn super_class(mut self, super_class: impl Into<String>) -> Self
    {
        self.super_class = Some(super_class.into());
        self
    }

    /// Adds a method to the class.
    pub fn method(mut self, method: MethodBuilder) -> Self {
        self.methods.push(method);
        self
    }

    /// Emits the class file to a vector of bytes.
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
