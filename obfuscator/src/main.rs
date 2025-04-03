use ast_transform::generate_obfuscated_expression_ast;

pub mod ast;
pub mod rule_loader;
mod count_expression;
mod ast_transform;


fn main() {
    generate_obfuscated_expression_ast("smt_delight", 1);
}
