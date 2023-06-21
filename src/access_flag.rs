pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

impl From<AccessFlag> for u16 {
    fn from(val: AccessFlag) -> u16 {
        match val {
            AccessFlag::Public => 0x0001,
            AccessFlag::Private => 0x0002,
            AccessFlag::Protected => 0x0004,
            AccessFlag::Static => 0x0008,
            AccessFlag::Final => 0x0010,
            AccessFlag::Synchronized => 0x0020,
            AccessFlag::Bridge => 0x0040,
            AccessFlag::Varargs => 0x0080,
            AccessFlag::Native => 0x0100,
            AccessFlag::Abstract => 0x0400,
            AccessFlag::Strict => 0x0800,
            AccessFlag::Synthetic => 0x1000,
        }
    }
}
