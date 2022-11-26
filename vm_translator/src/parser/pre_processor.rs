/// 与えられたアセンブリファイルの構文解析に先立って前処理を行う。
/// 空白文字やコメントの削除を行う。

fn trim_whitespace(line: String) -> String {
    line.trim().to_string()
}

fn remove_comment(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        line.chars().take(pos).collect()
    } else {
        line.to_string()
    }
}

pub(crate) fn pre_process(input: &str) -> Vec<String> {
    input
        .split('\n')
        .map(remove_comment)
        .map(trim_whitespace)
        .filter(|line| !line.is_empty())
        .collect()
}
