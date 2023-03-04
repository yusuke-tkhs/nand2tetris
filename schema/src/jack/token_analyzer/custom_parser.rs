use crate::jack::tokenizer::{Keyword, Symbol, Token};
use combine::error::StreamError;
use combine::stream::StreamErrorFor;
use combine::{parser, satisfy, value, Stream};

parser! {
    pub(super) fn keyword[Input](keyword: Keyword)(Input) -> ()
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Keyword(k) if k == *keyword )).with(value(()))
    }
}

parser! {
    pub(super) fn symbol[Input](symbol: Symbol)(Input) -> ()
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Symbol(s) if s == *symbol )).with(value(()))
    }
}

parser! {
    pub(super) fn identifier[Input]()(Input) -> String
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::Identifier(_)))
            .and_then(|t|match t{
                Token::Identifier(s) => Ok(s),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse identifer!"))
            })
            .message("identifier failed")
    }
}

parser! {
    pub(super) fn string_constant[Input]()(Input) -> String
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::StringConstant(_)))
            .and_then(|t|match t{
                Token::StringConstant(s) => Ok(s),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse string constant!"))
            })
            .message("string_constant failed")
    }
}

parser! {
    pub(super) fn integer_constant[Input]()(Input) -> u16
    where [Input: Stream<Token = Token>]
    {
        satisfy(|t|matches!(t, Token::IntegerConstant(_)))
            .and_then(|t|match t{
                Token::IntegerConstant(v) => Ok(v),
                _ => Err(StreamErrorFor::<Input>::message( "failed to parse integer constant!"))
            })
            .message("integer_constant failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jack::token_analyzer::tests::{easy_parser_assert_token, tokens};

    #[test]
    fn parse_keyword() {
        easy_parser_assert_token(keyword(Keyword::Class), &tokens!(keyword: Class,), ())
    }

    #[test]
    fn parse_symbol() {
        easy_parser_assert_token(symbol(Symbol::Comma), &tokens!(symbol: Comma,), ())
    }

    #[test]
    fn parse_string_constant() {
        easy_parser_assert_token(
            string_constant(),
            &tokens!(str_const: "string_constant",),
            "string_constant".to_string(),
        )
    }

    #[test]
    fn parse_identifier() {
        easy_parser_assert_token(
            identifier(),
            &tokens!(ident: "identifier",),
            "identifier".to_string(),
        )
    }
}
