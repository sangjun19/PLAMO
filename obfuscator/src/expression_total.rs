use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

/// <<<  여기에만 경로를 맞춰 주세요 >>>
const FILE_PATH: &str = "./../data/result_data/mba_solver_delight_1.txt";

/// 한 번에 모든 통계 출력
pub fn print_total() -> io::Result<()> {
    let (total_rows, unique_inputs, orig_total, obfus_total) = collect_stats()?;

    println!("Original expressions        : {}", unique_inputs);
    println!("Obfuscated expressions      : {}", total_rows);
    println!("Average original length     : {:.2}", orig_total  as f64 / total_rows as f64);
    println!("Average obfuscated length   : {:.2}", obfus_total as f64 / total_rows as f64);
    Ok(())
}

/// (전체 줄 수, 중복 제거한 입력식 수, 원본 길이 합, 난독화 길이 합) 반환
fn collect_stats() -> io::Result<(usize, usize, usize, usize)> {
    let file = File::open(FILE_PATH)?;
    let reader = io::BufReader::new(file);

    let mut total_rows   = 0;
    let mut orig_total   = 0;
    let mut obfus_total  = 0;
    let mut uniq_set: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        let mut parts = line.splitn(3, ',').map(|s| s.trim());
        let orig = parts.next();
        let _mba = parts.next();
        let len  = parts.next();

        if let (Some(o), Some(l)) = (orig, len) {
            if let Ok(ob_len) = l.parse::<usize>() {
                total_rows  += 1;
                orig_total  += o.chars().count();
                obfus_total += ob_len;
                uniq_set.insert(o.to_string());          // 중복 제거
            } else {
                eprintln!("⚠️  잘못된 길이 값: {}", l);
            }
        } else {
            eprintln!("⚠️  필드가 부족한 줄: {}", line);
        }
    }

    if total_rows == 0 {
        Err(io::Error::new(io::ErrorKind::Other, "no valid records found"))
    } else {
        Ok((total_rows, uniq_set.len(), orig_total, obfus_total))
    }
}

fn main() -> std::io::Result<()> {
    print_total()
}
