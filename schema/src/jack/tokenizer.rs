use super::*;
use crate::parser::{easily_parse, not_digit_starts_str, p_u16};
use crate::pre_processor::{remove_comment, remove_inline_comment, split_by_newline};
use combine::parser::char::char;
use combine::parser::char::string;
use combine::parser::choice::choice;
use combine::parser::repeat::{many, many1};
use combine::parser::token::value;
use combine::{attempt, satisfy};
use combine::{between, parser, Stream};

pub fn tokenize(code: String) -> anyhow::Result<Vec<Token>> {
    let pre_processed = split_by_newline(code)
        .map(remove_comment)
        .map(remove_inline_comment)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(pre_processed
        .into_iter()
        .map(tokenize_line)
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn tokenize_line(line: String) -> anyhow::Result<Vec<Token>> {
    Ok(line
        .split_whitespace()
        .map(parse_tokens)
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn parse_tokens(s: &str) -> anyhow::Result<Vec<Token>> {
    easily_parse(tokens, s)
}

parser! {
    fn tokens[Input]()(Input) -> Vec<Token>
    where [Input: Stream<Token = char>]
    {
        many1(token())
    }
}

parser! {
    fn token[Input]()(Input) -> Token
    where [Input: Stream<Token = char>]
    {
        choice((
            keyword().map(Token::Keyword),
            symbol().map(Token::Symbol),
            p_u16().map(Token::IntegerConstant),
            string_constant().map(Token::StringConstant),
            identifier().map(Token::Identifier)
        ))
    }
}

parser! {
    fn keyword[Input]()(Input) -> Keyword
    where [Input: Stream<Token = char>]
    {
        choice([
            attempt(string("class").with(value(Keyword::Class))),
            attempt(string("constructor").with(value(Keyword::Constructor))),
            attempt(string("function").with(value(Keyword::Function))),
            attempt(string("method").with(value(Keyword::Method))),
            attempt(string("field").with(value(Keyword::Field))),
            attempt(string("static").with(value(Keyword::Static))),
            attempt(string("var").with(value(Keyword::Var))),
            attempt(string("int").with(value(Keyword::Int))),
            attempt(string("char").with(value(Keyword::Char))),
            attempt(string("boolean").with(value(Keyword::Boolean))),
            attempt(string("void").with(value(Keyword::Void))),
            attempt(string("true").with(value(Keyword::True))),
            attempt(string("false").with(value(Keyword::False))),
            attempt(string("null").with(value(Keyword::Null))),
            attempt(string("this").with(value(Keyword::This))),
            attempt(string("let").with(value(Keyword::Let))),
            attempt(string("do").with(value(Keyword::Do))),
            attempt(string("if").with(value(Keyword::If))),
            attempt(string("else").with(value(Keyword::Else))),
            attempt(string("while").with(value(Keyword::While))),
            attempt(string("return").with(value(Keyword::Return))),
        ])
    }
}

parser! {
    fn symbol[Input]()(Input) -> Symbol
    where [Input: Stream<Token = char>]
    {
        choice([
            char('{').with(value(Symbol::WaveBracketStart)),
            char('}').with(value(Symbol::WaveBracketEnd)),
            char('(').with(value(Symbol::RoundBracketStart)),
            char(')').with(value(Symbol::RoundBracketEnd)),
            char('[').with(value(Symbol::SqareBracketStart)),
            char(']').with(value(Symbol::SquareBracketEnd)),
            char('.').with(value(Symbol::Dot)),
            char(',').with(value(Symbol::Comma)),
            char(';').with(value(Symbol::SemiColon)),
            char('+').with(value(Symbol::Plus)),
            char('-').with(value(Symbol::Minus)),
            char('*').with(value(Symbol::Asterisk)),
            char('/').with(value(Symbol::Slash)),
            char('&').with(value(Symbol::And)),
            char('|').with(value(Symbol::Pipe)),
            char('<').with(value(Symbol::AngleBracketStart)),
            char('>').with(value(Symbol::AngleBracketEnd)),
            char('=').with(value(Symbol::Equal)),
            char('~').with(value(Symbol::Tilde)),
        ])
    }
}

parser! {
    fn string_constant[Input]()(Input) -> String
    where [Input: Stream<Token = char>]
    {
        between(combine::token('"'), combine::token('"'), many(satisfy(|c|c != '"')))
    }
}

const AVAILABLE_CHARS_IN_IDENTIFIER: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

parser! {
    fn identifier[Input]()(Input) -> String
    where [Input: Stream<Token = char>]
    {
        not_digit_starts_str(AVAILABLE_CHARS_IN_IDENTIFIER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::easy_parser_assert;

    #[test]
    fn parse_token() {
        easy_parser_assert(token, "class", Token::Keyword(Keyword::Class));
        easy_parser_assert(token, "{", Token::Symbol(Symbol::WaveBracketStart));
        easy_parser_assert(token, "1234", Token::IntegerConstant(1234));
        easy_parser_assert(token, "\"_abc\"", Token::StringConstant("_abc".to_string()));
        easy_parser_assert(
            token,
            "identifier_",
            Token::Identifier("identifier_".to_string()),
        );
    }

    #[test]
    fn parse_keyword() {
        easy_parser_assert(keyword, "class", Keyword::Class);
        easy_parser_assert(keyword, "constructor", Keyword::Constructor);
        easy_parser_assert(keyword, "function", Keyword::Function);
        easy_parser_assert(keyword, "method", Keyword::Method);
        easy_parser_assert(keyword, "field", Keyword::Field);
        easy_parser_assert(keyword, "static", Keyword::Static);
        easy_parser_assert(keyword, "var", Keyword::Var);
        easy_parser_assert(keyword, "int", Keyword::Int);
        easy_parser_assert(keyword, "char", Keyword::Char);
        easy_parser_assert(keyword, "boolean", Keyword::Boolean);
        easy_parser_assert(keyword, "void", Keyword::Void);
        easy_parser_assert(keyword, "true", Keyword::True);
        easy_parser_assert(keyword, "false", Keyword::False);
        easy_parser_assert(keyword, "null", Keyword::Null);
        easy_parser_assert(keyword, "this", Keyword::This);
        easy_parser_assert(keyword, "let", Keyword::Let);
        easy_parser_assert(keyword, "do", Keyword::Do);
        easy_parser_assert(keyword, "if", Keyword::If);
        easy_parser_assert(keyword, "else", Keyword::Else);
        easy_parser_assert(keyword, "while", Keyword::While);
        easy_parser_assert(keyword, "return", Keyword::Return);
    }

    #[test]
    fn parse_symbol() {
        easy_parser_assert(symbol, "{", Symbol::WaveBracketStart);
        easy_parser_assert(symbol, "}", Symbol::WaveBracketEnd);
        easy_parser_assert(symbol, "(", Symbol::RoundBracketStart);
        easy_parser_assert(symbol, ")", Symbol::RoundBracketEnd);
        easy_parser_assert(symbol, "[", Symbol::SqareBracketStart);
        easy_parser_assert(symbol, "]", Symbol::SquareBracketEnd);
        easy_parser_assert(symbol, ".", Symbol::Dot);
        easy_parser_assert(symbol, ",", Symbol::Comma);
        easy_parser_assert(symbol, ";", Symbol::SemiColon);
        easy_parser_assert(symbol, "+", Symbol::Plus);
        easy_parser_assert(symbol, "-", Symbol::Minus);
        easy_parser_assert(symbol, "*", Symbol::Asterisk);
        easy_parser_assert(symbol, "/", Symbol::Slash);
        easy_parser_assert(symbol, "&", Symbol::And);
        easy_parser_assert(symbol, "|", Symbol::Pipe);
        easy_parser_assert(symbol, "<", Symbol::AngleBracketStart);
        easy_parser_assert(symbol, ">", Symbol::AngleBracketEnd);
        easy_parser_assert(symbol, "=", Symbol::Equal);
        easy_parser_assert(symbol, "~", Symbol::Tilde);
    }

    #[test]
    fn parse_string_constant() {
        easy_parser_assert(
            string_constant,
            "\"12345 . abcde\"",
            "12345 . abcde".to_string(),
        );
        easy_parser_assert(string_constant, "\"\"", "".to_string());
    }

    #[test]
    fn parse_identifier() {
        easy_parser_assert(identifier, "_abcde", "_abcde".to_string());
    }
}
