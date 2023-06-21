use criterion::{criterion_group, criterion_main, Criterion};
use rava::{
    access_flag::AccessFlag,
    attribute::code::CodeBuilder,
    instruction::Instruction::{
        aload, getstatic, iadd, iconst, iload, invokespecial, invokestatic, invokevirtual, ireturn,
        r#return,
    },
    method::MethodBuilder,
    ClassFileBuilder,
};

fn run() -> Vec<u8> {
    ClassFileBuilder::new()
        .access_flag(AccessFlag::Public)
        .class_name("Test".to_string())
        .method(
            MethodBuilder::new()
                .access_flag(AccessFlag::Public)
                .name("<init>".to_string())
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
                .name("add".to_string())
                .parameter("I".to_string())
                .parameter("I".to_string())
                .r#return("I".to_string())
                .code(CodeBuilder::new().max_locals(2).instructions([
                    iload { index: 0 },
                    iload { index: 1 },
                    iadd,
                    ireturn,
                ])),
        )
        .method(
            MethodBuilder::new()
                .access_flag(AccessFlag::Public)
                .access_flag(AccessFlag::Static)
                .name("main".to_string())
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
                    invokestatic {
                        class: "Test".to_string(),
                        name: "add".to_string(),
                        descriptor: "(II)I".to_string(),
                    },
                    iconst { value: 2 },
                    iconst { value: 2 },
                    invokestatic {
                        class: "Test".to_string(),
                        name: "add".to_string(),
                        descriptor: "(II)I".to_string(),
                    },
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
        .unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "generate bytecode for adding two numbers with a function",
        |b| b.iter(|| run()),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
