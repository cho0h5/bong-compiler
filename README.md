# MIPS Compiler

## Grammer
> Reference: https://go.dev/ref/spec#SimpleStmt
> Reference: https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.html

### Token
```
identifier = letter { letter | unicode_digit } .

int_lit        = "0" | ( "1" … "9" ) .
string_lit             = `"` { unicode_value | byte_value } `"` .

log_op  = "||" | "&&" .
rel_op     = "==" | "!=" | "<" | "<=" | ">" | ">=" .
add_op     = "+" | "-" | "|" | "^" .
mul_op     = "*" | "/" | "%" | "<<" | ">>" | "&" .

unary_op   = "+" | "-" | "!" | "~" | "*" | "&" .

lparen = "("
rparen = ")"
lbrace = "{"
rbrace = "}"
lbracket = "["
rbracket = "]"
semicolon = ";"
comma = ","
assign_op = "=" .

if = "if"
while = "while"
return = "return"
break = "break"
continue = "continue"
```
### CFG
```
Type      = TypeName [ TypeArgs ] | TypeLit | lparen Type rparen .
TypeName  = identifier  .
TypeArgs  = lbracket TypeList rbracket .
TypeList  = Type { comma Type } .
TypeLit   = ArrayType | StructType | PointerType | FunctionType | InterfaceType |
            SliceType | MapType | ChannelType .

Block = lbrace StatementList rbrace .
StatementList = { Statement semicolon } .

Declaration   = VarDecl .
TopLevelDecl  = FunctionDecl .

IdentifierList = identifier { comma identifier } .
ExpressionList = Expression { comma Expression } .

VarDecl     = Type identifier [ "=" ExpressionList ] .

FunctionDecl = Type identifier Parameters Block .
Parameters     = "(" [ ParameterList ] ")" .
ParameterList  = ParameterDecl { "," ParameterDecl } .
ParameterDecl  = Type identifier .

Operand     = Literal | identifier [ TypeArgs ] | "(" Expression ")" .
Literal     = int_lit | string_lit .

PrimaryExpr =
	Operand |
	PrimaryExpr Index |
	PrimaryExpr Arguments .

Index          = lbracket Expression rbracket .
Arguments      = lparen [ ExpressionList ] rparen .
ExpressionList = Expression { comma Expression } .

Expression = LogicalExpr .

LogicalExpr = RelationalExpr | LogicalExpr log_op RelationalExpr
RelationalExpr = AdditiveExpr | RelationalExpr rel_op AdditiveExpr
AdditiveExpr = MultiplicativeExpr | AdditiveExpr add_op MultiplicativeExpr
MultiplicativeExpr = UnaryExpr | MultiplicativeExpr mul_op UnaryExpr
UnaryExpr  = PrimaryExpr | unary_op UnaryExpr | lparen Expression rparen .

Statement =
    Assignment |
	Declaration |
	ReturnStmt | BreakStmt | ContinueStmt |
	Block | IfStmt |  WhileStmt .

Assignment = ExpressionList assign_op ExpressionList .
IfStmt = if Expression Block .
WhileStmt = while lparen Expression rparen Block .
ReturnStmt = return Expression .
BreakStmt = break .
ContinueStmt = continue .
```

## Operator priority

  Precedence    Operator
      4             *  /  %  <<  >>  &
      3             +  -  |  ^
      2             ==  !=  <  <=  >  >=
      1             && ||
