use core::panic;
use schema::vm;
use std::path::{Path, PathBuf};

mod assembler_code;
mod file_context;
mod semantics;

fn construct_assembler_code_blocks(
    input_path: impl AsRef<Path>,
) -> Vec<assembler_code::AssemblerCodeBlock> {
    let input = std::fs::read_to_string(input_path.as_ref()).unwrap();

    // 構文解析
    let vm_commands: Vec<vm::Command> = vm::parse(input).unwrap();

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
        .map(semantics::Command::try_from_command)
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();

    // アセンブラコード塊への変換
    assembler_code::construct_code_block(semantic_commands, &mut file_context).unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg_path: &Path = Path::new(args.get(1).unwrap());

    let assembler_code_blocks: Vec<assembler_code::AssemblerCodeBlock> = if input_arg_path.is_dir()
    {
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
            .flat_map(construct_assembler_code_blocks)
            .collect()
    } else if input_arg_path.is_file() {
        if input_arg_path.extension().unwrap() != std::ffi::OsStr::new("vm") {
            panic!("input file has to be .vm file");
        }
        construct_assembler_code_blocks(input_arg_path)
    } else {
        panic!("First argument has to be file path or directory path.")
    };

    let assembler_code = assembler_code::genarate_code_str(assembler_code_blocks);

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
