use crate::jack::jack_parser::common::{
    between_round_bracket, between_square_bracket, sep_by_comma,
};
use crate::jack::jack_parser::*;
use crate::{keyword_parsable_enum, symbol_parsable_enum};
use combine::{many, optional};

/// 式は、１つ以上の「項」から構成される。
/// ２つ以上の「項」を含む場合、それらは二項演算子で接続される。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Expression {
    pub term: Term,
    pub subsequent_terms: Vec<(BinaryOperator, Term)>,
}

parser! {
    pub(crate) fn expression[Input]()(Input) -> Expression
    where [Input: Stream<Token = Token>]
    {
        term()
        .and(many(BinaryOperator::parser().and(term())))
        .map(|(term, subsequent_terms)|Expression{
            term,
            subsequent_terms,
        })
    }
}

/// 式を構成する「項」。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Term {
    IntegerConstant(u16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    Identifier(String),
    ArrayIdentifier(String, Box<Expression>),
    SubroutineCall(SubroutineCall),
    RoundBraketedExpr(Box<Expression>),
    UnaryOperatedExpr(UnaryOperator, Box<Term>),
}

parser! {
    pub(crate) fn term[Input]()(Input) -> Term
    where [Input: Stream<Token = Token>]
    {
        choice((
            // 上の３つはidentifierから始まるので、attempt をつけてかつこの順番である必要がある
            attempt(array_identifier().map(|(ident, expr)|Term::ArrayIdentifier(ident, Box::new(expr)))),
            attempt(subroutine_call().map(Term::SubroutineCall)),
            identifier().map(Term::Identifier),
            integer_constant().map(Term::IntegerConstant),
            string_constant().map(Term::StringConstant),
            KeywordConstant::parser().map(Term::KeywordConstant),
            round_bracketed_expr().map(|expr|Term::RoundBraketedExpr(Box::new(expr))),
            unary_operated_expr().map(|(op, term)|Term::UnaryOperatedExpr(op, Box::new(term))),
        ))
    }
}

parser! {
    pub(crate) fn array_identifier[Input]()(Input) -> (String, Expression)
    where [Input: Stream<Token = Token>]
    {
        identifier()
            .and(between_square_bracket(expression()))
    }
}

parser! {
    pub(crate) fn round_bracketed_expr[Input]()(Input) -> Expression
    where [Input: Stream<Token = Token>]
    {
        between_round_bracket(expression())
    }
}

parser! {
    pub(crate) fn unary_operated_expr[Input]()(Input) -> (UnaryOperator, Term)
    where [Input: Stream<Token = Token>]
    {
        UnaryOperator::parser().and(term())
    }
}

symbol_parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum BinaryOperator {
        Plus: Plus,
        Minus: Minus,
        Multiplication: Asterisk,
        Division: Slash,
        And: And,
        Or: Pipe,
        SmallerThan: AngleBracketStart,
        LargerThan: AngleBracketEnd,
    }
}

symbol_parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum UnaryOperator {
        Not: Tilde,
        Minus: Minus,
    }
}

keyword_parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum KeywordConstant {
        True,
        False,
        Null,
        This
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SubroutineCall {
    pub subroutine_holder_name: Option<String>, // (className | varName).subroutine(args) のときSome
    pub subroutine_name: String,
    pub subroutine_args: Vec<Expression>,
}

parser! {
    pub(crate) fn subroutine_call[Input]()(Input) -> SubroutineCall
    where [Input: Stream<Token = Token>]
    {
        optional(attempt(identifier().skip(symbol(Symbol::Dot)))) // (className | varName)
        .and(identifier()) // subroutineName
        .and(between_round_bracket(sep_by_comma(expression())))
        .map(|((subroutine_holder_name,subroutine_name) ,subroutine_args)|SubroutineCall{
            subroutine_holder_name,
            subroutine_name,
            subroutine_args,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jack::jack_parser::tests::easy_parser_assert_token;
    use crate::tokens;

    #[test]
    fn parse_binary_operator() {
        // 1 + 2
        easy_parser_assert_token(
            expression(),
            &tokens!(
                int_const: 1,
                symbol: Plus,
                int_const: 2,
            ),
            Expression {
                term: Term::IntegerConstant(1),
                subsequent_terms: vec![(BinaryOperator::Plus, Term::IntegerConstant(2))],
            },
        );
    }

    #[test]
    fn parse_unary_operator() {
        // -1
        easy_parser_assert_token(
            expression(),
            &tokens!(symbol: Tilde, keyword: False,),
            Expression {
                term: Term::UnaryOperatedExpr(
                    UnaryOperator::Not,
                    Box::new(Term::KeywordConstant(KeywordConstant::False)),
                ),
                subsequent_terms: vec![],
            },
        );
    }

    #[test]
    fn parse_nested_expr() {
        // 2 * (1 + 3)
        easy_parser_assert_token(
            expression(),
            &tokens!(
                int_const: 2,
                symbol: Asterisk,
                symbol: RoundBracketStart,
                int_const: 1,
                symbol: Plus,
                int_const: 3,
                symbol: RoundBracketEnd,
            ),
            Expression {
                term: Term::IntegerConstant(2),
                subsequent_terms: vec![(
                    BinaryOperator::Multiplication,
                    Term::RoundBraketedExpr(Box::new(Expression {
                        term: Term::IntegerConstant(1),
                        subsequent_terms: vec![(BinaryOperator::Plus, Term::IntegerConstant(3))],
                    })),
                )],
            },
        );
    }

    #[test]
    fn parse_subroutine_call() {
        // get(c,d)
        easy_parser_assert_token(
            expression(),
            &tokens!(
                ident: "get",
                symbol: RoundBracketStart,
                ident: "c",
                symbol: Comma,
                ident: "d",
                symbol: RoundBracketEnd,
            ),
            Expression {
                term: Term::SubroutineCall(SubroutineCall {
                    subroutine_holder_name: None,
                    subroutine_name: "get".to_string(),
                    subroutine_args: vec![
                        Expression {
                            term: Term::Identifier("c".to_string()),
                            subsequent_terms: vec![],
                        },
                        Expression {
                            term: Term::Identifier("d".to_string()),
                            subsequent_terms: vec![],
                        },
                    ],
                }),
                subsequent_terms: vec![],
            },
        );
        // abc.get(c,d)
        easy_parser_assert_token(
            expression(),
            &tokens!(
                ident: "abc",
                symbol: Dot,
                ident: "get",
                symbol: RoundBracketStart,
                ident: "c",
                symbol: Comma,
                ident: "d",
                symbol: RoundBracketEnd,
            ),
            Expression {
                term: Term::SubroutineCall(SubroutineCall {
                    subroutine_holder_name: Some("abc".to_string()),
                    subroutine_name: "get".to_string(),
                    subroutine_args: vec![
                        Expression {
                            term: Term::Identifier("c".to_string()),
                            subsequent_terms: vec![],
                        },
                        Expression {
                            term: Term::Identifier("d".to_string()),
                            subsequent_terms: vec![],
                        },
                    ],
                }),
                subsequent_terms: vec![],
            },
        );
    }
}
