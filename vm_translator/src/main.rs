use core::panic;
use schema::vm;
use semantics::{genarate_assembler_code, AssemblerCodeBlock, Module};
use std::path::{Path, PathBuf};

mod semantics;

fn construct_assembler_code_blocks(input_path: impl AsRef<Path>) -> Vec<AssemblerCodeBlock> {
    let input = std::fs::read_to_string(input_path.as_ref()).unwrap();

    // ファイル名をモジュール名とする
    let module_name: &str = input_path.as_ref().file_stem().unwrap().to_str().unwrap();

    // 構文解析
    let vm_commands: Vec<vm::Command> = vm::parse(input).unwrap();

    // 意味解析（コード生成処理のアルゴリズムが使いやすい形にしておく）
    let module = Module::try_from_commands(module_name, vm_commands).unwrap();

    // アセンブラコード塊への変換
    module.into_code_blocks()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg_path: &Path = Path::new(args.get(1).unwrap());

    let assembler_code_blocks: Vec<AssemblerCodeBlock> = if input_arg_path.is_dir() {
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

    let assembler_code: String = genarate_assembler_code(assembler_code_blocks);

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
