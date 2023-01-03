use itertools::Itertools;

use schema::jack::token_analyzer::*;
use schema::jack::tokenizer::*;

pub fn class_to_xml(class: &Class) -> String {
    XmlNode::from_class(class).into_string(0)
}

#[derive(Debug, Clone)]
enum XmlNode {
    Terminal {
        key: &'static str,
        value: String,
    },
    NonTerminal {
        key: &'static str,
        values: Vec<XmlNode>,
    },
}

impl XmlNode {
    fn into_string(self, num_indent: usize) -> String {
        match self {
            Self::Terminal { key, value } => {
                let sanitized_value = super::sanitize(value.to_string());
                let indent = INDENT.repeat(num_indent);
                format!("{indent}<{key}> {sanitized_value} </{key}>")
            }
            Self::NonTerminal { key, values } => {
                let indent = INDENT.repeat(num_indent);
                std::iter::once(format!("{indent}<{key}>"))
                    .chain(
                        values
                            .into_iter()
                            .map(|node| node.into_string(num_indent + 1)),
                    )
                    .chain(std::iter::once(format!("{indent}</{key}>")))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
    fn from_class(class: &Class) -> Self {
        Self::NonTerminal {
            key: "class",
            values: [
                Self::from_identifier(&class.class_name),
                Self::from_symbol(Symbol::WaveBracketStart),
            ]
            .into_iter()
            .chain(
                class
                    .variable_declearations
                    .iter()
                    .map(Self::from_class_var_dec),
            )
            .chain(
                class
                    .subroutine_declerations
                    .iter()
                    .map(Self::from_subroutine_dec),
            )
            .chain(std::iter::once(Self::from_symbol(Symbol::WaveBracketEnd)))
            .collect(),
        }
    }
    fn from_keyword(keyword: impl Into<Keyword>) -> Self {
        Self::Terminal {
            key: "keyword",
            value: keyword.into().as_str().to_string(),
        }
    }
    fn from_symbol(symbol: impl Into<Symbol>) -> Self {
        Self::Terminal {
            key: "symbol",
            value: symbol.into().as_str().to_string(),
        }
    }
    fn from_integer_constant(int: &u16) -> Self {
        Self::Terminal {
            key: "integerConstant",
            value: int.to_string(),
        }
    }
    fn from_string_constant(str: &str) -> Self {
        Self::Terminal {
            key: "stringConstant",
            value: str.to_string(),
        }
    }
    fn from_identifier(ident: &str) -> Self {
        Self::Terminal {
            key: "identifier",
            value: ident.to_string(),
        }
    }
    fn from_class_var_dec(class_var_dec: &ClassVariableDecleration) -> Self {
        Self::NonTerminal {
            key: "classVarDec",
            values: [
                Self::from_keyword(class_var_dec.decleration_type),
                Self::from_type(&class_var_dec.return_type),
            ]
            .into_iter()
            .chain(
                #[allow(unstable_name_collisions)]
                class_var_dec
                    .var_names
                    .iter()
                    .map(|s| Self::from_identifier(&s))
                    .intersperse(Self::from_symbol(Symbol::Comma)),
            )
            .collect(),
        }
    }
    fn from_type(type_dec: &TypeDecleration) -> Self {
        match type_dec {
            TypeDecleration::Int => Self::from_keyword(Keyword::Int),
            TypeDecleration::Char => Self::from_keyword(Keyword::Char),
            TypeDecleration::Boolean => Self::from_keyword(Keyword::Boolean),
            TypeDecleration::ClassName(class_name) => Self::from_identifier(&class_name),
        }
    }
    fn from_subroutine_dec(subroutine_dec: &ClassSubroutineDecleration) -> Self {
        Self::NonTerminal {
            key: "subroutineDec",
            values: vec![
                Self::from_keyword(subroutine_dec.decleration_type),
                Self::from_subroutine_return_type(&subroutine_dec.return_type),
                Self::from_identifier(&subroutine_dec.name),
                Self::from_symbol(Symbol::RoundBracketStart),
                Self::from_parameter_list(&subroutine_dec.parameters),
                Self::from_symbol(Symbol::RoundBracketEnd),
                Self::from_subroutine_body(&subroutine_dec.body),
            ],
        }
    }
    fn from_subroutine_return_type(subroutine_return_type: &ClassSubroutineReturnType) -> Self {
        match subroutine_return_type {
            ClassSubroutineReturnType::Type(return_type) => Self::from_type(return_type),
            ClassSubroutineReturnType::Void => Self::from_keyword(Keyword::Void),
        }
    }
    fn from_parameter_list(parameter_list: &[ClassSubroutineParameter]) -> Self {
        #[allow(unstable_name_collisions)]
        Self::NonTerminal {
            key: "parameterList",
            values: parameter_list
                .iter()
                .map(|parameter| {
                    vec![
                        Self::from_type(&parameter.parameter_type),
                        Self::from_identifier(&parameter.name),
                    ]
                })
                .intersperse(vec![Self::from_symbol(Symbol::Comma)])
                .flatten()
                .collect(),
        }
    }
    fn from_subroutine_body(subroutine_body: &SubroutineBody) -> Self {
        Self::NonTerminal {
            key: "subroutineBody",
            values: std::iter::once(Self::from_symbol(Symbol::WaveBracketStart))
                .into_iter()
                .chain(
                    subroutine_body
                        .variable_declerations
                        .iter()
                        .map(Self::from_var_dec),
                )
                .chain(std::iter::once(Self::from_statements(
                    &subroutine_body.statements,
                )))
                .chain(std::iter::once(Self::from_symbol(Symbol::WaveBracketEnd)))
                .collect(),
        }
    }
    fn from_var_dec(var_dec: &SubroutineVariableDecleration) -> Self {
        Self::NonTerminal {
            key: "varDec",
            values: [
                Self::from_keyword(Keyword::Var),
                Self::from_type(&var_dec.variable_type),
            ]
            .into_iter()
            .chain(
                #[allow(unstable_name_collisions)]
                var_dec
                    .names
                    .iter()
                    .map(|name| Self::from_identifier(name))
                    .intersperse(Self::from_symbol(Symbol::Comma)),
            )
            .collect(),
        }
    }
    fn from_statements(statements: &[Statement]) -> Self {
        Self::NonTerminal {
            key: "statements",
            values: statements
                .iter()
                .map(|statement| match statement {
                    Statement::Let(let_statement) => Self::from_let_statement(let_statement),
                    Statement::If(if_statement) => Self::from_if_statement(if_statement),
                    Statement::While(while_statement) => {
                        Self::from_while_statement(while_statement)
                    }
                    Statement::Do(do_statement) => Self::from_do_statement(do_statement),
                    Statement::Return(return_statement) => {
                        Self::from_return_statement(return_statement)
                    }
                })
                .collect(),
        }
    }
    fn from_while_statement(while_statement: &WhileStatement) -> Self {
        unimplemented!()
    }
    fn from_if_statement(if_statement: &IfStatement) -> Self {
        unimplemented!()
    }
    fn from_return_statement(return_statement: &ReturnStatement) -> Self {
        unimplemented!()
    }
    fn from_let_statement(let_statement: &LetStatement) -> Self {
        Self::NonTerminal {
            key: "letStatement",
            values: [
                vec![
                    Self::from_keyword(Keyword::Let),
                    Self::from_identifier(&let_statement.target_name),
                ],
                match &let_statement.target_index {
                    Some(index_expr) => vec![
                        Self::from_symbol(Symbol::SquareBracketStart),
                        Self::from_expression(index_expr),
                        Self::from_symbol(Symbol::SquareBracketEnd),
                    ],
                    None => vec![],
                },
                vec![
                    Self::from_symbol(Symbol::Equal),
                    Self::from_expression(&let_statement.source),
                ],
            ]
            .into_iter()
            .flatten()
            .collect(),
        }
    }
    fn from_do_statement(do_statement: &DoStatement) -> Self {
        unimplemented!()
    }
    fn from_expression(expression: &Expression) -> Self {
        unimplemented!()
    }
    fn from_term(term: &Term) -> Self {
        unimplemented!()
    }
    fn from_expression_list(expresion_list: &[Expression]) -> Self {
        unimplemented!()
    }
}

const INDENT: &str = "  ";

#[test]
fn test_xml_to_string() {
    let xml_node = XmlNode::NonTerminal {
        key: "expression",
        values: vec![XmlNode::NonTerminal {
            key: "term",
            values: vec![XmlNode::Terminal {
                key: "keyword",
                value: "true".to_string(),
            }],
        }],
    };
    assert_eq!(
        xml_node.to_string(0),
        vec![
            "<expression>",
            "  <term>",
            "    <keyword> true </keyword>",
            "  </term>",
            "</expression>",
        ]
        .join("\n")
    )
}
