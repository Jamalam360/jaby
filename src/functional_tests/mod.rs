use crate::{
    attribute::code::CodeBuilder,
    instruction::Instruction::{
        aload, getstatic, iadd, iconst, iload, invokespecial, invokestatic, invokevirtual, ireturn,
        r#return,
    },
};

use super::*;
use std::fs;

pub fn run_bytecode<S>(bytes: Vec<u8>, expected_output: S)
where
    S: Into<String>,
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
        .unwrap();

    run_bytecode(bytes, "8\n".to_string());
}
