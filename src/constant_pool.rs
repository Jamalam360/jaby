use std::collections::HashMap;

use crate::Error;

#[derive(Debug)]
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

pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
    // TODO: Cache more than just string values
    cache: HashMap<String, u16>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cache: HashMap::new(),
        }
    }

    pub fn insert_string<S>(&mut self, s: S) -> u16
    where
        S: Into<String>,
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

    pub fn insert_integer(&mut self, i: i32) -> u16 {
        self.entries.push(ConstantPoolEntry::Integer(i));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_float(&mut self, f: f32) -> u16 {
        self.entries.push(ConstantPoolEntry::Float(f));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_long(&mut self, l: i64) -> u16 {
        self.entries.push(ConstantPoolEntry::Long(l));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_double(&mut self, d: f64) -> u16 {
        self.entries.push(ConstantPoolEntry::Double(d));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_class<S>(&mut self, s: S) -> u16
    where
        S: Into<String>,
    {
        let index = self.insert_string(s);
        self.entries.push(ConstantPoolEntry::Class(index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_string_reference<S>(&mut self, s: S) -> u16
    where
        S: Into<String>,
    {
        let index = self.insert_string(s);
        self.entries.push(ConstantPoolEntry::StringReference(index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_field<S>(&mut self, class: S, field: S, descriptor: S) -> u16
    where
        S: Into<String>,
    {
        let class_index = self.insert_class(class);
        let name_and_type_index = self.insert_name_and_type(field, descriptor);
        self.entries
            .push(ConstantPoolEntry::Field(class_index, name_and_type_index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_method<S>(&mut self, class: S, method: S, descriptor: S) -> u16
    where
        S: Into<String>,
    {
        let class_index = self.insert_class(class);
        let name_and_type_index = self.insert_name_and_type(method, descriptor);
        self.entries
            .push(ConstantPoolEntry::Method(class_index, name_and_type_index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_interface_method<S>(&mut self, interface: S, method: S, descriptor: S) -> u16
    where
        S: Into<String>,
    {
        let interface_index = self.insert_class(interface);
        let name_and_type_index = self.insert_name_and_type(method, descriptor);
        self.entries.push(ConstantPoolEntry::InterfaceMethod(
            interface_index,
            name_and_type_index,
        ));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_name_and_type<S>(&mut self, name: S, descriptor: S) -> u16
    where
        S: Into<String>,
    {
        let name_index = self.insert_string(name);
        let descriptor_index = self.insert_string(descriptor);
        self.entries
            .push(ConstantPoolEntry::NameAndType(name_index, descriptor_index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_method_type<S>(&mut self, descriptor: S) -> u16
    where
        S: Into<String>,
    {
        let descriptor_index = self.insert_string(descriptor);
        self.entries
            .push(ConstantPoolEntry::MethodType(descriptor_index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_dynamic(&mut self, bootstrap_method: u16, name_and_type: u16) -> u16 {
        self.entries
            .push(ConstantPoolEntry::Dynamic(bootstrap_method, name_and_type));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_invoke_dynamic(&mut self, bootstrap_method: u16, name_and_type: u16) -> u16 {
        self.entries.push(ConstantPoolEntry::InvokeDynamic(
            bootstrap_method,
            name_and_type,
        ));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_module<S>(&mut self, name: S) -> u16
    where
        S: Into<String>,
    {
        let name_index = self.insert_string(name);
        self.entries.push(ConstantPoolEntry::Module(name_index));
        self.entries.len().try_into().unwrap()
    }

    pub fn insert_package<S>(&mut self, name: S) -> u16
    where
        S: Into<String>,
    {
        let name_index = self.insert_string(name);
        self.entries.push(ConstantPoolEntry::Package(name_index));
        self.entries.len().try_into().unwrap()
    }

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
