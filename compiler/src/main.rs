mod xml;

use schema::jack;
use std::path::{Path, PathBuf};

fn construct_tokens(input_path: impl AsRef<Path>) -> anyhow::Result<Vec<jack::Token>> {
    let input = std::fs::read_to_string(input_path.as_ref()).unwrap();
    jack::tokenize(input)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg_path: &Path = Path::new(args.get(1).unwrap());

    let (output_path, tokens) = if input_arg_path.is_dir() {
        let input_files: Vec<PathBuf> = std::fs::read_dir(input_arg_path)
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap().path())
            .filter(|p| p.is_file())
            .filter(|p| p.extension().unwrap() == std::ffi::OsStr::new("jack"))
            .collect();
        // .jack ファイルが見つからなければエラーにする
        if input_files.is_empty() {
            panic!(".jack files could not be found in the input path");
        }

        let tokens = input_files
            .into_iter()
            .flat_map(|path| construct_tokens(path).unwrap())
            .collect();

        // jack言語から生成されたxml言語を出力するパス
        let output_path: PathBuf = input_arg_path.join(format!(
            "{}.xml",
            input_arg_path.file_stem().unwrap().to_str().unwrap()
        ));

        (output_path, tokens)
    } else if input_arg_path.is_file() {
        if input_arg_path.extension().unwrap() != std::ffi::OsStr::new("jack") {
            panic!("input file has to be .jack file");
        }
        let tokens = construct_tokens(input_arg_path).unwrap();

        // jack言語から生成されたxmlを出力するパス
        let output_path: PathBuf = {
            let mut path = std::path::PathBuf::from(input_arg_path.parent().unwrap());
            path.push(format!(
                "{}.xml",
                input_arg_path.file_stem().unwrap().to_str().unwrap()
            ));
            path
        };

        (output_path, tokens)
    } else {
        panic!("First argument has to be file path or directory path.")
    };

    std::fs::write(output_path, xml::output_tokens_xml(tokens)).unwrap();
}
