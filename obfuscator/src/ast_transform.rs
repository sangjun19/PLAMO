use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use crate::{ast::Expr, rule_loader};

// Output format: original expression, obfuscated expression, tree size

pub fn apply_rules_recursively(tree: Box<Expr>, ruleset: &[(Box<Expr>, Box<Expr>)]) -> Vec<Box<Expr>> {
    let mut results = Vec::new();

    // 현재 트리에 대해 모든 규칙 적용
    for (pattern, replacement) in ruleset {
        if tree_matches_ast(&tree, pattern) {
            results.push(replacement.clone());
        }
    }

    // 자식 노드에도 재귀적으로 규칙 적용
    match *tree {
        Expr::BExpr(left, op, right) => {
            let left_results = apply_rules_recursively(left.clone(), ruleset);
            let right_results = apply_rules_recursively(right.clone(), ruleset);

            for left in left_results {
                results.push(Box::new(Expr::BExpr(left, op.clone(), right.clone())));
            }

            for right in right_results {
                results.push(Box::new(Expr::BExpr(left.clone(), op.clone(), right)));
            }
        }
        Expr::UExpr(op, inner) => {
            let inner_results = apply_rules_recursively(inner.clone(), ruleset);

            for inner in inner_results {
                results.push(Box::new(Expr::UExpr(op.clone(), inner)));
            }
        }
        _ => {}
    }

    results
}

fn tree_matches_ast(tree: &Box<Expr>, pattern: &Box<Expr>) -> bool {
    match (&**tree, &**pattern) {
        (Expr::BExpr(l1, op1, r1), Expr::BExpr(l2, op2, r2)) => {
            op1 == op2 && tree_matches_ast(l1, l2) && tree_matches_ast(r1, r2)
        }
        (Expr::UExpr(op1, e1), Expr::UExpr(op2, e2)) => {
            op1 == op2 && tree_matches_ast(e1, e2)
        }
        (Expr::Number(n1), Expr::Number(n2)) => n1 == n2,
        (Expr::Variable(v1), Expr::Variable(v2)) => v1 == v2,
        _ => false,
    }
}

pub fn generate_obfuscated_expression_ast(rule_name: &str, max_iterations: usize) {

    let origin_file_path = Path::new("./../data/input_data/input.txt");
    let output_file_name = format!("./../data/output_data/output_ast_mba_{}.txt", rule_name);
    let mba_expr_file_path = Path::new(&output_file_name);
    let rule_file_name = format!("./../rule/{}.txt", rule_name);
    let rule_file_path = Path::new(&rule_file_name);

    if !origin_file_path.exists() {
        eprintln!("Error: File does not exist - {:?}", origin_file_path);
        return;
    } 

    let mut mba_exprs = Vec::new();

    // Read input expressions
    let input_file = File::open(&origin_file_path).unwrap();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line.unwrap();
        mba_exprs.push(line);
    }

    // Load ruleset
    let rule_set_tree = rule_loader::load_ruleset(rule_file_path);

    // Open output file for writing
    let mut output_file = File::create(&mba_expr_file_path).unwrap();

    for expr in mba_exprs.iter() {
        // Parse the original expression into an AST
        let parse_tree = rule_loader::parse_expression(expr);

        // Convert the original AST to a string
        let original_str = rule_loader::expr_to_string(parse_tree.clone(), false);

        // 난독화 초기 상태 설정
        let mut obfuscated_trees = vec![parse_tree];

        for _iteration in 0..max_iterations {
            let mut next_trees = Vec::new();

            // 현재 난독화된 모든 결과에 대해 재난독화 수행
            for tree in obfuscated_trees.iter() {
                let new_obfuscated_trees = apply_rules_recursively(tree.clone(), &*rule_set_tree);
                next_trees.extend(new_obfuscated_trees.clone());

                // 현재 단계에서 생성된 결과를 출력 파일에 기록
                for new_tree in new_obfuscated_trees.iter() {
                    let obfuscated_str = rule_loader::expr_to_string(new_tree.clone(), false);
                    let tree_size = calculate_tree_size(new_tree);

                    writeln!(
                        output_file,
                        "{}, {}, {}",
                        original_str, obfuscated_str, tree_size
                    )
                    .unwrap();
                }
            }

            if next_trees.is_empty() {
                break; // 더 이상 난독화할 수 없으면 종료
            }

            obfuscated_trees = next_trees; // 다음 단계의 입력으로 사용

            // println!(
            //     "Iteration {}: Generated {} obfuscated trees",
            //     iteration + 1,
            //     obfuscated_trees.len()
            // );
        }
    }

    // println!("Obfuscation results written to {:?}", mba_expr_file_path);
    // print_expression_stats(mba_expr_file_path);
}

// Helper function to calculate the size of an expression tree
fn calculate_tree_size(tree: &Box<Expr>) -> usize {
    match **tree {
        Expr::BExpr(ref left, _, ref right) => 1 + calculate_tree_size(left) + calculate_tree_size(right),
        Expr::UExpr(_, ref inner) => 1 + calculate_tree_size(inner),
        _ => 1,
    }
}