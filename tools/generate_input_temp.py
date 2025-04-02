def remove_duplicates(input_file, output_file):
    # Use a set to store unique lines
    unique_lines = set()
    global variation
    little_size = 0
    big_size = 100000
    limit_num = 1
    global cnt, num, size_max

    # Open the input file and read lines
    with open(input_file, 'r') as file:
        for line in file:
            cnt += 1
            # Split the line into original, obfuscated, size
            parts = line.strip().replace(" ", "").split(',')
            
            original, obfuscated, _ = parts 
            
            # Calculate the new size based on the obfuscated field
            new_size = len(obfuscated)
            
            
            if little_size > new_size or new_size > big_size:
                continue
            
            if(cnt % limit_num > 0):
                continue
            
            size_max = max(size_max, new_size)
            
            num += 1
            # Create a new line with the updated size
            new_line = f"{original},{obfuscated}"
            
            # Add the new line to the set (duplicates are automatically removed)
            unique_lines.add(new_line)   
            variation.add(original)                     

    # Write the unique lines back to the output file
    with open(output_file, 'w') as file:
        for line in unique_lines:
            file.write(line + '\n')

variation = set()
cnt = 0
num = 0
size_max = 0
input_file = "./experiment_data/output_ast_smt_delight_unique.txt"  # Replace with your input file name
output_file = "./experiment_data/temp_data/input_temp.txt"  # Replace with your desired output file name
remove_duplicates(input_file, output_file)
print(cnt)
print(num)
print(len(variation))
print(size_max)
