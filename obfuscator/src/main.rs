use std::path::Path;

use ast_transform::generate_obfuscated_expression_ast;
use verify_z3::verifying_expressions;

pub mod ast;
pub mod rule_loader;
mod verify_z3;
mod count_expression;
mod ast_transform;

fn main() {
    let input_data_path = Path::new("./../data/input_data/input.txt");
    let output_data_path = Path::new("./../data/output_data/smt_delight.txt");
    let rules: Vec<&str> = vec!["smt_delight"];
    let rules_slice: &[&str] = &rules;
    // generate_obfuscated_expression_ast(input_data_path, output_data_path, rules_slice, 2);
    verifying_expressions();
}