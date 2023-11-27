# MIPS Compiler

## Grammer
> Reference: https://go.dev/ref/spec#SimpleStmt
> Reference: https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.html

### Token
```
identifier = letter { letter | unicode_digit }

int_lit        = "0" | ( "1" … "9" )
string_lit             = `"` { unicode_value | byte_value } `"`

log_op  = "||" | "&&"
rel_op     = "==" | "!=" | "<" | "<=" | ">" | ">="
add_op     = "+" | "-" | "|" | "^"
mul_op     = "*" | "/" | "%" | "<<" | ">>" | "&"

unary_op   = "+" | "-" | "!" | "~" | "*" | "&"

pointer = "*"

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
PointerType -> Type pointer

Block -> lbrace StatementList rbrace
StatementList -> Statement semicolon StatementList
StatementList -> ''

VarDecl -> Type identifier

FunctionDecl -> Type identifier Parameters Block
Parameters -> lparen ParameterList rparen
ParameterList -> ParameterDecl comma ParameterList
ParameterList -> ''
ParameterDecl -> Type identifier

Operand -> int_lit
Operand -> string_lit
Operand -> identifier
Operand -> lparen Expression rparen

PrimaryExpr -> Operand
PrimaryExpr -> PrimaryExpr Index
PrimaryExpr -> PrimaryExpr Arguments

Index -> lbracket Expression rbracket
Arguments -> lparen ExpressionList rparen
ExpressionList -> Expression comma ExpressionList
ExpressionList -> ''

Expression -> LogicalExpr
LogicalExpr -> RelationalExpr
LogicalExpr -> LogicalExpr log_op RelationalExpr
RelationalExpr -> AdditiveExpr
RelationalExpr -> RelationalExpr rel_op AdditiveExpr
AdditiveExpr -> MultiplicativeExpr
AdditiveExpr -> AdditiveExpr add_op MultiplicativeExpr
MultiplicativeExpr -> UnaryExpr
MultiplicativeExpr -> MultiplicativeExpr mul_op UnaryExpr
UnaryExpr -> PrimaryExpr 
UnaryExpr -> unary_op UnaryExpr

Statement -> Assignment
Statement -> VarDecl
Statement -> ReturnStmt
Statement -> BreakStmt
Statement -> ContinueStmt
Statement -> IfStmt
Statement -> WhileStmt

Assignment -> ExpressionList assign_op ExpressionList
IfStmt -> if lparen Expression rparen Block
WhileStmt -> while lparen Expression rparen Block
ReturnStmt -> return Expression
BreakStmt -> break
ContinueStmt -> continue
```

## Operator priority

  Precedence    Operator
      4             *  /  %  <<  >>  &
      3             +  -  |  ^
      2             ==  !=  <  <=  >  >=
      1             && ||
