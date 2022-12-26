mod xml;

use schema::jack;
use std::path::{Path, PathBuf};

fn construct_tokens(input_path: impl AsRef<Path>) -> anyhow::Result<Vec<jack::Token>> {
    dbg!(&input_path.as_ref());
    let input = std::fs::read_to_string(input_path.as_ref()).unwrap();
    jack::tokenize(input)
}

// jack言語から生成されたxml言語を出力するパス
fn output_xml_path(path: impl AsRef<Path>, prefix: &str) -> Option<PathBuf> {
    let stem = path.as_ref().file_stem()?.to_str()?;
    let parent = path.as_ref().parent()?;
    Some(parent.join(format!("{stem}{prefix}.xml")))
}

fn output_tokens_xml_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    output_xml_path(path, "T_by_compiler")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg_path: &Path = Path::new(args.get(1).unwrap());

    if input_arg_path.is_dir() {
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

        input_files.into_iter().for_each(|path| {
            let tokens = construct_tokens(&path).unwrap();
            let xml = xml::tokens_to_xml(tokens);
            std::fs::write(output_tokens_xml_path(&path).unwrap(), xml).unwrap();
        });
    } else if input_arg_path.is_file() {
        if input_arg_path.extension().unwrap() != std::ffi::OsStr::new("jack") {
            panic!("input file has to be .jack file");
        }
        let tokens = construct_tokens(input_arg_path).unwrap();

        std::fs::write(
            output_tokens_xml_path(input_arg_path).unwrap(),
            xml::tokens_to_xml(tokens),
        )
        .unwrap();
    } else {
        panic!("First argument has to be file path or directory path.")
    };
}
