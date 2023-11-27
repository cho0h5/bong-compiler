# MIPS Compiler

## Grammer
> Reference: https://go.dev/ref/spec#SimpleStmt
> Reference: https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.html

```
identifier = letter { letter | unicode_digit } .

int_lit        = "0" | ( "1" … "9" ) .
string_lit             = `"` { unicode_value | byte_value } `"` .

Type      = TypeName [ TypeArgs ] | TypeLit | "(" Type ")" .
TypeName  = identifier  .
TypeArgs  = "[" TypeList "]" .
TypeList  = Type { "," Type } .
TypeLit   = ArrayType | StructType | PointerType | FunctionType | InterfaceType |
            SliceType | MapType | ChannelType .

Block = "{" StatementList "}" .
StatementList = { Statement ";" } .

Declaration   = VarDecl .
TopLevelDecl  = FunctionDecl .

IdentifierList = identifier { "," identifier } .
ExpressionList = Expression { "," Expression } .

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

Index          = "[" Expression [ "," ] "]" .
Arguments      = "(" [ ExpressionList ] ")" .
ExpressionList = Expression { "," Expression } .

Expression = LogicalExpr .

LogicalExpr = RelationalExpr | LogicalExpr log_op RelationalExpr
RelationalExpr = AdditiveExpr | RelationalExpr rel_op AdditiveExpr
AdditiveExpr = MultiplicativeExpr | AdditiveExpr add_op MultiplicativeExpr
MultiplicativeExpr = UnaryExpr | MultiplicativeExpr mul_op UnaryExpr
UnaryExpr  = PrimaryExpr | unary_op UnaryExpr | "(" Expression ")" .

log_op  = "||" | "&&" .
rel_op     = "==" | "!=" | "<" | "<=" | ">" | ">=" .
add_op     = "+" | "-" | "|" | "^" .
mul_op     = "*" | "/" | "%" | "<<" | ">>" | "&" .

unary_op   = "+" | "-" | "!" | "~" | "*" | "&" .

#	Precedence    Operator
#	    4             *  /  %  <<  >>  &
#	    3             +  -  |  ^
#	    2             ==  !=  <  <=  >  >=
#	    1             && ||

Statement =
    Assignment |
	Declaration | SimpleStmt |
	ReturnStmt | BreakStmt | ContinueStmt |
	Block | IfStmt |  WhileStmt .

Assignment = ExpressionList assign_op ExpressionList .
assign_op = [ add_op | mul_op ] "=" .
IfStmt = "if" [ SimpleStmt ";" ] Expression Block [ "else" ( IfStmt | Block ) ] .
WhileStmt = "while" "(" Condition ")" Block .
Condition = Expression .
ReturnStmt = "return" [ ExpressionList ] .
BreakStmt = "break" .
ContinueStmt = "continue" .
```
