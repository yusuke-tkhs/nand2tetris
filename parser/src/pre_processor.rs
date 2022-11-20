/// 与えられたアセンブリファイルの構文解析に先立って前処理を行う。
/// 空白文字やコメントの削除を行う。

fn remove_whitespace(line: &str) -> String {
    line.split_whitespace().collect()
}

fn remove_comment(line: String) -> String {
    if let Some(pos) = line.find("//") {
        line.chars().take(pos).collect()
    } else {
        line
    }
}

pub(crate) fn pre_process(input: String) -> Vec<String> {
    input
        .split('\n')
        .map(remove_whitespace)
        .map(remove_comment)
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_whitespace_works() {
        assert_eq!(remove_whitespace("\ta b   c  d e "), "abcde");
    }

    #[test]
    fn remove_comment_works() {
        assert_eq!(remove_comment("//".to_string()), "");
        assert_eq!(remove_comment("@variable//".to_string()), "@variable");
    }

    #[test]
    fn preprocess_works() {
        assert_eq!(
            pre_process("@\tvariable // comments".to_string())[0],
            "@variable"
        );
    }
}
