use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn remove_duplicates(input_file: &str, output_file: &str) {
    let mut unique_lines: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut max_len = 0;
    let mut variety: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Open the input file and read lines
    let file = File::open(input_file).expect("Failed to open input file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            // Split the line into original, obfuscated, size
            let parts: Vec<&str> = line.replace(" ", "").split(',').collect();
            
            if parts.len() < 3 {
                continue;
            }
            
            let original = parts[0].to_string();
            let obfuscated = parts[1].to_string();
            
            // Calculate the new size based on the obfuscated field
            let new_size = obfuscated.len();
            
            // Create a new line with the updated size
            let new_line = format!("{},{},{}", original, obfuscated, new_size);
            
            max_len = max_len.max(new_size);
            
            // Add the new line to the set (duplicates are automatically removed)
            unique_lines.insert(new_line.clone());
            variety.insert(original);
        }
    }

    println!("Max length: {}", max_len);
    println!("Unique lines: {}", unique_lines.len());
    println!("Variety: {}", variety.len());

    // Write the unique lines back to the output file
    let output = File::create(output_file).expect("Failed to create output file");
    let mut writer = BufWriter::new(output);
    for line in unique_lines {
        writeln!(writer, "{}", line).expect("Failed to write line");
    }
}

fn main() {
    let input_file = "./experiment_data/MBA_Solver_result/output_ast_mba_smt_delight_infix.txt";
    let output_file = "./experiment_data/MBA_Solver_result/output_ast_mba_smt_delight_unique.txt";
    remove_duplicates(input_file, output_file);
}
