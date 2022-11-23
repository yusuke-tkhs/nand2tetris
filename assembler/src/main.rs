use core::panic;
use schema::hack;
use std::path::{Path, PathBuf};

mod machine_code;
mod parser;
mod symbol_table;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 入力されるアセンブラ言語のパス
    let input_path: &Path = Path::new(args.get(1).unwrap());

    // .asm 以外はエラーにする
    if input_path.extension().unwrap() != "asm" {
        panic!("input file format must be .asm");
    }

    let input = std::fs::read_to_string(input_path).unwrap();

    let commands: Vec<hack::Command> = parser::parse(&input).unwrap();
    dbg!(commands.clone());

    let symbol_table = symbol_table::create(&commands);

    let machine_code = machine_code::construct(&symbol_table, &commands);

    let machine_code_str = machine_code::generate(machine_code);

    // アセンブラ言語から生成された機械語を出力するパス
    let output_path: PathBuf = {
        let mut path = std::path::PathBuf::from(input_path.parent().unwrap());
        path.push(format!(
            "{}.hack",
            input_path.file_stem().unwrap().to_str().unwrap()
        ));
        path
    };
    std::fs::write(output_path, machine_code_str).unwrap();
}
