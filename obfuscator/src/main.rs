use std::path::Path;

use ast_transform::generate_obfuscated_expression_ast;
use verify_z3_vec::verify_vec;
use verify_z3_int::verify_int;
use verify_z3_rule::verify_rule;
use remove_duplicate::remove_duplicates;
use pre_to_in::pre_to_in;

pub mod ast;
pub mod rule_loader;
mod verify_z3_vec;
mod verify_z3_int;
mod verify_z3_rule;
mod count_expression;
mod ast_transform;
mod remove_duplicate;
mod pre_to_in;

fn main() {
    let file_name = "loki_delight";
    let recursive = 5;
    let input_data_path = Path::new("./../data/input_data/input.txt");

    let obfuscated_data_string = format!("./../data/obfuscated_data/{}_{}.txt", file_name, recursive);
    let obfuscated_data_path = Path::new(&obfuscated_data_string);

    let unique_data_string = format!("./../data/unique_data/{}_{}.txt", file_name, recursive);
    let unique_data_path = Path::new(&unique_data_string);

    let result_data_string = format!("./../data/result_data/{}_{}.txt", file_name, recursive);
    let result_data_path = Path::new(&result_data_string);

    let rules: Vec<&str> = vec!["loki", "delight"];
    let rules_slice: &[&str] = &rules;

    let rule_path: &Path = Path::new("./../rule/general.txt");

    generate_obfuscated_expression_ast(input_data_path, obfuscated_data_path, rules_slice, 5);
    remove_duplicates(obfuscated_data_path, unique_data_path);
    pre_to_in(unique_data_path, result_data_path);
    // verify_vec(unique_data_path);
    // verify_rule(rule_path);
}