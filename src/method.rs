use crate::{
    access_flag::AccessFlag, attribute::code::CodeBuilder, constant_pool::ConstantPool, Error,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A builder for an individual method.
pub struct MethodBuilder {
    access_flags: Vec<AccessFlag>,
    name: Option<String>,
    parameter_descriptors: Vec<String>,
    return_descriptor: Option<String>,
    code: Option<CodeBuilder>,
}

impl MethodBuilder {
    /// Creates a new [MethodBuilder].
    pub fn new() -> Self {
        Self {
            access_flags: Vec::new(),
            name: None,
            parameter_descriptors: Vec::new(),
            return_descriptor: None,
            code: None,
        }
    }

    /// Adds an access flag to the method.
    /// 'Access' flags also include modifiers such as `final` or `abstract`.
    pub fn access_flag(mut self, flag: AccessFlag) -> Self {
        self.access_flags.push(flag);
        self
    }

    /// Sets the name of the method.
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Adds a parameter to the method.
    /// This is in the internal format, `Lcom/example/ExampleClass`.
    pub fn parameter<S>(mut self, descriptor: S) -> Self
    where
        S: Into<String>,
    {
        self.parameter_descriptors.push(descriptor.into());
        self
    }

    /// Sets the return type of the method.
    /// This is in the internal format, `Lcom/example/ExampleClass`.
    pub fn r#return<S>(mut self, descriptor: S) -> Self
    where
        S: Into<String>,
    {
        self.return_descriptor = Some(descriptor.into());
        self
    }

    /// Sets the code body of the method.
    pub fn code(mut self, code: CodeBuilder) -> Self {
        self.code = Some(code);
        self
    }

    /// Emits the method to a vector of bytes.
    pub fn emit(self, constant_pool: &mut ConstantPool) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(
            &self
                .access_flags
                .into_iter()
                .fold(0, |acc, flag| acc | <AccessFlag as Into<u16>>::into(flag))
                .to_be_bytes(),
        );

        let name_index = constant_pool.insert_string(self.name.unwrap());
        bytes.extend_from_slice(&name_index.to_be_bytes());

        let descriptor_index = constant_pool.insert_string(format!(
            "({}){}",
            self.parameter_descriptors.join(""),
            self.return_descriptor.unwrap_or("V".to_string())
        ));
        bytes.extend_from_slice(&descriptor_index.to_be_bytes());

        let attributes = vec![self.code.unwrap().build(constant_pool)?];

        bytes.extend_from_slice(&(attributes.len() as u16).to_be_bytes());

        for attribute in attributes {
            bytes.extend::<Vec<u8>>(attribute.emit(constant_pool)?);
        }

        Ok(bytes)
    }
}

impl Default for MethodBuilder {
    fn default() -> Self {
        Self::new()
    }
}
