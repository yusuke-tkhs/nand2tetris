/// 与えられたアセンブリファイルの構文解析に先立って前処理を行うための共通関数群

// pub(crate) fn split_by_newline(line: String) -> Vec<String> {
//     line.split('\n').into_iter().map(String::from).collect()
// }
pub(crate) fn split_by_newline(line: String) -> impl Iterator<Item = String> {
    line.split('\n')
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter()
}

/// 空白文字の削除
pub(crate) fn remove_whitespace(line: String) -> String {
    line.split_whitespace().collect()
}

/// 前後の空白の削除
pub(crate) fn trim_whitespace(line: String) -> String {
    line.trim().to_string()
}

/// コメントの削除
pub(crate) fn remove_comment(line: String) -> String {
    if let Some(pos) = line.find("//") {
        line.chars().take(pos).collect()
    } else {
        line.to_string()
    }
}

/// 空行でないことを保証する
pub(crate) fn non_empty_line(line: &String) -> bool {
    !line.is_empty()
}
