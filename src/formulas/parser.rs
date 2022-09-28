//! Formulas parser.

use inkwell::IntPredicate;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "formulas/formulas.pest"]
pub struct FormulasParser;

#[derive(Debug)]
pub enum FormulaError {
    Empty,
    ParseError(pest::error::Error<Rule>),
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FormulaOp {
    Lt,
    Eq,
    NotEq,
    LtEq,
    GtEq,
    Gt,
    Add,
    And,
    Or,
}

impl Into<IntPredicate> for FormulaOp {
    fn into(self) -> IntPredicate {
        match self {
            FormulaOp::Eq => IntPredicate::EQ,
            FormulaOp::NotEq => IntPredicate::NE,
            FormulaOp::Lt => IntPredicate::ULT,
            FormulaOp::Gt => IntPredicate::UGT,
            FormulaOp::LtEq => IntPredicate::ULE,
            FormulaOp::GtEq => IntPredicate::UGE,
            // TODO: fixme
            _ => panic!("invalid op conversion: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FormulaTerm {
    Property(String, String),
    Number(u64),
    Boolean(bool),
    String(String),
    CountCall(String),
}

impl FormulaTerm {
    pub fn str_value(&self) -> Option<&str> {
        match self {
            FormulaTerm::String(str) => Some(str),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FormulaExpr {
    Binary {
        lhs: Box<FormulaExpr>,
        rhs: Box<FormulaExpr>,
        binary_op: FormulaOp,
    },
    Term(FormulaTerm),
}

impl FormulaExpr {
    pub fn term(&self) -> Option<&FormulaTerm> {
        match self {
            FormulaExpr::Term(term) => Some(term),
            _ => None,
        }
    }
}

impl From<&str> for FormulaExpr {
    fn from(str: &str) -> Self {
        FormulaExpr::Term(FormulaTerm::String(str.to_string()))
    }
}

impl From<u64> for FormulaExpr {
    fn from(num: u64) -> Self {
        FormulaExpr::Term(FormulaTerm::Number(num))
    }
}

fn parse_term(pair: Pair<Rule>) -> FormulaTerm {
    match pair.as_rule() {
        Rule::property => {
            let mut pair = pair.into_inner();
            let ident1 = pair.next().unwrap();
            let ident2 = pair.next().unwrap();
            FormulaTerm::Property(ident1.as_str().to_string(), ident2.as_str().to_string())
        }
        Rule::literal_value => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::literal_number => FormulaTerm::Number(pair.as_str().parse().unwrap()),
                Rule::literal_string => {
                    FormulaTerm::String(pair.into_inner().next().unwrap().as_str().to_string())
                }
                Rule::literal_bool => {
                    FormulaTerm::Boolean(pair.into_inner().as_str().parse().unwrap())
                }
                _ => todo!(),
            }
        }
        Rule::count => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::ident => FormulaTerm::CountCall(pair.as_str().to_string()),
                _ => todo!(),
            }
        }
        _ => {
            dbg!(&pair);
            todo!();
        }
    }
}

fn parse_expr(pair: Pair<Rule>) -> FormulaExpr {
    match pair.as_rule() {
        Rule::expression => {
            let mut pair = pair.into_inner();
            let lhs = parse_expr(pair.next().unwrap());

            let next_op = pair.next();

            if let Some(true) = next_op.as_ref().map(|p| p.as_rule() == Rule::binary_op) {
                let rhs = parse_expr(pair.next().unwrap());

                return FormulaExpr::Binary {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    binary_op: match next_op.unwrap().as_str() {
                        "&&" => FormulaOp::And,
                        "||" => FormulaOp::Or,
                        ">=" => FormulaOp::GtEq,
                        "<=" => FormulaOp::LtEq,
                        "!=" => FormulaOp::NotEq,
                        ">" => FormulaOp::Gt,
                        "<" => FormulaOp::Lt,
                        "=" => FormulaOp::Eq,
                        "+" => FormulaOp::Add,
                        _ => todo!(),
                    },
                };
            } else {
                lhs
            }
        }
        Rule::term => {
            let term = pair.into_inner().next().unwrap();
            if term.as_rule() == Rule::expression {
                parse_expr(term)
            } else {
                FormulaExpr::Term(parse_term(term))
            }
        }
        _ => todo!(),
    }
}

pub fn parse_formula(input: &str) -> Result<FormulaExpr, FormulaError> {
    // TODO: must support nested expressions, e.g `(input.port + 2) + 1`
    if input.is_empty() {
        return Err(FormulaError::Empty);
    }

    let mut parser =
        FormulasParser::parse(Rule::expression, input).map_err(FormulaError::ParseError)?;

    let pair = parser
        .next()
        .ok_or_else(|| FormulaError::Other("no parse result".to_string()))?; // TODO: error

    Ok(parse_expr(pair))
}

#[cfg(test)]
mod tests {
    use super::{parse_formula, FormulaExpr, FormulaOp, FormulaTerm};

    #[test]
    fn test_basic_parser() {
        // let parsed = parse_formula("input.port >= 8080");

        let parsed = parse_formula("input.port >= 8080").unwrap();

        assert_eq!(
            parsed,
            FormulaExpr::Binary {
                lhs: Box::new(FormulaExpr::Term(FormulaTerm::Property(
                    "input".to_string(),
                    "port".to_string()
                ))),
                rhs: Box::new(FormulaExpr::Term(FormulaTerm::Number(8080))),
                binary_op: FormulaOp::GtEq
            }
        );

        let parsed = parse_formula("input.process_name = \"bash\"").unwrap();

        assert_eq!(
            parsed,
            FormulaExpr::Binary {
                lhs: Box::new(FormulaExpr::Term(FormulaTerm::Property(
                    "input".to_string(),
                    "process_name".to_string()
                ))),
                rhs: Box::new(FormulaExpr::Term(FormulaTerm::String("bash".to_string()))),
                binary_op: FormulaOp::Eq
            }
        );

        let parsed = parse_formula("count(input)").unwrap();

        assert_eq!(
            parsed,
            FormulaExpr::Term(FormulaTerm::CountCall("input".to_string(),))
        );
    }
}
