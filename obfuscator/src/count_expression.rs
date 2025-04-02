use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Counts the total number of unique obfuscated expressions and the total number of expressions.
///
/// # Arguments
/// * `file_path` - Path to the output file containing obfuscated expressions.
///
/// # Returns
/// A tuple where:
/// - The first value is the total number of unique expressions.
/// - The second value is the total number of expressions in the file (including duplicates).
pub fn count_expressions(file_path: &Path) -> (usize, usize) {
    let mut unique_expressions = HashSet::new(); // To store unique obfuscated expressions
    let mut total_lines = 0; // To count all lines in the file

    // Open the file for reading
    let file = File::open(file_path).expect("Failed to open the file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read a line");

        total_lines += 1; // Count every line in the file

        // Split the line into parts: original_expression, obfuscated_expression, tree_size, iteration
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        // if parts.len() < 4 {
        //     eprintln!("Skipping invalid line: {}", line);
        //     continue; // Skip invalid lines
        // }

        // Extract the obfuscated expression (2nd column)
        let obfuscated_expression = parts[1].to_string();

        // println!("Adding to HashSet: {}", obfuscated_expression); // Debugging output

        // Insert the obfuscated expression into the set
        unique_expressions.insert(obfuscated_expression);
    }

    // The size of the set represents the number of unique expressions
    let unique_count = unique_expressions.len();

    (unique_count, total_lines)
}

/// Prints the total number of unique expressions and all expressions.
pub fn print_expression_stats(file_path: &Path) {
    let (unique_count, total_lines) = count_expressions(file_path);

    println!("Total unique obfuscated expressions: {}", unique_count);
    println!("Total duplicate obfuscated expressions: {}", total_lines - unique_count);
    println!("Total obfuscated expressions (including duplicates): {}", total_lines);
}
