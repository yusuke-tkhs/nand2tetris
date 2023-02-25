mod codegen;
mod xml;

use schema::jack::{
    token_analyzer::parse_tokens_as_class,
    tokenizer::{tokenize, Token},
};
use std::path::{Path, PathBuf};

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
            generate_files(&path)
                .map_err(|e: anyhow::Error| {
                    format!(
                        "Compile failed!\nPath: {}, \nError: {}",
                        path.as_os_str().to_str().unwrap(),
                        e.to_string()
                    )
                })
                .unwrap();
        });
    } else if input_arg_path.is_file() {
        if input_arg_path.extension().unwrap() != std::ffi::OsStr::new("jack") {
            panic!("input file has to be .jack file");
        }
        generate_files(input_arg_path).unwrap();
    } else {
        panic!("First argument has to be file path or directory path.")
    };
}

fn generate_files(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let tokens = construct_tokens(&path)?;
    let tokens_xml = xml::tokens_to_xml(&tokens);
    std::fs::write(output_tokens_xml_path(&path).unwrap(), tokens_xml)?;

    let class = parse_tokens_as_class(&tokens)?;
    let class_xml = xml::class_to_xml(&class);
    std::fs::write(output_jack_token_xml_path(&path).unwrap(), class_xml)?;

    let vm_commands = codegen::class_to_commands(&class);
    let vm_code = codegen::commands_to_code(&vm_commands);
    std::fs::write(output_vm_path(&path).unwrap(), vm_code)?;

    Ok(())
}

fn construct_tokens(input_path: impl AsRef<Path>) -> anyhow::Result<Vec<Token>> {
    let input = std::fs::read_to_string(input_path.as_ref())?;
    tokenize(input)
}

// jack言語から生成されたファイルを出力するパス
fn output_path(path: impl AsRef<Path>, suffix: &str, extension: &str) -> Option<PathBuf> {
    let stem = path.as_ref().file_stem()?.to_str()?;
    let parent = path.as_ref().parent()?;
    Some(parent.join(format!("{stem}{suffix}.{extension}")))
}

// 字句解析結果の出力先
fn output_tokens_xml_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    output_path(path, "T_by_compiler", "xml")
}

// 構文解析結果の出力先
fn output_jack_token_xml_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    output_path(path, "_by_compiler", "xml")
}

// コンパイル結果の出力先
fn output_vm_path(path: impl AsRef<Path>) -> Option<PathBuf> {
    output_path(path, "", "vm")
}
