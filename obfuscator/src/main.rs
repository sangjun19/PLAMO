use std::path::Path;

use ast_transform::generate_obfuscated_expression_ast;
use remove_duplicate::remove_duplicates;
use pre_to_in::pre_to_in;

pub mod ast;
pub mod rule_loader;
mod count_expression;
mod ast_transform;
mod remove_duplicate;
mod pre_to_in;

fn main() {
    let input_data_path = Path::new("./../data/input_data/input.txt");
    let obfuscated_data_path = Path::new("./../data/obfuscated_data/smt_delight.txt");
    let unique_data_path = Path::new("./../data/unique_data/smt_delight.txt");
    let result_data_path: &Path = Path::new("./../data/result_data/smt_delight.txt");
    let rules: Vec<&str> = vec!["smt_delight"];
    let rules_slice: &[&str] = &rules;
    generate_obfuscated_expression_ast(input_data_path, obfuscated_data_path, rules_slice, 2);
    remove_duplicates(obfuscated_data_path, unique_data_path);
    pre_to_in(unique_data_path, result_data_path);
}