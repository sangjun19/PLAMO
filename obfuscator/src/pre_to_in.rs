use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::collections::VecDeque;
use std::path::Path;

// Operator 체크 함수
fn is_operator(token: &str) -> bool {
    matches!(token, "+" | "-" | "*" | "/" | "&" | "|" | "^" | "~" | "<<" | ">>" | "%")
}

// Prefix 식을 Infix로 변환
fn prefix_to_infix(prefix_str: &str) -> String {
    let binding = prefix_str.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<&str> = binding.split_whitespace().collect();
    let mut stack = VecDeque::new();

    for token in tokens.iter().rev() {
        if !is_operator(token) && token != &"(" && token != &")" {
            stack.push_back(token.to_string());
            // println!("{} ", token); // 디버깅용 출력
        } else if is_operator(token) {
            if token == &"~" {
                let operand = stack.pop_back().unwrap();
                stack.push_back(format!("({}{})", token, operand));
            } else {
                if stack.len() < 2 {
                    continue; // 피연산자가 부족한 경우 스킵
                }
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                stack.push_back(format!("({}{}{})", right, token, left));
            }
        }
    }

    if stack.is_empty() {
        return String::new(); // 스택이 비어 있을 경우 빈 문자열 반환
    }

    stack.pop_back().unwrap()
}

pub fn pre_to_in(input_file_path: &Path, output_file_path: &Path) {        
    // 파일 열기
    let input_file = File::open(input_file_path).expect("Failed to open input file");
    let reader = BufReader::new(input_file);
    let output = File::create(output_file_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() != 3 {
                continue;
            }

            let original_prefix = parts[0];
            let obfuscated_prefix = parts[1];

            let original_infix = prefix_to_infix(original_prefix);
            let obfuscated_infix = prefix_to_infix(obfuscated_prefix);

            writeln!(writer, "{}, {}, {}", original_infix, obfuscated_infix, parts[2]).expect("Failed to write line");
        }
    }
}
