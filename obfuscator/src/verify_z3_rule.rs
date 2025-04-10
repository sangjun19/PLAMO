use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio, Child};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

fn read_formulas(file_path: &str) -> Vec<(String, String)> {
    let mut formulas = Vec::new();
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut cnt = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if cnt % 2 == 0 {
            let origin = line.to_string();
            formulas.push((origin, String::new())); // Temporarily push origin with an empty obfuscation
        } else {
            let obfuscation = line.to_string();
            if let Some(last) = formulas.last_mut() {
                last.1 = obfuscation; // Update the last tuple's obfuscation
            }
        }
        cnt += 1;
    }
    formulas
}

// 재귀적으로 표현식 파싱을 위한 개선된 접근법
fn infix_to_prefix(infix: &str) -> String {
    struct Parser {
        tokens: Vec<String>,
        pos: usize,
    }
    
    impl Parser {
        fn new(infix: &str) -> Self {
            let tokens = tokenize(infix);
            Self { tokens, pos: 0 }
        }
        
        fn peek(&self) -> Option<&String> {
            if self.pos < self.tokens.len() {
                Some(&self.tokens[self.pos])
            } else {
                None
            }
        }
        
        fn consume(&mut self) -> Option<String> {
            if self.pos < self.tokens.len() {
                let token = self.tokens[self.pos].clone();
                self.pos += 1;
                Some(token)
            } else {
                None
            }
        }
        
        fn parse(&mut self) -> String {
            self.expression()
        }
        
        fn expression(&mut self) -> String {
            self.binary_expression(0)
        }
        
        fn binary_expression(&mut self, min_precedence: i32) -> String {
            let mut left = self.unary_expression();
            
            while let Some(op) = self.peek() {
                let precedence = match op.as_str() {
                    "|" => 1,
                    "&" => 2,
                    "+" | "-" => 3,
                    "*" | "/" => 4,
                    "^" => 5,
                    _ => -1,
                };
                
                if precedence < min_precedence {
                    break;
                }
                
                let operator = self.consume().unwrap();
                let right = self.binary_expression(precedence + 1);
                left = format!("({} {} {})", operator, left, right);
            }
            
            left
        }
        
        fn unary_expression(&mut self) -> String {
            if let Some(op) = self.peek() {
                if op == "~" {
                    self.consume();
                    let operand = self.primary();
                    return format!("(~ {})", operand);
                }
            }
            
            self.primary()
        }
        
        fn primary(&mut self) -> String {
            if let Some(token) = self.peek() {
                if token == "(" {
                    self.consume(); // 왼쪽 괄호 소비
                    let expr = self.expression();
                    
                    if let Some(close) = self.peek() {
                        if close == ")" {
                            self.consume(); // 오른쪽 괄호 소비
                            return expr;
                        }
                    }
                    
                    // 괄호가 닫히지 않은 경우
                    return expr;
                } else if !["+", "-", "*", "/", "&", "|", "^", "~", ")", "("].contains(&token.as_str()) {
                    // 변수나 상수인 경우
                    return self.consume().unwrap();
                }
            }
            
            // 오류 상황 - 빈 문자열 반환
            String::new()
        }
    }
    
    fn tokenize(infix: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = infix.chars().collect();
        
        while i < chars.len() {
            match chars[i] {
                ' ' => {
                    i += 1;
                    continue;
                },
                '(' | ')' | '+' | '-' | '*' | '/' | '&' | '|' | '^' | '~' => {
                    tokens.push(chars[i].to_string());
                    i += 1;
                },
                _ => {
                    // 변수 또는 상수 (연속된 알파벳 또는 숫자)
                    let mut var = String::new();
                    while i < chars.len() && !['(', ')', '+', '-', '*', '/', '&', '|', '^', '~', ' '].contains(&chars[i]) {
                        var.push(chars[i]);
                        i += 1;
                    }
                    if !var.is_empty() {
                        tokens.push(var);
                    }
                }
            }
        }
        
        tokens
    }
    
    let mut parser = Parser::new(infix);
    parser.parse()
}

fn convert_to_z3_format(prefix: &str) -> String {
    // 정수 0을 32비트 비트벡터 상수로 변환
    let result = if prefix == "0" {
        "#x00000000".to_string()
    } else {
        // Z3 형식으로 변환
        prefix
            .replace("(+", "(bvadd")
            .replace("(-", "(bvsub")
            .replace("(*", "(bvmul")
            .replace("(&", "(bvand")
            .replace("(|", "(bvor")
            .replace("(~", "(bvnot")
            .replace("(^", "(bvxor")
    };
    
    result
}

// 시간 제한이 있는 Z3 프로세스 실행 함수
fn run_z3_with_timeout(
    init_code: &str, 
    command: &str, 
    timeout_seconds: u64
) -> Result<String, String> {
    let child = Command::new("./z3/z3.exe")
        .arg("-in")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();
    
    let mut child = match child {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to spawn z3 process: {}", e)),
    };
    
    // stdin에 명령어 전송
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(init_code.as_bytes()).unwrap();
        stdin.write_all(command.as_bytes()).unwrap();
        stdin.write_all(b"(check-sat)\n").unwrap();
        stdin.write_all(b"(exit)\n").unwrap();
    }
    
    // 공유 상태 생성
    let completed = Arc::new(AtomicBool::new(false));
    let result = Arc::new(Mutex::new(String::new()));
    let child_arc = Arc::new(Mutex::new(child));
    
    // 결과를 수집하는 스레드 생성
    let completed_clone = completed.clone();
    let result_clone = result.clone();
    let child_clone = child_arc.clone();
    let handle = thread::spawn(move || {
        // Take stdout from child
        let stdout = {
            let mut child = child_clone.lock().unwrap();
            child.stdout.take()
        };
        
        if let Some(mut stdout) = stdout {
            let mut output = Vec::new();
            if let Ok(_) = stdout.read_to_end(&mut output) {
                // Wait for child to exit
                {
                    let mut child = child_clone.lock().unwrap();
                    if let Ok(_) = child.wait() {
                        let output_str = String::from_utf8_lossy(&output).to_string();
                        let mut result = result_clone.lock().unwrap();
                        *result = output_str;
                    } else {
                        let mut result = result_clone.lock().unwrap();
                        *result = "Error waiting for child process".to_string();
                    }
                }
            } else {
                let mut result = result_clone.lock().unwrap();
                *result = "Error reading stdout".to_string();
            }
        } else {
            let mut result = result_clone.lock().unwrap();
            *result = "Failed to capture stdout".to_string();
        }
        
        completed_clone.store(true, Ordering::SeqCst);
    });
    
    // 타임아웃 처리
    let start_time = Instant::now();
    while !completed.load(Ordering::SeqCst) {
        if start_time.elapsed() > Duration::from_secs(timeout_seconds) {
            // 타임아웃 발생, 프로세스 종료
            let mut child = child_arc.lock().unwrap();
            child.kill().ok(); // 프로세스 강제 종료 (오류 무시)
            return Err(format!("Z3 process timed out after {} seconds", timeout_seconds));
        }
        thread::sleep(Duration::from_millis(100)); // CPU 사용량 줄이기 위한 짧은 대기
    }
    
    // 스레드 조인
    handle.join().ok();
    
    // 결과 반환
    let output_str = result.lock().unwrap().clone();
    Ok(output_str)
}

pub fn verify_rule(file_path: &Path) {
    let mut init_code = String::new();
    let mut init_file = File::open("./z3/z3_init_vec.txt").unwrap();
    init_file.read_to_string(&mut init_code).unwrap();

    let formulas = read_formulas(file_path.to_str().unwrap());

    let mut total_count = 0;
    let mut match_count = 0;
    let mut timeout_count = 0;

    for (origin, obfuscation) in formulas {
        println!("Origin: {}, Obfuscation: {}", origin, obfuscation);

        let origin_prefix = infix_to_prefix(&origin);
        let obfuscation_prefix = infix_to_prefix(&obfuscation);

        println!("Origin prefix: {}", origin_prefix);
        println!("Obfuscation prefix: {}", obfuscation_prefix);

        let origin_z3 = convert_to_z3_format(&origin_prefix);
        let obfuscation_z3 = convert_to_z3_format(&obfuscation_prefix);

        let z3_command = format!("(assert (not (= {} {})))\n", origin_z3, obfuscation_z3);
        println!("Z3 command: {}", z3_command);
        
        // 5초 타임아웃으로 Z3 실행
        let result = run_z3_with_timeout(&init_code, &z3_command, 5);
        
        total_count += 1;
        
        match result {
            Ok(output_str) => {
                println!("Z3 output: {}", output_str);
                
                if output_str.contains("unsat") {
                    println!("✅ Formula match confirmed!");
                    match_count += 1;
                } else {
                    println!("❌ Formula does not match");
                }
            },
            Err(error_msg) => {
                println!("⚠ {}", error_msg);
                timeout_count += 1;
            }
        }
        
        println!("-----------------------------------\n");
    }

    println!("\nTotal Formulas: {}", total_count);
    println!("Matched Formulas: {}", match_count);
    println!("Timeout Formulas: {}", timeout_count);
    let valid_count = total_count - timeout_count;
    if valid_count > 0 {
        println!("Match Rate: {:.2}%", (match_count as f64 / valid_count as f64) * 100.0);
    } else {
        println!("Match Rate: N/A (All timeouts)");
    }
}

// 테스트 함수
fn main() {
    // 테스트 케이스
    let test_cases = [
        ("x|y", "(x&(~y))+y"),
        ("a&b", "a&b"),
        ("~(x&y)", "(~x)|(~y)"),
        ("(a|b)&c", "a&c|b&c")
    ];
    
    for (origin, obfuscation) in test_cases.iter() {
        let origin_prefix = infix_to_prefix(origin);
        let obfuscation_prefix = infix_to_prefix(obfuscation);
        
        println!("Origin: {}", origin);
        println!("Origin prefix: {}", origin_prefix);
        println!("Obfuscation: {}", obfuscation);
        println!("Obfuscation prefix: {}", obfuscation_prefix);
        println!("------------------------------");
    }
}