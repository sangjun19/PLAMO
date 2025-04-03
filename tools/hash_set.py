def remove_duplicates(input_file, output_file):
    # Use a set to store unique lines
    unique_lines = set()    
    max_len = 0
    variety = set()

    # Open the input file and read lines
    with open(input_file, 'r') as file:
        for line in file:
            # Split the line into original, obfuscated, size
            parts = line.strip().replace(" ", "").split(',')
            
            original, obfuscated, _ = parts
            
            # Calculate the new size based on the obfuscated field
            new_size = len(obfuscated)
            
            # if new_size < 40:
            #     continue
            
            # Create a new line with the updated size
            new_line = f"{original},{obfuscated},{new_size}"
            
            max_len = max(max_len, new_size)
            
            # Add the new line to the set (duplicates are automatically removed)
            unique_lines.add(new_line)
            variety.add(original)
            
    print(f"Max length: {max_len}")
    print(f"Unique lines: {len(unique_lines)}")
    print(f"Variety: {len(variety)}")

    # Write the unique lines back to the output file
    with open(output_file, 'w') as file:
        for line in unique_lines:
            file.write(line + '\n')

input_file = "./data/output_data/output_ast_mba_smt_delight_infix.txt"  # Replace with your input file name
output_file = "./data/output_data/output_ast_mba_smt_delight_unique.txt"  # Replace with your desired output file name
remove_duplicates(input_file, output_file)
