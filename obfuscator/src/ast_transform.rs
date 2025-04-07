use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use crate::{ast::Expr, rule_loader};

// 트리의 크기를 구하는 함수
fn calculate_tree_size(tree: &Box<Expr>) -> usize {
    match **tree {
        Expr::BExpr(ref left, _, ref right) => 1 + calculate_tree_size(left) + calculate_tree_size(right),
        Expr::UExpr(_, ref inner) => 1 + calculate_tree_size(inner),
        _ => 1,
    }
}

// 트리와 패턴이 일치하는지 확인하는 함수
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

// 재귀적으로 규칙을 적용하여 새로운 트리들을 생성
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

            // 왼쪽과 오른쪽 자식 노드에 대해 각각 규칙을 적용한 결과를 조합
            for left in left_results {
                results.push(Box::new(Expr::BExpr(left, op.clone(), right.clone())));
            }

            for right in right_results {
                results.push(Box::new(Expr::BExpr(left.clone(), op.clone(), right)));
            }
        }
        // 단항 연산자에 대해 재귀적으로 규칙 적용
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

// 규칙을 로드하여 트리 형태로 변환
pub fn generate_rules(rule_names: &[&str]) -> Vec<(Box<Expr>, Box<Expr>)> {
    let mut rule_set_tree: Vec<(Box<Expr>, Box<Expr>)> = Vec::new();
    for rule in rule_names {
        let rule_file_name = format!("./../rule/{}.txt", rule);
        let rule_file_path = Path::new(&rule_file_name);

        if !rule_file_path.exists() {
            eprintln!("Error: File does not exist - {:?}", rule_file_path);
            continue;
        }

        rule_set_tree.extend(rule_loader::load_ruleset(&rule_file_path));
    }
    rule_set_tree
}

pub fn generate_obfuscated_expression_ast(origin_file_path: &Path, output_file_path: &Path, rule_names: &[&str], max_iterations: usize) {

    if !origin_file_path.exists() {
        eprintln!("Error: File does not exist - {:?}", origin_file_path);
        return;
    }

    // 적용할 규칙들 로드
    let rule_set_tree = generate_rules(rule_names);

    // 결과값 저장할 파일 열기
    let mut output_file = File::create(&output_file_path).unwrap();

    // 원본 파일 열기
    let input_file = File::open(&origin_file_path).unwrap();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let expr = line.unwrap();

        // 원본 표현식을 AST로 파싱
        let parse_tree = rule_loader::parse_expression(&expr);

        // 원본 표현식 문자열 저장
        let original_str = rule_loader::expr_to_string(parse_tree.clone(), false);

        // 난독화 초기 상태 설정
        let mut obfuscated_trees = vec![parse_tree];

        // 최대 반복 횟수만큼 난독화 수행
        for _ in 0..max_iterations {
            let mut next_trees = Vec::new();

            // 현재 난독화된 모든 결과에 대해 재난독화 수행
            for tree in &obfuscated_trees {
                let new_obfuscated_trees = apply_rules_recursively(tree.clone(), &*rule_set_tree);
                next_trees.extend(new_obfuscated_trees.clone());

                // 현재 단계에서 생성된 결과를 출력 파일에 기록
                for new_tree in &new_obfuscated_trees {
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

            // 더 이상 난독화할 수 없으면 종료
            if next_trees.is_empty() {
                break;
            }

            obfuscated_trees = next_trees; // 다음 단계의 입력으로 사용
        }
    }
}
