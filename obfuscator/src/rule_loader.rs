
use std::{fs::File, io, io::{prelude::*, BufReader}, path::Path};
use itertools::Itertools;
use egg::{Rewrite, rewrite, Pattern, define_language, Symbol, Id, Var, EGraph, Runner, RecExpr};
use crate::ast::{Expr, BOp, UOp};
use crate::rule_loader::mbaexpr::ExprParser;

use lalrpop_util::lalrpop_mod;
use rand::Rng;

define_language! {
    pub enum MixedBooleanArithmetic {
        Num(i32),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "%" = Mod([Id; 2]),
        "<<" = Shl([Id; 2]),
        ">>" = Shr([Id; 2]),
        "&" = And([Id; 2]),
        "|" = Or([Id; 2]),
        "^" = Xor([Id; 2]),
        "-" = Neg(Id),
        "~" = Not(Id),
        Symbol(Symbol),
    }
}

lalrpop_mod!(pub mbaexpr);

fn uop_to_string(op: crate::ast::UOp) -> String {
    match op {
        UOp::Not => "~".to_string(),
        UOp::Neg => "-".to_string(),
    }
}

fn bop_to_string(op: crate::ast::BOp) -> String {
    match op {
        BOp::Add => "+".to_string(),
        BOp::Sub => "-".to_string(),
        BOp::Mul => "*".to_string(),
        BOp::Div => "/".to_string(),
        BOp::Mod => "%".to_string(),
        BOp::Shl => "<<".to_string(),
        BOp::Shr => ">>".to_string(),
        BOp::And => "&".to_string(),
        BOp::Or => "|".to_string(),
        BOp::Xor => "^".to_string(),
    }
}

pub fn expr_to_string(parsed_expr: Box<crate::ast::Expr>, is_pattern: bool) -> String {
    match *parsed_expr {
        Expr::Number(n) => format!("{}", n),
        Expr::Variable(v) => is_pattern.then(|| "?".to_string()).unwrap_or("".to_string()) + &v,
        Expr::BExpr(e1, op, e2) => format!("({} {} {})", bop_to_string(op), expr_to_string(e1, is_pattern), expr_to_string(e2, is_pattern)),
        Expr::UExpr(op, e) => format!("({} {})", uop_to_string(op), expr_to_string(e, is_pattern)),
    }
}

pub fn string_to_mbastr(raw_expr: String) -> String {
    let parser: mbaexpr::ExprParser = mbaexpr::ExprParser::new();
    // println!("Parsing: {}", raw_expr);
    let parsed_expr = parser.parse(&raw_expr).unwrap();
    expr_to_string(parsed_expr, false)
}

fn expr_to_pattern(raw_expr: String) -> Pattern<MixedBooleanArithmetic> {
    let parser: mbaexpr::ExprParser = mbaexpr::ExprParser::new();
    let parsed_expr = parser.parse(&raw_expr).unwrap();
    let patstring = expr_to_string(parsed_expr, true);
    patstring.parse().unwrap()
}

pub fn parse_expression(input: &str) -> Box<Expr> {
    let parser = ExprParser::new();
    let parsed_tree = parser.parse(input).expect("Failed to parse expression");
    let result_parsed_tree = parser.parse(input).expect("Failed to parse expression");

    // println!("Debug: Parsed expression - {}", box_expr_to_string(parsed_tree));

    result_parsed_tree
}

pub fn box_expr_to_string(parsed_expr: Box<Expr>) -> String {
    expr_to_string(parsed_expr, false)
}

// rule to ast tree
pub fn load_ruleset(filename: impl AsRef<Path>) -> Vec<(Box<Expr>, Box<Expr>)> {
    let file = File::open(filename).expect("Failed to open ruleset file");
    let mut lines = io::BufReader::new(file).lines();
    let mut ruleset = Vec::new();

    while let (Some(Ok(lhs)), Some(Ok(rhs))) = (lines.next(), lines.next()) {
        let lhs_expr = parse_expression(&lhs);
        let rhs_expr = parse_expression(&rhs);
        ruleset.push((lhs_expr, rhs_expr));
    }

    ruleset
}

// rule to e-graph data structure (maybe)
pub fn load_rule(filename: impl AsRef<Path>, is_base: bool) -> Vec<Rewrite<MixedBooleanArithmetic, ()>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let raw_rule: Vec<(String, String)> = buf.lines()
        .map(|l| l.expect("Could not get line"))
        .tuples()
        .collect();

    let mut rules: Vec<Rewrite<MixedBooleanArithmetic, ()>> = Vec::new();

    for (i, (lhs, rhs)) in raw_rule.iter().enumerate() {
        let lhs_pattern = expr_to_pattern(lhs.to_string());
        let rhs_pattern = expr_to_pattern(rhs.to_string());
        let lhs_vars = lhs_pattern.vars();
        let rhs_vars = rhs_pattern.vars();

        fn var_check(lhs: &Vec<Var> , rhs: &Vec<Var>) -> bool {
            for r in rhs {
                if !(lhs.contains(r)) {
                    return false;
                }
            }
            true
        }

        let rule_name;
        if is_base { rule_name = format!("baserule-{}", i); }
        else { rule_name = format!("rule-{}", i); }

        // println!("=====debug=======lhs_pattern {}", lhs_pattern);
        // println!("=====debug=======rhs_pattern {}", rhs_pattern);

        if var_check(&lhs_vars, &rhs_vars) {
            let rewrite = rewrite!(rule_name; lhs_pattern => rhs_pattern);
            rules.push(rewrite);
        }
    }

    rules
}

// pub fn insert_tree(expr: Box<Expr>, target_op: BOp, subtree: Box<Expr>) -> Box<Expr> {
//     match *expr {
//         Expr::BExpr(left, op, right) => {
//             if op == target_op {
//                 // 연산자를 만나면, 해당 위치에 subtree 삽입
//                 Box::new(Expr::BExpr(left, op, Box::new(Expr::BExpr(right, target_op, subtree))))
//             } else {
//                 // 재귀적으로 좌우 서브트리 탐색
//                 Box::new(Expr::BExpr(
//                     insert_tree(left, target_op.clone(), subtree.clone()),
//                     op,
//                     insert_tree(right, target_op, subtree),
//                 ))
//             }
//         }
//         Expr::UExpr(op, e) => {
//             Box::new(Expr::UExpr(op, insert_tree(e, target_op, subtree)))
//         }
//         _ => expr, // Number, Variable 등은 그대로 반환
//     }
// }