use std::collections::HashMap;

use crate::Error;

#[derive(Debug, Clone, PartialEq)]
/// Represents an entry in the constant pool of a class file.
/// See https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4
pub enum ConstantPoolEntry {
    String(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    StringReference(u16),
    Field(u16, u16),
    Method(u16, u16),
    InterfaceMethod(u16, u16),
    NameAndType(u16, u16),
    MethodType(u16),
    Dynamic(u16, u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
}

impl From<ConstantPoolEntry> for Vec<u8> {
    fn from(val: ConstantPoolEntry) -> Vec<u8> {
        let mut bytes = Vec::new();

        match val {
            ConstantPoolEntry::String(s) => {
                bytes.extend_from_slice(&[0x01]);
                let len: u16 = s.len().try_into().unwrap();
                bytes.extend(len.to_be_bytes());
                bytes.extend_from_slice(s.as_bytes());
            }
            ConstantPoolEntry::Integer(i) => {
                bytes.extend_from_slice(&[0x03]);
                bytes.extend_from_slice(&i.to_be_bytes());
            }
            ConstantPoolEntry::Float(f) => {
                bytes.extend_from_slice(&[0x04]);
                bytes.extend_from_slice(&f.to_be_bytes());
            }
            ConstantPoolEntry::Long(l) => {
                bytes.extend_from_slice(&[0x05]);
                bytes.extend_from_slice(&l.to_be_bytes());
            }
            ConstantPoolEntry::Double(d) => {
                bytes.extend_from_slice(&[0x06]);
                bytes.extend_from_slice(&d.to_be_bytes());
            }
            ConstantPoolEntry::Class(i) => {
                bytes.extend_from_slice(&[0x07]);
                bytes.extend_from_slice(&i.to_be_bytes());
            }
            ConstantPoolEntry::StringReference(i) => {
                bytes.extend_from_slice(&[0x08]);
                bytes.extend_from_slice(&i.to_be_bytes());
            }
            ConstantPoolEntry::Field(class, name_and_type) => {
                bytes.extend_from_slice(&[0x09]);
                bytes.extend_from_slice(&class.to_be_bytes());
                bytes.extend_from_slice(&name_and_type.to_be_bytes());
            }
            ConstantPoolEntry::Method(class, name_and_type) => {
                bytes.extend_from_slice(&[0x0a]);
                bytes.extend_from_slice(&class.to_be_bytes());
                bytes.extend_from_slice(&name_and_type.to_be_bytes());
            }
            ConstantPoolEntry::InterfaceMethod(interface, name_and_type) => {
                bytes.extend_from_slice(&[0x0b]);
                bytes.extend_from_slice(&interface.to_be_bytes());
                bytes.extend_from_slice(&name_and_type.to_be_bytes());
            }
            ConstantPoolEntry::NameAndType(name, descriptor) => {
                bytes.extend_from_slice(&[0x0c]);
                bytes.extend_from_slice(&name.to_be_bytes());
                bytes.extend_from_slice(&descriptor.to_be_bytes());
            }
            ConstantPoolEntry::MethodType(descriptor) => {
                bytes.extend_from_slice(&[0x10]);
                bytes.extend_from_slice(&descriptor.to_be_bytes());
            }
            ConstantPoolEntry::Dynamic(bootstrap_method, name_and_type) => {
                bytes.extend_from_slice(&[0x12]);
                bytes.extend_from_slice(&bootstrap_method.to_be_bytes());
                bytes.extend_from_slice(&name_and_type.to_be_bytes());
            }
            ConstantPoolEntry::InvokeDynamic(bootstrap_method, name_and_type) => {
                bytes.extend_from_slice(&[0x13]);
                bytes.extend_from_slice(&bootstrap_method.to_be_bytes());
                bytes.extend_from_slice(&name_and_type.to_be_bytes());
            }
            ConstantPoolEntry::Module(name) => {
                bytes.extend_from_slice(&[0x16]);
                bytes.extend_from_slice(&name.to_be_bytes());
            }
            ConstantPoolEntry::Package(name) => {
                bytes.extend_from_slice(&[0x17]);
                bytes.extend_from_slice(&name.to_be_bytes());
            }
        }

        bytes
    }
}

#[derive(Debug, Clone)]
/// Represents the constant pool of a class file.
/// See https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
    // TODO: Cache more than just string values
    cache: HashMap<String, u16>,
}

impl ConstantPool {
    /// Creates a new empty constant pool.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cache: HashMap::new(),
        }
    }

    /// Inserts a new string into the constant pool.
    /// This is cached, so if the string already exists in the constant pool, the index of the existing string is returned.
    pub fn insert_string(&mut self, s: impl Into<String>) -> u16
    {
        let s = s.into();
        if let Some(index) = self.cache.get(&s) {
            return *index;
        }

        self.entries.push(ConstantPoolEntry::String(s.clone()));
        let index = self.entries.len().try_into().unwrap();
        self.cache.insert(s, index);
        index
    }

    /// Inserts a new integer into the constant pool.
    pub fn insert_integer(&mut self, i: i32) -> u16 {
        self.entries.push(ConstantPoolEntry::Integer(i));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new float into the constant pool.
    pub fn insert_float(&mut self, f: f32) -> u16 {
        self.entries.push(ConstantPoolEntry::Float(f));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new long into the constant pool.
    pub fn insert_long(&mut self, l: i64) -> u16 {
        self.entries.push(ConstantPoolEntry::Long(l));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new double into the constant pool.
    pub fn insert_double(&mut self, d: f64) -> u16 {
        self.entries.push(ConstantPoolEntry::Double(d));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new class into the constant pool.
    pub fn insert_class(&mut self, s: impl Into<String>) -> u16
    {
        let index = self.insert_string(s);
        self.entries.push(ConstantPoolEntry::Class(index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a string reference into the constant pool.
    pub fn insert_string_reference(&mut self, s:  impl Into<String>) -> u16
    {
        let index = self.insert_string(s);
        self.entries.push(ConstantPoolEntry::StringReference(index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new field reference into the constant pool.
    pub fn insert_field(&mut self, class: impl Into<String>, field: impl Into<String>, descriptor: impl Into<String>) -> u16
    {
        let class_index = self.insert_class(class);
        let name_and_type_index = self.insert_name_and_type(field, descriptor);
        self.entries
            .push(ConstantPoolEntry::Field(class_index, name_and_type_index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new method reference into the constant pool.
    pub fn insert_method(&mut self, class: impl Into<String>, method: impl Into<String>, descriptor: impl Into<String>) -> u16
    {
        let class_index = self.insert_class(class);
        let name_and_type_index = self.insert_name_and_type(method, descriptor);
        self.entries
            .push(ConstantPoolEntry::Method(class_index, name_and_type_index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new interface method reference into the constant pool.
    pub fn insert_interface_method(&mut self, interface: impl Into<String>, method: impl Into<String>, descriptor: impl Into<String>) -> u16
    {
        let interface_index = self.insert_class(interface);
        let name_and_type_index = self.insert_name_and_type(method, descriptor);
        self.entries.push(ConstantPoolEntry::InterfaceMethod(
            interface_index,
            name_and_type_index,
        ));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new name and type into the constant pool.
    pub fn insert_name_and_type(&mut self, name: impl Into<String>, descriptor: impl Into<String>) -> u16
    {
        let name_index = self.insert_string(name);
        let descriptor_index = self.insert_string(descriptor);
        self.entries
            .push(ConstantPoolEntry::NameAndType(name_index, descriptor_index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new method type into the constant pool.
    pub fn insert_method_type(&mut self, descriptor: impl Into<String>) -> u16
    {
        let descriptor_index = self.insert_string(descriptor);
        self.entries
            .push(ConstantPoolEntry::MethodType(descriptor_index));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new method handle into the constant pool.
    pub fn insert_dynamic(&mut self, bootstrap_method: u16, name_and_type: u16) -> u16 {
        self.entries
            .push(ConstantPoolEntry::Dynamic(bootstrap_method, name_and_type));
        self.entries.len().try_into().unwrap()
    }

    /// Inserts a new method handle into the constant pool.
    pub fn insert_invoke_dynamic(&mut self, bootstrap_method: u16, name_and_type: u16) -> u16 {
        self.entries.push(ConstantPoolEntry::InvokeDynamic(
            bootstrap_method,
            name_and_type,
        ));
        self.entries.len().try_into().unwrap()
    }

    /// Insert a new module reference into the constant pool.
    pub fn insert_module(&mut self, name: impl Into<String>) -> u16
    {
        let name_index = self.insert_string(name);
        self.entries.push(ConstantPoolEntry::Module(name_index));
        self.entries.len().try_into().unwrap()
    }

    /// Insert a new package reference into the constant pool.
    pub fn insert_package(&mut self, name: impl Into<String>) -> u16
    {
        let name_index = self.insert_string(name);
        self.entries.push(ConstantPoolEntry::Package(name_index));
        self.entries.len().try_into().unwrap()
    }

    /// Emit the constant pool as a byte vector.
    pub fn emit(self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.entries.len() as u16 + 1).to_be_bytes());

        for entry in self.entries {
            bytes.extend::<Vec<u8>>(entry.into());
        }

        Ok(bytes)
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}
