import sys
import os
from contextlib import contextmanager
from miasm.expression.expression import *
from miasm.expression.simplifications import expr_simp
from miasm.core.locationdb import LocationDB

# Prefix expression string to AST
def prefix_to_ast(prefix_str):
    prefix_expr = prefix_str.replace("(", " ( ").replace(")", " ) ").split()
    stack = []
    i = len(prefix_expr) - 1

    while i >= 0:
        token = prefix_expr[i]

        if not isOperator(token) and token not in ["(", ")"]:
            stack.append(ExprId(token, 64))  # Operand
        elif isOperator(token):
            if token == "~":  # Unary operator
                operand = stack.pop()
                stack.append(ExprOp(token, operand))
            else:  # Binary operator
                left = stack.pop()
                right = stack.pop()
                stack.append(ExprOp(token, left, right))
        i -= 1

    return stack.pop()

# AST to infix conversion
def ast_to_infix(expr):
    if isinstance(expr, ExprId):
        return str(expr)
    elif isinstance(expr, ExprOp):
        if len(expr.args) == 1: 
            return f"( {expr.op} {ast_to_infix(expr.args[0])} )"
        elif len(expr.args) == 2: 
            left = ast_to_infix(expr.args[0])
            right = ast_to_infix(expr.args[1])
            return f"( {left} {expr.op} {right} )"
    return str(expr)

def isOperator(token):
    return token in ["+", "-", "*", "/", "&", "|", "^", "~", "<<", ">>", "%"]

@contextmanager
def suppress_stdout():
    original_stdout = sys.stdout
    sys.stdout = open(os.devnull, 'w')
    try:
        yield
    finally:
        sys.stdout = original_stdout

# File paths
input_file_path_str = "./experiment_data/MBA_Solver_result/output_ast_mba_smt_delight.txt"
output_file_path_str = "./experiment_data/MBA_Solver_result/output_ast_mba_smt_delight_infix.txt"

# Open files
input_file = open(input_file_path_str, "r")
output_file = open(output_file_path_str, "w")

with suppress_stdout():
    problematic_expressions = []

    for line in input_file:
        # Split the line into original, obfuscated, and size
        parts = line.strip().split(", ")
        if len(parts) != 3:
            continue  # Skip invalid lines
        
        original_prefix = parts[0]
        obfuscated_prefix = parts[1]
        
        try:
            # Convert obfuscated expression from prefix to infix
            original_ast = prefix_to_ast(original_prefix)
            original_infix = ast_to_infix(original_ast)
            obfuscated_ast = prefix_to_ast(obfuscated_prefix)
            obfuscated_infix = ast_to_infix(obfuscated_ast)
            
            # Write the result to the output file
            output_file.write(f"{original_infix}, {obfuscated_infix}, {parts[2]}\n")
        except Exception as e:
            # 문제가 있는 식을 따로 저장
            problematic_expressions.append((original_prefix, obfuscated_prefix, str(e)))
            print(f"Error!")

    # 문제가 있는 식을 파일에 저장
    if len(problematic_expressions) > 0:
        with open("./experiment_data/problematic_expressions.txt", "w") as problem_file:
            for original, obfuscated, error in problematic_expressions:
                problem_file.write(f"{original}, {obfuscated}, {error}\n")        

# Close files
input_file.close()
output_file.close()

