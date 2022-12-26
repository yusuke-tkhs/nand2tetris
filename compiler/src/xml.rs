use schema::jack::Token;

pub fn tokens_to_xml(tokens: Vec<Token>) -> String {
    std::iter::once("<tokens>".to_string())
        .chain(
            tokens
                .into_iter()
                .map(|token| match token {
                    Token::Keyword(keyword) => ("keyword", keyword.to_str().to_string()),
                    Token::Symbol(symbol) => ("symbol", symbol.to_str().to_string()),
                    Token::IntegerConstant(v) => ("integerConstant", v.to_string()),
                    Token::StringConstant(s) => ("stringConstant", s),
                    Token::Identifier(s) => ("identifier", s),
                })
                .map(|(key_name, value)| {
                    let sanitized_value = sanitize(value);
                    format!("<{key_name}> {sanitized_value} </{key_name}>")
                }),
        )
        .chain(std::iter::once("</tokens>".to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn sanitize(s: String) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}