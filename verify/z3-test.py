import csv
from pathlib import Path
from z3 import BitVec, Solver, unsat

# Z3용 비트벡터 변수 정의 (여기서는 32비트로 설정)
x = BitVec('x', 32)
y = BitVec('y', 32)
z = BitVec('z', 32)
a = BitVec('a', 32)
b = BitVec('b', 32)
c = BitVec('c', 32)

# eval 시 사용할 환경 변수
env = {'x': x, 'y': y, 'z': z, 'a': a, 'b': b, 'c': c}

# 현재 스크립트의 디렉토리 가져오기
script_dir = Path(__file__).parent
file_path = script_dir / "../experiment_data/test.csv"

def convert_to_z3(expr: str) -> str:
    """
    Rust에서 생성된 비트 연산자를 Z3 Python이 해석할 수 있도록 변환.
    예: '!' -> '~'
    """
    expr = expr.replace("!", "~")
    expr = expr.replace("&", " & ")
    expr = expr.replace("|", " | ")
    expr = expr.replace("^", " ^ ")
    return expr

with open(file_path, newline='', encoding='utf-8') as csvfile:
    reader = csv.reader(csvfile)
    next(reader)  # 헤더 스킵

    for row in reader:
        obfuscated_expr = row[0]  # 난독화된 식
        original_expr = row[1]    # 원본 식

        try:
            # Z3에서 사용할 수 있도록 식 변환
            obf_expr_str = convert_to_z3(obfuscated_expr)
            orig_expr_str = convert_to_z3(original_expr)

            # eval을 통해 문자열 식을 Z3 표현식으로 변환
            obf_expr = eval(obf_expr_str, {}, env)
            orig_expr = eval(orig_expr_str, {}, env)

            # 두 식이 서로 다를 수 있는지 확인 (즉, 모순이 없는지 검증)
            solver = Solver()
            solver.add(obf_expr != orig_expr)
            if solver.check() == unsat:
                print(f"✅ 변환 검증 성공: {original_expr} == {obfuscated_expr}")
            else:
                print(f"❌ 변환 검증 실패: {original_expr} != {obfuscated_expr}")
        except Exception as e:
            print(f"⚠️ 검증 오류 발생: {e}")
