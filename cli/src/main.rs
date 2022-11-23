use core::panic;
use std::path::{Path, PathBuf};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 入力されるアセンブラ言語のパス
    let input_path: &Path = Path::new(args.get(1).unwrap());

    // .asm 以外はエラーにする
    if input_path.extension().unwrap() != "asm" {
        panic!("input file format must be .asm");
    }

    // アセンブラ言語から生成された機械語を生成するパス
    let output_path: PathBuf = {
        let mut path = std::path::PathBuf::from(input_path.parent().unwrap());
        path.push(format!(
            "{}.hack",
            input_path.file_stem().unwrap().to_str().unwrap()
        ));
        path
    };

    dbg!(input_path);
    dbg!(parser::parse(&std::fs::read_to_string(input_path).unwrap()).unwrap());
    dbg!(output_path);
}
