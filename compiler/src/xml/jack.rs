use std::num;

use schema::jack::token_analyzer::*;
use schema::jack::tokenizer::*;

pub fn class_to_xml(class: &Class) -> String {
    XmlNode::from_class(class).to_string(0)
}

#[derive(Debug)]
enum XmlNode {
    Terminal {
        key: &'static str,
        value: &'static str,
    },
    NonTerminal {
        key: &'static str,
        values: Vec<XmlNode>,
    },
}

impl XmlNode {
    fn from_keyword(keyword: &Keyword) -> Self {
        unimplemented!()
    }
    fn from_symbol(symbol: &Symbol) -> Self {
        unimplemented!()
    }
    fn from_integer_constant(int: &u16) -> Self {
        unimplemented!()
    }
    fn from_string_constant(str: &str) -> Self {
        unimplemented!()
    }
    fn from_identifier(ident: &str) -> Self {
        unimplemented!()
    }
    fn from_class(class: &Class) -> Self {
        unimplemented!()
    }
    fn from_class_var_dec(class_var_dec: &ClassVariableDecleration) -> Self {
        unimplemented!()
    }
    fn from_subroutine_dec(subroutine_dec: &ClassSubroutineDecleration) -> Self {
        unimplemented!()
    }
    fn from_parameter_list(subroutine_dec: &[ClassSubroutineParameter]) -> Self {
        unimplemented!()
    }
    fn from_subroutine_body(subroutine_body: SubroutineBody) -> Self {
        unimplemented!()
    }
    fn from_var_dec(var_dec: &SubroutineVariableDecleration) -> Self {
        unimplemented!()
    }
    fn from_statements(statements: &[Statement]) -> Self {
        unimplemented!()
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
        unimplemented!()
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
    fn to_string(self, num_indent: usize) -> String {
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
                            .map(|node| node.to_string(num_indent + 1)),
                    )
                    .chain(std::iter::once(format!("{indent}</{key}>")))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
}

#[test]
fn test_xml_to_string() {
    let xml_node = XmlNode::NonTerminal {
        key: "expression",
        values: vec![XmlNode::NonTerminal {
            key: "term",
            values: vec![XmlNode::Terminal {
                key: "keyword",
                value: "true",
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

const INDENT: &str = "  ";
