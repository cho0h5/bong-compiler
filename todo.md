// Program' -> Program
// Program -> FunctionDecl Program
// Program -> ''

/ Type -> int
/ Type -> void
/ Type -> ArrayType
/ Type -> PointerType

/ ArrayType -> Type lbracket int_lit rbracket
/ PointerType -> Type star

// Block -> lbrace StatementList rbrace
// StatementList -> Statement semicolon StatementList
// StatementList -> Statement
// StatementList -> ''

// VarDecl -> Type identifier

// FunctionDecl -> Type identifier Parameters Block
// Parameters -> lparen ParameterList rparen
// ParameterList -> ParameterDecl comma ParameterList
// ParameterList -> ParameterDecl
// ParameterList -> ''
// ParameterDecl -> Type identifier

// Operand -> int_lit
Operand -> string_lit
// Operand -> identifier
// Operand -> lparen Expression rparen

PrimaryExpr -> PrimaryExpr Index
// PrimaryExpr -> PrimaryExpr Arguments
// PrimaryExpr -> Operand

Index -> lbracket Expression rbracket
// Arguments -> lparen ExpressionList rparen
// ExpressionList -> Expression comma ExpressionList
// ExpressionList -> Expression
// ExpressionList -> ''

// Expression -> LogicalExpr
/ LogicalExpr -> LogicalExpr log_op RelationalExpr
/ LogicalExpr -> RelationalExpr
RelationalExpr -> RelationalExpr rel_op AdditiveExpr
RelationalExpr -> AdditiveExpr
// AdditiveExpr -> AdditiveExpr add_op MultiplicativeExpr
// AdditiveExpr -> AdditiveExpr add_minus MultiplicativeExpr
// AdditiveExpr -> MultiplicativeExpr
MultiplicativeExpr -> MultiplicativeExpr mul_op UnaryExpr
MultiplicativeExpr -> MultiplicativeExpr star UnaryExpr
MultiplicativeExpr -> MultiplicativeExpr and UnaryExpr
MultiplicativeExpr -> UnaryExpr
UnaryExpr -> unary_op UnaryExpr
UnaryExpr -> star UnaryExpr
UnaryExpr -> and UnaryExpr
UnaryExpr -> add_minus UnaryExpr
UnaryExpr -> PrimaryExpr 

// Statement -> Assignment
// Statement -> VarDecl
// Statement -> ReturnStmt
// Statement -> BreakStmt
// Statement -> ContinueStmt
// Statement -> IfStmt
// Statement -> WhileStmt
// Statement -> Expression

// Assignment -> Expression assign_op Expression
// IfStmt -> if lparen Expression rparen Block
// WhileStmt -> while lparen Expression rparen Block
// ReturnStmt -> return Expression
// BreakStmt -> break
// ContinueStmt -> continue
