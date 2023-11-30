# bong-lang Compiler

## Grammer

### Token
```
identifier = letter { letter | unicode_digit }

int_lit = "0" | ( "1" … "9" )
string_lit = `"` { unicode_value | byte_value } `"`

log_op  = "||" | "&&"
rel_op     = "==" | "!=" | "<" | "<=" | ">" | ">="
add_op     = "|" | "^"
mul_op     = "/" | "%" | "<<" | ">>"

unary_op   = "!" | "~"

star = "*"
and = "&"
add_minus = "+" | "-"

lparen = "("
rparen = ")"
lbrace = "{"
rbrace = "}"
lbracket = "["
rbracket = "]"
semicolon = ";"
comma = ","
assign_op = "="

if = "if"
while = "while"
return = "return"
break = "break"
continue = "continue"

int = "int"
void = "void"
```
### CFG
```
Program' -> Program
Program -> FunctionDecl Program
Program -> ''

Type -> int
Type -> void
Type -> ArrayType
Type -> PointerType

ArrayType -> Type lbracket int_lit rbracket
PointerType -> Type star

Block -> lbrace StatementList rbrace
StatementList -> Statement semicolon StatementList
StatementList -> Statement
StatementList -> ''

VarDecl -> Type identifier

FunctionDecl -> Type identifier Parameters Block
Parameters -> lparen ParameterList rparen
ParameterList -> ParameterDecl comma ParameterList
ParameterList -> ParameterDecl
ParameterList -> ''
ParameterDecl -> Type identifier

Operand -> int_lit
Operand -> string_lit
Operand -> identifier
Operand -> lparen Expression rparen

PrimaryExpr -> PrimaryExpr Index
PrimaryExpr -> PrimaryExpr Arguments
PrimaryExpr -> Operand

Index -> lbracket Expression rbracket
Arguments -> lparen ExpressionList rparen
ExpressionList -> Expression comma ExpressionList
ExpressionList -> Expression
ExpressionList -> ''

Expression -> LogicalExpr
LogicalExpr -> LogicalExpr log_op RelationalExpr
LogicalExpr -> RelationalExpr
RelationalExpr -> RelationalExpr rel_op AdditiveExpr
RelationalExpr -> AdditiveExpr
AdditiveExpr -> AdditiveExpr add_op MultiplicativeExpr
AdditiveExpr -> AdditiveExpr add_minus MultiplicativeExpr
AdditiveExpr -> MultiplicativeExpr
MultiplicativeExpr -> MultiplicativeExpr mul_op UnaryExpr
MultiplicativeExpr -> MultiplicativeExpr star UnaryExpr
MultiplicativeExpr -> MultiplicativeExpr and UnaryExpr
MultiplicativeExpr -> UnaryExpr
UnaryExpr -> unary_op UnaryExpr
UnaryExpr -> star UnaryExpr
UnaryExpr -> and UnaryExpr
UnaryExpr -> add_minus UnaryExpr
UnaryExpr -> PrimaryExpr 

Statement -> Assignment
Statement -> VarDecl
Statement -> ReturnStmt
Statement -> BreakStmt
Statement -> ContinueStmt
Statement -> IfStmt
Statement -> WhileStmt
Statement -> Expression

Assignment -> Expression assign_op Expression
IfStmt -> if lparen Expression rparen Block
WhileStmt -> while lparen Expression rparen Block
ReturnStmt -> return Expression
BreakStmt -> break
ContinueStmt -> continue
```

## Operator priority

| Precedence | Operator |
| --- | --- |
| 4 | `*` `/` `%` `<<` `>>` `&` |
| 3 | `+` `-` `\|` `^` |
| 2 | `==` `!=` `<` `<=` `>` `>=` |
| 1 | `&&` `\|\|` |

# Reference
## CFG, BNF
- https://go.dev/ref/spec#SimpleStmt  
- https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.html

## Symbol Table
- https://web.cs.wpi.edu/~kal/courses/compilers/module5/myst.html

## Code Generation
- https://web.cs.wpi.edu/~kal/courses/cs4533/module6/mycg.html

## Code
- https://github.com/cho0h5/simple-java-parser
- https://github.com/cho0h5/simple-parser
