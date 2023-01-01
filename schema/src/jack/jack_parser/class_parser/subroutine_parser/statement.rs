use super::subroutine_call::{subroutine_call, SubroutineCall};
use crate::jack::jack_parser::common::{
    between_round_bracket, between_square_bracket, between_wave_bracket,
};
use crate::jack::jack_parser::*;
use combine::{many, optional};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub(crate) enum Statement {
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
pub(crate) struct LetStatement {
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
        .and(optional(between_square_bracket(expression_mock())))
        .skip(symbol(Symbol::Equal))
        .and(expression_mock())
        .skip_semicolon()
        .map(|((target_name,target_index),source)|LetStatement{
            source,
            target_name,
            target_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct IfStatement {
    pub condition: Expression,
    pub if_statements: Vec<Statement>,
    pub else_statements: Vec<Statement>, // else 句が無い場合は空
}

parser! {
    pub(crate) fn if_statement[Input]()(Input) -> IfStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::If)
        .with(between_round_bracket(expression_mock()))
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
pub(crate) struct WhileStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
}

parser! {
    pub(crate) fn while_statement[Input]()(Input) -> WhileStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::While)
        .with(between_round_bracket(expression_mock()))
        .and(between_wave_bracket(many(statement())))
        .map(|(condition, statements)|WhileStatement{
            condition,
            statements,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DoStatement {
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
pub(crate) struct ReturnStatement {
    expression: Option<Expression>,
}

parser! {
    pub(crate) fn return_statement[Input]()(Input) -> ReturnStatement
    where [Input: Stream<Token = Token>]
    {
        keyword(Keyword::Return)
        .with(optional(expression_mock()))
        .map(|expression|ReturnStatement{
            expression,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Expression {}

parser! {
    pub(crate) fn expression_mock[Input](

    )(Input) -> Expression
    where [Input: Stream<Token = Token>]
    {
        // TODO 実装する
        identifier()
        .with(value(Expression{}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jack::jack_parser::tests::easy_parser_assert_token;
    use crate::tokens;

    macro_rules! make_sample_let_statement(
        ($name: literal) => {
            tokens!(
                keyword: Let,
                ident: $name,
                symbol: SquareBracketStart,
                ident: "expression_mock",
                symbol: SquareBracketEnd,
                symbol: Equal,
                ident: "expression_mock",
                symbol: SemiColon,
            )
        }
    );

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
                    ident: "expression_mock",
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                make_sample_let_statement!("a"),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            Statement::If(IfStatement {
                condition: Expression {},
                if_statements: vec![Statement::Let(LetStatement {
                    source: Expression {},
                    target_name: "a".to_string(),
                    target_index: Some(Expression {}),
                })],
                else_statements: vec![],
            }),
        );
    }

    #[test]
    fn parse_let_statement() {
        easy_parser_assert_token(
            let_statement(),
            &make_sample_let_statement!("a"),
            LetStatement {
                source: Expression {},
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
                ident: "expression_mock",
                symbol: SquareBracketEnd,
                symbol: Equal,
                ident: "expression_mock",
                symbol: SemiColon,
            ),
            LetStatement {
                source: Expression {},
                target_name: "a".to_string(),
                target_index: Some(Expression {}),
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
                    ident: "expression_mock",
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                make_sample_let_statement!("a"),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            IfStatement {
                condition: Expression {},
                if_statements: vec![Statement::Let(LetStatement {
                    source: Expression {},
                    target_name: "a".to_string(),
                    target_index: Some(Expression {}),
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
                    ident: "expression_mock",
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                make_sample_let_statement!("a"),
                tokens!(
                    symbol: WaveBracketEnd,
                    keyword: Else,
                    symbol: WaveBracketStart,
                ),
                make_sample_let_statement!("b"),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            IfStatement {
                condition: Expression {},
                if_statements: vec![Statement::Let(LetStatement {
                    source: Expression {},
                    target_name: "a".to_string(),
                    target_index: Some(Expression {}),
                })],
                else_statements: vec![Statement::Let(LetStatement {
                    source: Expression {},
                    target_name: "b".to_string(),
                    target_index: Some(Expression {}),
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
                    ident: "expression_mock",
                    symbol: RoundBracketEnd,
                    symbol: WaveBracketStart,
                ),
                make_sample_let_statement!("a"),
                tokens!(symbol: WaveBracketEnd,),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
            WhileStatement {
                condition: Expression {},
                statements: vec![Statement::Let(LetStatement {
                    source: Expression {},
                    target_name: "a".to_string(),
                    target_index: Some(Expression {}),
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
                ident: "subroutine_call_mock",
                symbol: SemiColon,
            ),
            DoStatement {
                subroutine_call: SubroutineCall {},
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
            &tokens!(
                keyword: Return,
                ident: "expression_mock",
                symbol: SemiColon,
            ),
            ReturnStatement {
                expression: Some(Expression {}),
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
