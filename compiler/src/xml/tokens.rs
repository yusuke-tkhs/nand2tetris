use super::sanitize;
use schema::jack::tokenizer::Token;

pub fn tokens_to_xml(tokens: &[Token]) -> String {
    std::iter::once("<tokens>".to_string())
        .chain(tokens.iter().map(|token| {
            let (key_name, value) = match token {
                Token::Keyword(keyword) => ("keyword", keyword.as_str().to_string()),
                Token::Symbol(symbol) => ("symbol", symbol.as_str().to_string()),
                Token::IntegerConstant(v) => ("integerConstant", v.to_string()),
                Token::StringConstant(s) => ("stringConstant", s.clone()),
                Token::Identifier(s) => ("identifier", s.clone()),
            };
            let sanitized_value = sanitize(value);
            format!("<{key_name}> {sanitized_value} </{key_name}>")
        }))
        .chain(std::iter::once("</tokens>".to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
