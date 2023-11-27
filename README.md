# MIPS Compiler

## Grammer
```
<translation-unit> ::= {<function-definition>}*

<function-definition> ::= <type_specifier> <declarator> <compound-statement>

<type-specifier> ::= void
                   | int

<specifier-qualifier> ::= <type-specifier>

<declarator> ::= {<pointer>}? <direct-declarator>

<pointer> ::= * {<pointer>}?

<direct-declarator> ::= <identifier>
                      | ( <declarator> )
                      | <direct-declarator> [ {<constant-expression>}? ]
                      | <direct-declarator> ( <parameter-type-list> )
                      | <direct-declarator> ( {<identifier>}* )

<constant-expression> ::= <logical-or-expression>

<logical-or-expression> ::= <logical-and-expression>
                          | <logical-or-expression> || <logical-and-expression>

<logical-and-expression> ::= <inclusive-or-expression>
                           | <logical-and-expression> && <inclusive-or-expression>

<inclusive-or-expression> ::= <exclusive-or-expression>
                            | <inclusive-or-expression> | <exclusive-or-expression>

<exclusive-or-expression> ::= <and-expression>
                            | <exclusive-or-expression> ^ <and-expression>

<and-expression> ::= <equality-expression>
                   | <and-expression> & <equality-expression>

<equality-expression> ::= <relational-expression>
                        | <equality-expression> == <relational-expression>
                        | <equality-expression> != <relational-expression>

<relational-expression> ::= <shift-expression>
                          | <relational-expression> < <shift-expression>
                          | <relational-expression> > <shift-expression>
                          | <relational-expression> <= <shift-expression>
                          | <relational-expression> >= <shift-expression>

<shift-expression> ::= <additive-expression>
                     | <shift-expression> << <additive-expression>
                     | <shift-expression> >> <additive-expression>

<additive-expression> ::= <multiplicative-expression>
                        | <additive-expression> + <multiplicative-expression>
                        | <additive-expression> - <multiplicative-expression>

<multiplicative-expression> ::= <unary-expression>
                              | <multiplicative-expression> * <unary-expression>
                              | <multiplicative-expression> / <unary-expression>
                              | <multiplicative-expression> % <unary-expression>

<unary-expression> ::= <postfix-expression>
                     | <unary-operator> <unary-expression>
                     | sizeof <unary-expression>

<postfix-expression> ::= <primary-expression>
                       | <postfix-expression> [ <expression> ]
                       | <postfix-expression> ( {<assignment-expression>}* )

<primary-expression> ::= <identifier>
                       | <constant>
                       | <string>
                       | ( <expression> )

<constant> ::= <integer-constant>
             | <character-constant>

<expression> ::= <assignment-expression>
               | <expression> , <assignment-expression>

<assignment-expression> ::= <unary-expression> <assignment-operator> <assignment-expression>

<assignment-operator> ::= =

<unary-operator> ::= &
                   | *
                   | +
                   | -
                   | ~
                   | !

<parameter-type-list> ::= <parameter-list>
                        | <parameter-list> , ...

<parameter-list> ::= <parameter-declaration>
                   | <parameter-list> , <parameter-declaration>

<parameter-declaration> ::= <type-specifier> <declarator>
                          | <type-specifier>

<declaration> ::=  <type-specifier> {<init-declarator>}* ;

<init-declarator> ::= <declarator>
                    | <declarator> = <initializer>

<initializer> ::= <assignment-expression>
                | { <initializer-list> }
                | { <initializer-list> , }

<initializer-list> ::= <initializer>
                     | <initializer-list> , <initializer>

<compound-statement> ::= { {<declaration>}* {<statement>}* }

<statement> ::= <expression-statement>
              | <if-statement>
              | <while-statement>

<expression-statement> ::= {<expression>}? ;

<if-statement> ::= if ( <expression> ) <statement>

<while-statement> ::= while ( <expression> ) <statement>
```
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
