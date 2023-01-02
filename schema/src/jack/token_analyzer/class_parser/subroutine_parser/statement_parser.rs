use super::expression_parser::{expression, subroutine_call, Expression, SubroutineCall};
use crate::jack::token_analyzer::{
    combine_extension::SkipSemicolon,
    custom_combinators::between::{
        between_round_bracket, between_square_bracket, between_wave_bracket,
    },
    custom_parser::{identifier, keyword, symbol},
};
use crate::jack::tokenizer::{Keyword, Symbol, Token};
use combine::{choice, many, optional, parser, Stream};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    While(WhileStatement),
    Do(DoStatement),
    Return(ReturnStatement),
}

parser! {
    pub(crate) fn statement[Input]()(Input) -> Statement
    where [Input: Stream<Token = Token>]
    {
        choice((
            let_statement().map(Statement::Let),
            if_statement().map(Statement::If),
            while_statement().map(Statement::While),
            do_statement().map(Statement::Do),
            return_statement().map(Statement::Return),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LetStatement {
    pub source: Expression,
    pub target_name: String,
    pub target_index: Option<Expression>,
}

parser! {
    pub(crate) fn let_statement[Input]()(Input) -> LetStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Let)
        .with(identifier()) // varName
        .and(optional(between_square_bracket(expression())))
        .skip(symbol(Symbol::Equal))
        .and(expression())
        .skip_semicolon()
        .map(|((target_name,target_index),source)|LetStatement{
            source,
            target_name,
            target_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_statements: Vec<Statement>,
    pub else_statements: Vec<Statement>, // else 句が無い場合は空
}

parser! {
    pub(crate) fn if_statement[Input]()(Input) -> IfStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::If)
        .with(between_round_bracket(expression()))
        .and(between_wave_bracket(many(statement())))
        .and(optional(
            keyword(Keyword::Else)
            .with(between_wave_bracket(many(statement())))
        ))
        .map(|((condition,if_statements), else_statements)|IfStatement{
            condition,
            if_statements,
            else_statements: else_statements.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WhileStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
}

parser! {
    pub(crate) fn while_statement[Input]()(Input) -> WhileStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::While)
        .with(between_round_bracket(expression()))
        .and(between_wave_bracket(many(statement())))
        .map(|(condition, statements)|WhileStatement{
            condition,
            statements,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DoStatement {
    pub subroutine_call: SubroutineCall,
}

parser! {
    pub(crate) fn do_statement[Input]()(Input) -> DoStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Do)
        .with(subroutine_call())
        .skip_semicolon()
        .map(|subroutine_call|DoStatement{
            subroutine_call,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}

parser! {
    pub(crate) fn return_statement[Input]()(Input) -> ReturnStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Return)
        .with(optional(expression()))
        .skip_semicolon()
        .map(|expression|ReturnStatement{
            expression,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::expression_parser::{KeywordConstant, Term};
    use crate::jack::token_analyzer::tests::{easy_parser_assert_token, tokens};

    // true
    fn expr_true() -> Expression {
        Expression {
            term: Term::KeywordConstant(KeywordConstant::True),
            subsequent_terms: vec![],
        }
    }

    // 1
    fn expr_one() -> Expression {
        Expression {
            term: Term::IntegerConstant(1),
            subsequent_terms: vec![],
        }
    }

    fn let_a_equal_one() -> Vec<Token> {
        tokens!(
            keyword: Let,
            ident: "a",
            symbol: Equal,
            int_const: 1,
            symbol: SemiColon,
        )
    }
    fn let_b_equal_one() -> Vec<Token> {
        tokens!(
            keyword: Let,
            ident: "b",
            symbol: Equal,
            int_const: 1,
            symbol: SemiColon,
        )
    }

    #[test]
    fn parse_statement_recursive() {
        /*
            if (expr) {
                let a = expr;
            }
        */
        easy_parser_assert_token(
            statement(),
            &vec![
                tokens!(
                    keyword: If,
                    symbol: RoundBracketStart,
                    keyword: True,
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                let_a_equal_one(),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            Statement::If(IfStatement {
                condition: expr_true(),
                if_statements: vec![Statement::Let(LetStatement {
                    source: expr_one(),
                    target_name: "a".to_string(),
                    target_index: None,
                })],
                else_statements: vec![],
            }),
        );
    }

    #[test]
    fn parse_let_statement() {
        easy_parser_assert_token(
            let_statement(),
            &let_a_equal_one(),
            LetStatement {
                source: expr_one(),
                target_name: "a".to_string(),
                target_index: None,
            },
        );
        // index ありの場合
        easy_parser_assert_token(
            let_statement(),
            &tokens!(
                keyword: Let,
                ident: "a",
                symbol: SquareBracketStart,
                int_const: 1,
                symbol: SquareBracketEnd,
                symbol: Equal,
                int_const: 1,
                symbol: SemiColon,
            ),
            LetStatement {
                source: expr_one(),
                target_name: "a".to_string(),
                target_index: Some(expr_one()),
            },
        )
    }

    #[test]
    fn parse_if_statement() {
        /*
            if (expr) {
                let a = expr;
            }
        */
        easy_parser_assert_token(
            if_statement(),
            &vec![
                tokens!(
                    keyword: If,
                    symbol: RoundBracketStart,
                    keyword: True,
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                let_a_equal_one(),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            IfStatement {
                condition: expr_true(),
                if_statements: vec![Statement::Let(LetStatement {
                    source: expr_one(),
                    target_name: "a".to_string(),
                    target_index: None,
                })],
                else_statements: vec![],
            },
        );
        /*
            if (expr) {
                let a = expr;
            } else {
                let b = expr;
            }
        */
        easy_parser_assert_token(
            if_statement(),
            &vec![
                tokens!(
                    keyword: If,
                    symbol: RoundBracketStart,
                    keyword: True,
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                let_a_equal_one(),
                tokens!(
                    symbol: WaveBracketEnd,
                    keyword: Else,
                    symbol: WaveBracketStart,
                ),
                let_b_equal_one(),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            IfStatement {
                condition: expr_true(),
                if_statements: vec![Statement::Let(LetStatement {
                    source: expr_one(),
                    target_name: "a".to_string(),
                    target_index: None,
                })],
                else_statements: vec![Statement::Let(LetStatement {
                    source: expr_one(),
                    target_name: "b".to_string(),
                    target_index: None,
                })],
            },
        );
    }

    #[test]
    fn parse_while_statement() {
        /*
            while (expr) {
                let a = expr;
            }
        */
        easy_parser_assert_token(
            while_statement(),
            &vec![
                tokens!(
                    keyword: While,
                    symbol: RoundBracketStart,
                    keyword: True,
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                let_a_equal_one(),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            WhileStatement {
                condition: expr_true(),
                statements: vec![Statement::Let(LetStatement {
                    source: expr_one(),
                    target_name: "a".to_string(),
                    target_index: None,
                })],
            },
        );
    }

    #[test]
    fn parse_do_statement() {
        /*
            do subroutineCall;
        */
        easy_parser_assert_token(
            do_statement(),
            &tokens!(
                keyword: Do,
                ident: "get",
                symbol: RoundBracketStart,
                int_const: 1,
                symbol: RoundBracketEnd,
                symbol: SemiColon,
            ),
            DoStatement {
                subroutine_call: SubroutineCall {
                    subroutine_holder_name: None,
                    subroutine_name: "get".to_string(),
                    subroutine_args: vec![expr_one()],
                },
            },
        );
    }

    #[test]
    fn parse_return_statement() {
        /*
            return expression;
        */
        easy_parser_assert_token(
            return_statement(),
            &tokens!(keyword: Return, keyword: True, symbol: SemiColon,),
            ReturnStatement {
                expression: Some(expr_true()),
            },
        );
        /*
            return;
        */
        easy_parser_assert_token(
            return_statement(),
            &tokens!(keyword: Return, symbol: SemiColon,),
            ReturnStatement { expression: None },
        );
    }
}
