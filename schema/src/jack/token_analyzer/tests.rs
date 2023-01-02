use crate::jack::tokenizer::Token;
use combine::EasyParser;

pub(crate) fn easy_parser_assert_token<'a, O, P>(mut parser: P, input: &'a [Token], expected: O)
where
    P: EasyParser<&'a [Token], Output = O>,
    O: PartialEq + std::fmt::Debug + Clone,
{
    match parser.easy_parse(input) {
        Ok((output, _)) => assert_eq!(output, expected),
        Err(e) => {
            // let position_translated_error = e.map_position(|p| p.translate_position(input));
            // panic!("{:?}", position_translated_error)
            panic!("{:?}", e)
        }
    }
}

macro_rules! tokens {
    ($(
        $(keyword: $keyword: ident)?
        $(symbol: $symbol: ident)?
        $(ident: $ident: literal)?
        $(str_const: $str_const: literal)?
        $(int_const: $int_const: literal)?
    ,)+) => {
        vec![
            $(
                $(Token::Keyword(Keyword::$keyword))?
                $(Token::Symbol(Symbol::$symbol))?
                $(Token::Identifier($ident.to_string()))?
                $(Token::StringConstant($str_const.to_string()))?
                $(Token::IntegerConstant($int_const))?
            ),+
        ]
    };
}
pub(crate) use tokens;
