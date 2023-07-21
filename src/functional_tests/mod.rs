use crate::{
    attribute::code::CodeBuilder,
    instruction::{
        aload, getstatic, iadd, iconst, iload, invokespecial, invokestatic, invokevirtual, ireturn,
        r#return,
    },
};

use super::*;
use std::fs;

pub fn run_bytecode(bytes: Vec<u8>, expected_output: impl Into<String>)
{
    let expected_output = expected_output.into();
    let dir = tempfile::TempDir::new().unwrap();
    let file = dir.path().join("Test.class");
    println!("file: {:?}", file);
    fs::write(&file, bytes).unwrap();

    let output = std::process::Command::new("java")
        .arg("Test")
        .current_dir(&dir)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    if stdout != expected_output {
        let javap = std::process::Command::new("javap")
            .arg("-c")
            .arg("Test")
            .current_dir(&dir)
            .output()
            .expect("failed to execute process");

        let javap_stdout = String::from_utf8(javap.stdout).unwrap();
        let javap_stderr = String::from_utf8(javap.stderr).unwrap();
        println!("javap stdout: {}", javap_stdout);
        println!("javap stderr: {}", javap_stderr);
    }

    assert_eq!(stdout, expected_output);
    assert_eq!(stderr, "");
}

#[test]
fn add_two_numbers() {
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
        .unwrap();

    run_bytecode(bytes, "4\n".to_string());
}

#[test]
fn add_two_numbers_via_a_method() {
    let bytes = ClassFileBuilder::new()
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
        .unwrap();

    run_bytecode(bytes, "8\n".to_string());
}
