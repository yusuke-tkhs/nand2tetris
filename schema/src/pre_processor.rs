/// 与えられたアセンブリファイルの構文解析に先立って前処理を行うための共通関数群

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

/// インラインコメントの削除
/// /* */ or /** */
pub(crate) fn remove_inline_comment(line: String) -> anyhow::Result<String> {
    let find_str = |pat: &str, start: usize| -> Option<usize> {
        line[start..].find(pat).map(|pos| pos + start)
    };

    let mut current_index: usize = 0;
    let mut res = String::new();
    while let Some(comment_start_index) = find_str("/*", current_index) {
        if let Some(comment_end_index) = find_str("*/", comment_start_index + 2) {
            res.push_str(&line[current_index..comment_start_index]);
            current_index = comment_end_index + 2;
        } else {
            anyhow::bail!("not closed inline comment is detected!")
        }
    }
    res.push_str(&line[current_index..]);
    Ok(res)
}

#[test]
fn test_remove_inline_comment() {
    // コメント無し
    let line_with_inline_comment = "let x = 10;".to_string();
    assert_eq!(
        remove_inline_comment(line_with_inline_comment).unwrap(),
        "let x = 10;"
    );

    // 単一のコメント
    let line_with_inline_comment = "let x = /* this is sentence */ 10;".to_string();
    assert_eq!(
        remove_inline_comment(line_with_inline_comment).unwrap(),
        "let x =  10;"
    );

    // 複数のコメント
    let line_with_inline_comment =
        "let /* this is variable name */ x = /* this is integer constant */ 10;".to_string();
    assert_eq!(
        remove_inline_comment(line_with_inline_comment).unwrap(),
        "let  x =  10;"
    );

    // APIコメント
    let line_with_inline_comment = "/** api description */".to_string();
    assert_eq!(remove_inline_comment(line_with_inline_comment).unwrap(), "");

    // 閉じていないコメント
    let line_with_open_inline_comment = "let x = /* this is sentence 10;".to_string();
    assert!(remove_inline_comment(line_with_open_inline_comment).is_err());
}

/// 空行でないことを保証する
pub(crate) fn non_empty_line(line: &String) -> bool {
    !line.is_empty()
}
