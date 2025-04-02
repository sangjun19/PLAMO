from z3 import Solver, sat, unsat # type: ignore
from miasm.ir.translators.z3_ir import TranslatorZ3
from miasm.expression.expression import *
import re
import random
from pathlib import Path

def preprocess(expr: str) -> str:  # Parse integers in the string
    expr = expr.replace("\n", " ")
    expr = re.sub(r'\b\d+\b', insertInt, expr)
    expr = re.sub(r'\b0x\w+\b', insertInt, expr)
    return expr

def insertInt(n):  # Called by preprocess to make ExprInt compatible with miasm
    m = n.group()
    return f"ExprInt({m}, size)"

size = 32

# Define symbolic variables
a = ExprId('a', size)
b = ExprId('b', size)
c = ExprId('c', size)
d = ExprId('d', size)
e = ExprId('e', size)
x = ExprId('x', size)
y = ExprId('y', size)
z = ExprId('z', size)

# Initialize Z3 solver and translator
s = Solver()
translator_z3 = TranslatorZ3()

# Get the current script's directory
script_dir = Path(__file__).parent
file_path = "./../experiment_data/output_ast_smt_solver_unique.txt"  # Update to the .txt file path

# Number of random expressions to test
n = 10  # Change this value as needed

# Read and process the file
with open(file_path, 'r', encoding='utf-8') as txtfile:
    lines = txtfile.readlines()
    data = [line.strip().split(",")[:2] for line in lines if line.strip()]  # Extract original and obfuscated only

# Randomly select n expressions
random_samples = random.sample(data, min(n, len(data)))

whole_count = 0
correct = 0

for original, obfuscated in random_samples:
    print(f"Testing expressions:")
    print(f"Original: {original}")
    print(f"Obfuscated: {obfuscated}")
    
    try:
        # Preprocess and evaluate the expressions
        miasmir_input_expr = eval(preprocess(obfuscated))
        miasmir_test_expr = eval(preprocess(original))

        s.reset()
        s.set("timeout", 10000)

        origin_expr = translator_z3.from_expr(miasmir_input_expr)
        simple_expr = translator_z3.from_expr(miasmir_test_expr)

        s.add(origin_expr != simple_expr)

        result = s.check()  # Check satisfiability

        if result == sat:
            print(f"✅ Satisfiable: {original} == {obfuscated}")
            print(s.model())  # Print the model
            correct += 1
        elif result == unsat:
            print(f"❌ Unsatisfiable: {original} != {obfuscated}")
        else:
            print(f"⚠️ Unknown result: {original} != {obfuscated}")
            print("Reason:", s.reason_unknown())  # Print reason for unknown
    except Exception as e:
        print(f"⚠️ Error while processing expressions: {e}")
    
    whole_count += 1


print(f"+correct: {correct}/{whole_count}")
print(f"accuracy: {correct/whole_count*100:.2f}%")
