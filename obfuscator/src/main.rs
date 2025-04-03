use std::path::Path;

use ast_transform::generate_obfuscated_expression_ast;

pub mod ast;
pub mod rule_loader;
mod count_expression;
mod ast_transform;

fn main() {
    let input_data_path = Path::new("./../data/input_data/test.txt");
    let output_data_path = Path::new("./../data/output_data/test.txt");
    let rules: Vec<&str> = vec!["test", "test2"];
    let rules_slice: &[&str] = &rules;
    generate_obfuscated_expression_ast(input_data_path, output_data_path, rules_slice, 2);
}