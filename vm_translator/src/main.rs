use core::panic;
use schema::{hack, vm};
use std::path::{Path, PathBuf};

mod assembler_code;
mod parser;
mod semantics;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 入力されるアセンブラ言語のパス
    let input_path: &Path = Path::new(args.get(1).unwrap());

    // .vm 以外はエラーにする
    if input_path.extension().unwrap() != "vm" {
        panic!("input file format must be .vm");
    }

    let input = std::fs::read_to_string(input_path).unwrap();

    // 構文解析
    let vm_commands: Vec<vm::Command> = parser::parse(input).unwrap();
    dbg!(vm_commands.clone());

    // 意味解析（コード生成処理のアルゴリズムが使いやすい形にしておく）
    let semantic_commands: Vec<semantics::Command> = vm_commands
        .into_iter()
        .map(|command| semantics::Command::try_from_command(command, "file_name".to_string()))
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

    let assembler_commands: Vec<hack::Command> =
        assembler_code::construct(semantic_commands).unwrap();

    let assembler_code = assembler_code::generate(assembler_commands);

    // vm言語から生成されたアセンブリ言語を出力するパス
    let output_path: PathBuf = {
        let mut path = std::path::PathBuf::from(input_path.parent().unwrap());
        path.push(format!(
            "{}.hack",
            input_path.file_stem().unwrap().to_str().unwrap()
        ));
        path
    };
    std::fs::write(output_path, assembler_code).unwrap();
}
