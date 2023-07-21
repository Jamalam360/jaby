# Jaby

Jaby (**Ja**va **By**tecode) is a complete Rust crate for creating Java class files.

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
                    aload(0),
                    invokespecial("java/lang/Object", "<init>", "()V"),
                    r#return(),
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
                    getstatic("java/lang/System", "out", "Ljava/io/PrintStream"),
                    iconst(2),
                    iconst(2),
                    iadd(),
                    invokevirtual("java/io/PrintStream", "println", "(I)V"),
                    r#return(),
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
