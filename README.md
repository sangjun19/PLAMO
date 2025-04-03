# PLAMO
Chungnam National University  
Programming Language and System Lab. (PLAS Lab.)  
ast(Abstract Syntax Tree) based MBA(Mixed Boolean Arithmetic) Obfuscator written in Rust

- Outfile format
  - txt(obfuscator, original, size)
```
obfuscator = obfuscated expression
original = original expression
```
- Outfile format(final)
  - txt(ground truth, obfuscation expression, category, AST size, MBA alternation)
```
ground truth = original expression, source
obfuscation expression = generated MBA expression, MBA obfuscation expression
category = MBA category (linear, poly, non-poly, ...)
AST size = generated MBA expression Abstract Syntzx Tree Size, ex) x + y = 3, ((x + y) << 2) = 9(?)
MBA Alternation = Number of MBA Alternation operator
```
- Inputfile,Ouputfile is placed in plas-nas