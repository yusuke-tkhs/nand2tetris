use crate::{
    parser::{easily_parse, not_digit_starts_str, p_u16, parsable_enum},
    pre_processor::{remove_comment, remove_multi_line_comment, split_by_newline, trim_whitespace},
};

use combine::{
    between, choice, optional, parser,
    parser::{
        char::space,
        repeat::{many, many1},
    },
    satisfy, Stream,
};

pub fn tokenize(code: String) -> anyhow::Result<Vec<Token>> {
    let pre_processed = split_by_newline(remove_multi_line_comment(code)?)
        .map(remove_comment)
        .map(trim_whitespace)
        .filter(|s| !s.is_empty());

    Ok(pre_processed
        .map(|s| easily_parse(tokens, s.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

parser! {
    fn tokens[Input]()(Input) -> Vec<Token>
    where [Input: Stream<Token = char>]
    {
        many1(optional(space()).with(token()).skip(optional(space())))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
}

parser! {
    fn token[Input]()(Input) -> Token
    where [Input: Stream<Token = char>]
    {
        choice((
            Keyword::parser().map(Token::Keyword),
            Symbol::parser().map(Token::Symbol),
            p_u16().map(Token::IntegerConstant),
            string_constant().map(Token::StringConstant),
            identifier().map(Token::Identifier)
        ))
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

parsable_enum! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Keyword {
        Class: "class",
        Constructor: "constructor",
        Function: "function",
        Method: "method",
        Field: "field",
        Static: "static",
        Var: "var",
        Int: "int",
        Char: "char",
        Boolean: "boolean",
        Void: "void",
        True: "true",
        False: "false",
        Null: "null",
        This: "this",
        Let: "let",
        Do: "do",
        If: "if",
        Else: "else",
        While: "while",
        Return: "return",
    }
}

parsable_enum! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Symbol {
        WaveBracketStart: "{",
        WaveBracketEnd: "}",
        RoundBracketStart: "(",
        RoundBracketEnd: ")",
        SquareBracketStart: "[",
        SquareBracketEnd: "]",
        Dot: ".",
        Comma: ",",
        SemiColon: ";",
        Plus: "+",
        Minus: "-",
        Asterisk: "*",
        Slash: "/",
        And: "&",
        Pipe: "|",
        AngleBracketStart: "<",
        AngleBracketEnd: ">",
        Equal: "=",
        Tilde: "~",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::easy_parser_assert;

    #[test]
    fn test_tokenize() {
        let res = easily_parse(
            tokens,
            "\tlet length = Keyboard.readInt(\"HOW MANY NUMBERS? \");\r",
        )
        .unwrap();
        assert_eq!(
            res,
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("length".to_string()),
                Token::Symbol(Symbol::Equal,),
                Token::Identifier("Keyboard".to_string()),
                Token::Symbol(Symbol::Dot,),
                Token::Identifier("readInt".to_string()),
                Token::Symbol(Symbol::RoundBracketStart),
                Token::StringConstant("HOW MANY NUMBERS? ".to_string()),
                Token::Symbol(Symbol::RoundBracketEnd),
                Token::Symbol(Symbol::SemiColon),
            ]
        )
    }

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
        easy_parser_assert(Keyword::parser, "class", Keyword::Class);
        easy_parser_assert(Keyword::parser, "constructor", Keyword::Constructor);
        easy_parser_assert(Keyword::parser, "function", Keyword::Function);
        easy_parser_assert(Keyword::parser, "method", Keyword::Method);
        easy_parser_assert(Keyword::parser, "field", Keyword::Field);
        easy_parser_assert(Keyword::parser, "static", Keyword::Static);
        easy_parser_assert(Keyword::parser, "var", Keyword::Var);
        easy_parser_assert(Keyword::parser, "int", Keyword::Int);
        easy_parser_assert(Keyword::parser, "char", Keyword::Char);
        easy_parser_assert(Keyword::parser, "boolean", Keyword::Boolean);
        easy_parser_assert(Keyword::parser, "void", Keyword::Void);
        easy_parser_assert(Keyword::parser, "true", Keyword::True);
        easy_parser_assert(Keyword::parser, "false", Keyword::False);
        easy_parser_assert(Keyword::parser, "null", Keyword::Null);
        easy_parser_assert(Keyword::parser, "this", Keyword::This);
        easy_parser_assert(Keyword::parser, "let", Keyword::Let);
        easy_parser_assert(Keyword::parser, "do", Keyword::Do);
        easy_parser_assert(Keyword::parser, "if", Keyword::If);
        easy_parser_assert(Keyword::parser, "else", Keyword::Else);
        easy_parser_assert(Keyword::parser, "while", Keyword::While);
        easy_parser_assert(Keyword::parser, "return", Keyword::Return);
    }

    #[test]
    fn parse_symbol() {
        easy_parser_assert(Symbol::parser, "{", Symbol::WaveBracketStart);
        easy_parser_assert(Symbol::parser, "}", Symbol::WaveBracketEnd);
        easy_parser_assert(Symbol::parser, "(", Symbol::RoundBracketStart);
        easy_parser_assert(Symbol::parser, ")", Symbol::RoundBracketEnd);
        easy_parser_assert(Symbol::parser, "[", Symbol::SquareBracketStart);
        easy_parser_assert(Symbol::parser, "]", Symbol::SquareBracketEnd);
        easy_parser_assert(Symbol::parser, ".", Symbol::Dot);
        easy_parser_assert(Symbol::parser, ",", Symbol::Comma);
        easy_parser_assert(Symbol::parser, ";", Symbol::SemiColon);
        easy_parser_assert(Symbol::parser, "+", Symbol::Plus);
        easy_parser_assert(Symbol::parser, "-", Symbol::Minus);
        easy_parser_assert(Symbol::parser, "*", Symbol::Asterisk);
        easy_parser_assert(Symbol::parser, "/", Symbol::Slash);
        easy_parser_assert(Symbol::parser, "&", Symbol::And);
        easy_parser_assert(Symbol::parser, "|", Symbol::Pipe);
        easy_parser_assert(Symbol::parser, "<", Symbol::AngleBracketStart);
        easy_parser_assert(Symbol::parser, ">", Symbol::AngleBracketEnd);
        easy_parser_assert(Symbol::parser, "=", Symbol::Equal);
        easy_parser_assert(Symbol::parser, "~", Symbol::Tilde);
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
