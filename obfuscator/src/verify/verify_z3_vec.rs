use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

fn read_formulas(file_path: &str) -> Vec<(String, String, i32)> {
    let mut formulas = Vec::new();
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let origin = parts[0].trim().to_string();
            let obfuscation = parts[1].trim().to_string();
            let _id = parts[2].trim().parse::<i32>().unwrap();
            formulas.push((origin, obfuscation, _id));
        }
    }
    formulas
}

pub fn verify_vec(file_path: &Path) {
    let mut init_code = String::new();
    let mut init_file = File::open("./z3/z3_init_vec.txt").unwrap();
    init_file.read_to_string(&mut init_code).unwrap();

    let formulas = read_formulas(file_path.to_str().unwrap());

    let mut total_count = 0;
    let mut match_count = 0;

    for (origin, obfuscation, _id) in formulas {
        let mut child = match Command::new("./z3/z3.exe")
            .arg("-in") // Z3의 REPL 모드로 실행
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(child) => child,
            Err(e) => panic!("Failed to spawn z3 process: {}", e),
        };

        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(init_code.as_bytes()).unwrap(); // 초기화 명령어 보내기

        // 비트 벡터로 선언된 변수 x, y, z를 사용하도록 명제 수정
        let origin_modified = origin.replace("+", "bvadd").replace("-", "bvsub").replace("*", "bvmul").replace("&", "bvand").replace("|", "bvor").replace("~", "bvnot").replace("^", "bvxor");
        let obfuscation_modified = obfuscation.replace("+", "bvadd").replace("-", "bvsub").replace("*", "bvmul").replace("&", "bvand").replace("|", "bvor").replace("~", "bvnot").replace("^", "bvxor");

        stdin.write_all(format!("(assert (not (= (bv2int {}) (bv2int {}))))\n", origin_modified, obfuscation_modified).as_bytes()).unwrap();
        stdin.write_all(b"(check-sat)\n").unwrap(); // SAT 문제 해결
        stdin.write_all(b"(exit)\n").unwrap(); // Z3 종료

        let output = child.wait_with_output().unwrap();

        let output_str = String::from_utf8_lossy(&output.stdout);
        total_count += 1;

        if output_str.contains("unsat") {
            // println!("Origin: {}, Obfuscation: {} : ", origin, obfuscation);
            // println!("{} Correct!", "\u{2705}"); // 
            match_count += 1;
        } else {
            // println!("Origin: {}, Obfuscation: {} : ", origin, obfuscation);
            // println!("{} Incorrect!", "\u{274C}"); // 
        }
    }

    println!("\nTotal Formulas: {}", total_count);
    println!("Matched Formulas: {}", match_count);
    println!("Match Rate: {:.2}%", (match_count as f64 / total_count as f64) * 100.0);
}
