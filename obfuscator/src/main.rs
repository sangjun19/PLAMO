use std::path::Path;

use ast_transform::generate_obfuscated_expression_ast;
use verify_z3_vec::verify_vec;
use verify_z3_int::verify_int;
use remove_duplicate::remove_duplicates;
use pre_to_in::pre_to_in;

pub mod ast;
pub mod rule_loader;
mod verify_z3_vec;
mod verify_z3_int;
mod count_expression;
mod ast_transform;
mod remove_duplicate;
mod pre_to_in;

fn main() {
    let file_name = "mba_delight";
    let input_data_path = Path::new("./../data/input_data/input.txt");

    let obfuscated_data_string = format!("./../data/obfuscated_data/{}.txt", file_name);
    let obfuscated_data_path = Path::new(&obfuscated_data_string);

    let unique_data_string = format!("./../data/unique_data/{}.txt", file_name);
    let unique_data_path = Path::new(&unique_data_string);

    let result_data_string = format!("./../data/result_data/{}.txt", file_name);
    let result_data_path = Path::new(&result_data_string);

    let rules: Vec<&str> = vec!["mba_delight"];
    let rules_slice: &[&str] = &rules;

    generate_obfuscated_expression_ast(input_data_path, obfuscated_data_path, rules_slice, 4);
    remove_duplicates(obfuscated_data_path, unique_data_path);
    pre_to_in(unique_data_path, result_data_path);
    verify_vec(unique_data_path);
}