use core::panic;
use schema::{hack, vm};
use std::path::{Path, PathBuf};

mod assembler_code;
mod file_context;
mod parser;
mod semantics;

fn construct_assembler_command(input_path: impl AsRef<Path>) -> Vec<hack::Command> {
    let input = std::fs::read_to_string(input_path.as_ref()).unwrap();

    // 構文解析
    let vm_commands: Vec<vm::Command> = parser::parse(input).unwrap();

    // 意味解析（コード生成処理のアルゴリズムが使いやすい形にしておく）
    let mut file_context = file_context::FileContext::new(
        input_path
            .as_ref()
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap(),
    );
    let semantic_commands: Vec<semantics::Command> = vm_commands
        .into_iter()
        .map(|command| semantics::Command::try_from_command(command, &mut file_context))
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

    // アセンブラコードのスキーマへの変換
    assembler_code::construct(semantic_commands).unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg_path: &Path = Path::new(args.get(1).unwrap());

    let assembler_commands: Vec<hack::Command> = if input_arg_path.is_dir() {
        let input_files: Vec<PathBuf> = std::fs::read_dir(input_arg_path)
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap().path())
            .filter(|p| p.is_file())
            .filter(|p| p.extension().unwrap() == std::ffi::OsStr::new("vm"))
            .collect();
        // .vm ファイルが見つからなければエラーにする
        if input_files.is_empty() {
            panic!(".vm files could not be found in the input path");
        }

        input_files
            .into_iter()
            .flat_map(construct_assembler_command)
            .collect()
    } else if input_arg_path.is_file() {
        if input_arg_path.extension().unwrap() != std::ffi::OsStr::new("vm") {
            panic!("input file has to be .vm file");
        }

        construct_assembler_command(input_arg_path)
    } else {
        panic!("First argument has to be file path or directory path.")
    };

    let assembler_code = assembler_code::generate(assembler_commands);

    // vm言語から生成されたアセンブリ言語を出力するパス
    let output_path: PathBuf = {
        let mut path = std::path::PathBuf::from(input_arg_path.parent().unwrap());
        path.push(format!(
            "{}.asm",
            input_arg_path.file_stem().unwrap().to_str().unwrap()
        ));
        path
    };

    std::fs::write(output_path, assembler_code).unwrap();
}
