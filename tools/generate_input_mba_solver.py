import os

# 출력 디렉토리가 존재하는지 확인하고 생성
output_dir = "./experiment_data/input_data"
os.makedirs(output_dir, exist_ok=True)

# 파일 경로 목록
paths = [
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.boolector.verify.64bit.before.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.simplify.txt.boolector.verify.64bit.before.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.simplify.txt.stp.verify.64bit.after.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.simplify.txt.z3.verify.64bit.after.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.stp.verify.64bit.before.simplify.txt',
    './experiment_data/MBA-Solver/pldi_dataset_linear_MBA.txt.z3.verify.64bit.before.simplify.txt'
]

# 출력 파일 경로
output_file = "./experiment_data/input_data/input_mba_solver.txt"
unique_expr = set()
cnt = 0

# 파일 내용을 읽어 출력 파일에 작성
with open(output_file, 'w') as outfile:
    for path in paths:
        try:
            with open(path, 'r') as infile:
                for line in infile:
                    # 줄을 쉼표로 구분된 부분으로 나눔
                    parts = line.strip().replace(" ", "").split(',')
                    
                    # 줄이 정확히 두 개의 값으로 구성되어 있는지 확인
                    if len(parts) >= 2:                        
                        unique_expr.add(parts[1])
                        cnt += 1
                    else:
                        print(f"파일 {path}의 줄을 건너뜁니다: {line} (잘못된 형식)")
        except FileNotFoundError:
            print(f"파일을 찾을 수 없습니다: {path}")
        except Exception as e:
            print(f"파일 {path} 읽기 오류: {e}")

    # 고유한 표현식을 출력 파일에 작성
    for expr in unique_expr:
        outfile.write(expr + '\n')

print(f"total : {cnt}")
print(f"unique : {len(unique_expr)}")
