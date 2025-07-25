pub mod keywords;
pub mod parser;
pub mod parser_common;
pub mod parser_expr;
pub mod parser_stmt;
pub mod parser_type;
use nom::{
    IResult,
    character::complete::{char, multispace0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::tuple,
};

use crate::ir::ast::Statement;
use crate::parser::parser_common::SEMICOLON_CHAR;

pub use parser_common::keyword;
pub use parser_expr::{parse_expression, parse_lambda};
pub use parser_stmt::{parse_formal_argument, parse_return_statement, parse_statement};
pub use parser_type::parse_type;

pub fn parse(input: &str) -> IResult<&str, Vec<Statement>> {
    map(
        tuple((
            multispace0,
            separated_list0(
                tuple((multispace0, char(SEMICOLON_CHAR), multispace0)),
                parse_statement,
            ),
            opt(tuple((multispace0, char(SEMICOLON_CHAR)))), // optional trailing semicolon
            multispace0,
        )),
        |(_, statements, _, _)| statements,
    )(input)
}

pub use parser::parse_chained_blocks;
