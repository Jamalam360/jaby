# Rava

_Rust + Java = Rava_

Rava is a complete Rust crate for creating Java class files, or _bytecode_.

## Features

- Support for Java 17
- Builder API
- Auto-computed max stack size
- Fully tested, including running the generated code in the JVM

## Example

```rust
let bytes = ClassFileBuilder::new()
        .access_flag(AccessFlag::Public)
        .class_name("Test")
        .method(
            MethodBuilder::new()
                .access_flag(AccessFlag::Public)
                .name("<init>")
                .code(CodeBuilder::new().max_locals(1).instructions([
                    aload { index: 0 },
                    invokespecial {
                        class: "java/lang/Object".to_string(),
                        name: "<init>".to_string(),
                        descriptor: "()V".to_string(),
                    },
                    r#return,
                ])),
        )
        .method(
            MethodBuilder::new()
                .access_flag(AccessFlag::Public)
                .access_flag(AccessFlag::Static)
                .name("main")
                .parameter("[Ljava/lang/String;".to_string())
                .r#return("V".to_string())
                .code(CodeBuilder::new().max_locals(1).instructions([
                    getstatic {
                        class: "java/lang/System".to_string(),
                        name: "out".to_string(),
                        descriptor: "Ljava/io/PrintStream;".to_string(),
                    },
                    iconst { value: 2 },
                    iconst { value: 2 },
                    iadd,
                    invokevirtual {
                        class: "java/io/PrintStream".to_string(),
                        name: "println".to_string(),
                        descriptor: "(I)V".to_string(),
                    },
                    r#return,
                ])),
        )
        .emit()
```

```
java Test.class
```

```
4
```

You can find more examples in the `src/functional_tests` directory.

## Recommendations

- Import each `Instruction` individually (i.e. `use rava::instruction::{iload, iadd, ireturn}`) to make your code more concise.
