use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use ast_transform::generate_obfuscated_expression_ast;
use egg::{RecExpr, Runner, Extractor, AstSize, Rewrite, Analysis, EGraph, DidMerge, Id};
use crate::ast::Expr;

pub mod ast;
pub mod rule_loader;
mod count_expression;
mod ast_transform;


fn main() {
    generate_obfuscated_expression_ast("smt_delight", 5);
}
