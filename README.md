# MIPS Compiler

## Grammer
```
FunctionDecl = Type identifier "(" ParameterList ")" Block
ParameterList = ParameterDecl { "," ParameterDecl }
ParameterDecl = Type identifier
Type = "int" | "void"
Block = "{" StatementList "}"
StatementList = { Statement ";" }

Statement =
  Declaration |
  SimpleStmt |
  IfStmt |
  WhileStmt
```
> Reference: https://go.dev/ref/spec#SimpleStmt
> Reference: https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.html
