use criterion::{criterion_group, criterion_main, Criterion};
use jaby::{
    access_flag::AccessFlag,
    attribute::code::CodeBuilder,
    instruction::{
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
                    aload(0),
                    invokespecial("java/lang/Object", "<init>", "()V"),
                    r#return(),
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
                    iload(0),
                    iload(1),
                    iadd(),
                    ireturn(),
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
                    getstatic("java/lang/System", "out", "Ljava/io/PrintStream"),
                    iconst(2),
                    iconst(2),
                    invokestatic("Test", "add", "(II)I"),
                    iconst(2),
                    iconst(2),
                    invokestatic("Test", "add", "(II)I"),
                    iadd(),
                    invokevirtual("java/io/PrintStream", "println", "(I)V"),
                    r#return(),
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
