// 이 파일은 parsing을 위해 필요한 정보인 CFG와 parsing table의 정보가
// hard coding되어있습니다.

use crate::token_reader::Token;
use crate::token_reader::Token::*;
use std::collections::HashMap;
use TableElement::*;

// parsing table의 각 rule을 나타내는 enum입니다.
#[derive(Debug, PartialEq)]
pub enum TableElement {
    Shift(usize),
    Reduce(usize),
    Goto(usize),
    Accepted,
}

// reduction table의 각 rule을 나타내는 enum입니다.
// left는 CFG의 좌항의 non-terminal을 나타내며,
// right는 CFG 우항의 non-terminal, termianl의 수를 나타냅니다.
// VDECL -> vtype id semi
//     => Reduction { left: VDECL, right: 3 }
#[derive(Clone, Copy)]
pub struct Reduction {
    pub left: Token,
    pub right: usize,
}

impl Reduction {
    // Reduction struct를 생성하는 함수입니다.
    fn from(left: Token, right: usize) -> Reduction {
        Reduction {
            left: left,
            right: right,
        }
    }
}

// 총 39개의 reduction rule 정보를 return합니다.
pub fn get_reduction_table() -> Vec<Reduction> {
    let mut table = vec![];

    table.push(Reduction::from(PROGRAM_, 1)); //  0
    table.push(Reduction::from(PROGRAM, 2)); //  0
    table.push(Reduction::from(PROGRAM, 0)); //  0
    table.push(Reduction::from(TYPE, 1)); //  0
    table.push(Reduction::from(TYPE, 1)); //  0
    table.push(Reduction::from(TYPE, 1)); //  0
    table.push(Reduction::from(TYPE, 1)); //  0
    table.push(Reduction::from(ARRAY_TYPE, 4)); //  0
    table.push(Reduction::from(POINTER_TYPE, 2)); //  0
    table.push(Reduction::from(BLOCK, 3)); //  0
    table.push(Reduction::from(STATEMENT_LIST, 3)); //  0
    table.push(Reduction::from(STATEMENT_LIST, 1)); //  0
    table.push(Reduction::from(STATEMENT_LIST, 0)); //  0
    table.push(Reduction::from(VAR_DECL, 2)); //  0
    table.push(Reduction::from(FUNCTION_DECL, 4)); //  0
    table.push(Reduction::from(PARAMETERS, 3)); //  0
    table.push(Reduction::from(PARAMETER_LIST, 3)); //  0
    table.push(Reduction::from(PARAMETER_LIST, 1)); //  0
    table.push(Reduction::from(PARAMETER_LIST, 0)); //  0
    table.push(Reduction::from(PARAMETER_DECL, 2)); //  0
    table.push(Reduction::from(OPERAND, 1)); //  0
    table.push(Reduction::from(OPERAND, 1)); //  0
    table.push(Reduction::from(OPERAND, 1)); //  0
    table.push(Reduction::from(OPERAND, 3)); //  0
    table.push(Reduction::from(PRIMARY_EXPR, 2)); //  0
    table.push(Reduction::from(PRIMARY_EXPR, 2)); //  0
    table.push(Reduction::from(PRIMARY_EXPR, 1)); //  0
    table.push(Reduction::from(INDEX, 3)); //  0
    table.push(Reduction::from(ARGUMENTS, 3)); //  0
    table.push(Reduction::from(EXPRESSION_LIST, 3)); //  0
    table.push(Reduction::from(EXPRESSION_LIST, 1)); //  0
    table.push(Reduction::from(EXPRESSION_LIST, 0)); //  0
    table.push(Reduction::from(EXPRESSION, 1)); //  0
    table.push(Reduction::from(LOGICAL_EXPR, 3)); //  0
    table.push(Reduction::from(LOGICAL_EXPR, 1)); //  0
    table.push(Reduction::from(RELATIONAL_EXPR, 3)); //  0
    table.push(Reduction::from(RELATIONAL_EXPR, 1)); //  0
    table.push(Reduction::from(ADDITIVE_EXPR, 3)); //  0
    table.push(Reduction::from(ADDITIVE_EXPR, 1)); //  0
    table.push(Reduction::from(MULTIPLICATIVE_EXPR, 3)); //  0
    table.push(Reduction::from(MULTIPLICATIVE_EXPR, 1)); //  0
    table.push(Reduction::from(UNARY_EXPR, 2)); //  0
    table.push(Reduction::from(UNARY_EXPR, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(STATEMENT, 1)); //  0
    table.push(Reduction::from(ASSIGNMENT, 3)); //  0
    table.push(Reduction::from(IF_STMT, 5)); //  0
    table.push(Reduction::from(WHILE_STMT, 5)); //  0
    table.push(Reduction::from(RETURN_STMT, 2)); //  0
    table.push(Reduction::from(BREAK_STMT, 1)); //  0
    table.push(Reduction::from(CONTINUE_STMT, 1)); //  0

    table
}

// parsing table을 return합니다.
// 반환 자료형인 Vec<HashMap<Token, TableElement>>는
// usize(state)와 Token을 key로 하며 TableElement를 element로 가집니다.
// 이 코드는 html2code/main.py에 의해 생성되었습니다.
pub fn get_parsing_table() -> Vec<HashMap<Token, TableElement>> {
    let mut table = vec![];

    // for state 0
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(EOL, Reduce(2));
    hashmap.insert(PROGRAM, Goto(1));
    hashmap.insert(TYPE, Goto(3));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(FUNCTION_DECL, Goto(2));
    table.push(hashmap);

    // for state 1
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Accepted);
    table.push(hashmap);

    // for state 2
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(EOL, Reduce(2));
    hashmap.insert(PROGRAM, Goto(8));
    hashmap.insert(TYPE, Goto(3));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(FUNCTION_DECL, Goto(2));
    table.push(hashmap);

    // for state 3
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Pointer, Shift(11));
    hashmap.insert(Identifier, Shift(9));
    table.push(hashmap);

    // for state 4
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(3));
    hashmap.insert(Pointer, Reduce(3));
    hashmap.insert(Identifier, Reduce(3));
    table.push(hashmap);

    // for state 5
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(4));
    hashmap.insert(Pointer, Reduce(4));
    hashmap.insert(Identifier, Reduce(4));
    table.push(hashmap);

    // for state 6
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(5));
    hashmap.insert(Pointer, Reduce(5));
    hashmap.insert(Identifier, Reduce(5));
    table.push(hashmap);

    // for state 7
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(6));
    hashmap.insert(Pointer, Reduce(6));
    hashmap.insert(Identifier, Reduce(6));
    table.push(hashmap);

    // for state 8
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Reduce(1));
    table.push(hashmap);

    // for state 9
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(13));
    hashmap.insert(PARAMETERS, Goto(12));
    table.push(hashmap);

    // for state 10
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(14));
    table.push(hashmap);

    // for state 11
    let mut hashmap = HashMap::new();
    hashmap.insert(Identifier, Reduce(8));
    table.push(hashmap);

    // for state 12
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(16));
    hashmap.insert(BLOCK, Goto(15));
    table.push(hashmap);

    // for state 13
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(Rparen, Reduce(18));
    hashmap.insert(TYPE, Goto(19));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(PARAMETER_LIST, Goto(17));
    hashmap.insert(PARAMETER_DECL, Goto(18));
    table.push(hashmap);

    // for state 14
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(20));
    table.push(hashmap);

    // for state 15
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Reduce(14));
    hashmap.insert(Void, Reduce(14));
    hashmap.insert(EOL, Reduce(14));
    table.push(hashmap);

    // for state 16
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(21));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 17
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(49));
    table.push(hashmap);

    // for state 18
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(17));
    hashmap.insert(Comma, Shift(50));
    table.push(hashmap);

    // for state 19
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Pointer, Shift(11));
    hashmap.insert(Identifier, Shift(51));
    table.push(hashmap);

    // for state 20
    let mut hashmap = HashMap::new();
    hashmap.insert(Identifier, Reduce(7));
    table.push(hashmap);

    // for state 21
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(52));
    table.push(hashmap);

    // for state 22
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(11));
    hashmap.insert(Semicolon, Shift(53));
    table.push(hashmap);

    // for state 23
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(43));
    hashmap.insert(Semicolon, Reduce(43));
    table.push(hashmap);

    // for state 24
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(44));
    hashmap.insert(Semicolon, Reduce(44));
    table.push(hashmap);

    // for state 25
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(45));
    hashmap.insert(Semicolon, Reduce(45));
    table.push(hashmap);

    // for state 26
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(46));
    hashmap.insert(Semicolon, Reduce(46));
    table.push(hashmap);

    // for state 27
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(47));
    hashmap.insert(Semicolon, Reduce(47));
    table.push(hashmap);

    // for state 28
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(48));
    hashmap.insert(Semicolon, Reduce(48));
    table.push(hashmap);

    // for state 29
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(49));
    hashmap.insert(Semicolon, Reduce(49));
    table.push(hashmap);

    // for state 30
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(50));
    hashmap.insert(Semicolon, Reduce(50));
    hashmap.insert(AssignOp, Shift(54));
    table.push(hashmap);

    // for state 31
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(10));
    hashmap.insert(Pointer, Shift(11));
    hashmap.insert(Identifier, Shift(55));
    table.push(hashmap);

    // for state 32
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(EXPRESSION, Goto(56));
    hashmap.insert(LOGICAL_EXPR, Goto(57));
    hashmap.insert(RELATIONAL_EXPR, Goto(58));
    hashmap.insert(ADDITIVE_EXPR, Goto(59));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(60));
    hashmap.insert(UNARY_EXPR, Goto(61));
    table.push(hashmap);

    // for state 33
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(55));
    hashmap.insert(Semicolon, Reduce(55));
    table.push(hashmap);

    // for state 34
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(56));
    hashmap.insert(Semicolon, Reduce(56));
    table.push(hashmap);

    // for state 35
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(69));
    table.push(hashmap);

    // for state 36
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(70));
    table.push(hashmap);

    // for state 37
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(32));
    hashmap.insert(Semicolon, Reduce(32));
    hashmap.insert(LogOp, Shift(71));
    hashmap.insert(AssignOp, Reduce(32));
    table.push(hashmap);

    // for state 38
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(34));
    hashmap.insert(Semicolon, Reduce(34));
    hashmap.insert(LogOp, Reduce(34));
    hashmap.insert(RelOp, Shift(72));
    hashmap.insert(AssignOp, Reduce(34));
    table.push(hashmap);

    // for state 39
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(36));
    hashmap.insert(Semicolon, Reduce(36));
    hashmap.insert(LogOp, Reduce(36));
    hashmap.insert(RelOp, Reduce(36));
    hashmap.insert(AddOp, Shift(73));
    hashmap.insert(AssignOp, Reduce(36));
    table.push(hashmap);

    // for state 40
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(Semicolon, Reduce(38));
    hashmap.insert(LogOp, Reduce(38));
    hashmap.insert(RelOp, Reduce(38));
    hashmap.insert(AddOp, Reduce(38));
    hashmap.insert(MulOp, Shift(74));
    hashmap.insert(AssignOp, Reduce(38));
    table.push(hashmap);

    // for state 41
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(40));
    hashmap.insert(Semicolon, Reduce(40));
    hashmap.insert(LogOp, Reduce(40));
    hashmap.insert(RelOp, Reduce(40));
    hashmap.insert(AddOp, Reduce(40));
    hashmap.insert(MulOp, Reduce(40));
    hashmap.insert(AssignOp, Reduce(40));
    table.push(hashmap);

    // for state 42
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(UNARY_EXPR, Goto(75));
    table.push(hashmap);

    // for state 43
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(78));
    hashmap.insert(Rbrace, Reduce(42));
    hashmap.insert(Semicolon, Reduce(42));
    hashmap.insert(Lparen, Shift(79));
    hashmap.insert(LogOp, Reduce(42));
    hashmap.insert(RelOp, Reduce(42));
    hashmap.insert(AddOp, Reduce(42));
    hashmap.insert(MulOp, Reduce(42));
    hashmap.insert(AssignOp, Reduce(42));
    hashmap.insert(INDEX, Goto(76));
    hashmap.insert(ARGUMENTS, Goto(77));
    table.push(hashmap);

    // for state 44
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Rbrace, Reduce(26));
    hashmap.insert(Semicolon, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp, Reduce(26));
    hashmap.insert(RelOp, Reduce(26));
    hashmap.insert(AddOp, Reduce(26));
    hashmap.insert(MulOp, Reduce(26));
    hashmap.insert(AssignOp, Reduce(26));
    table.push(hashmap);

    // for state 45
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Rbrace, Reduce(20));
    hashmap.insert(Semicolon, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp, Reduce(20));
    hashmap.insert(RelOp, Reduce(20));
    hashmap.insert(AddOp, Reduce(20));
    hashmap.insert(MulOp, Reduce(20));
    hashmap.insert(AssignOp, Reduce(20));
    table.push(hashmap);

    // for state 46
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Rbrace, Reduce(21));
    hashmap.insert(Semicolon, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp, Reduce(21));
    hashmap.insert(RelOp, Reduce(21));
    hashmap.insert(AddOp, Reduce(21));
    hashmap.insert(MulOp, Reduce(21));
    hashmap.insert(AssignOp, Reduce(21));
    table.push(hashmap);

    // for state 47
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Rbrace, Reduce(22));
    hashmap.insert(Semicolon, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp, Reduce(22));
    hashmap.insert(RelOp, Reduce(22));
    hashmap.insert(AddOp, Reduce(22));
    hashmap.insert(MulOp, Reduce(22));
    hashmap.insert(AssignOp, Reduce(22));
    table.push(hashmap);

    // for state 48
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(80));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 49
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Reduce(15));
    table.push(hashmap);

    // for state 50
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(Rparen, Reduce(18));
    hashmap.insert(TYPE, Goto(19));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(PARAMETER_LIST, Goto(93));
    hashmap.insert(PARAMETER_DECL, Goto(18));
    table.push(hashmap);

    // for state 51
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(19));
    hashmap.insert(Comma, Reduce(19));
    table.push(hashmap);

    // for state 52
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Reduce(9));
    hashmap.insert(Void, Reduce(9));
    hashmap.insert(EOL, Reduce(9));
    table.push(hashmap);

    // for state 53
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(94));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 54
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(EXPRESSION, Goto(95));
    hashmap.insert(LOGICAL_EXPR, Goto(57));
    hashmap.insert(RELATIONAL_EXPR, Goto(58));
    hashmap.insert(ADDITIVE_EXPR, Goto(59));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(60));
    hashmap.insert(UNARY_EXPR, Goto(61));
    table.push(hashmap);

    // for state 55
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(13));
    hashmap.insert(Semicolon, Reduce(13));
    table.push(hashmap);

    // for state 56
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(54));
    hashmap.insert(Semicolon, Reduce(54));
    table.push(hashmap);

    // for state 57
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(32));
    hashmap.insert(Semicolon, Reduce(32));
    hashmap.insert(LogOp, Shift(96));
    table.push(hashmap);

    // for state 58
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(34));
    hashmap.insert(Semicolon, Reduce(34));
    hashmap.insert(LogOp, Reduce(34));
    hashmap.insert(RelOp, Shift(97));
    table.push(hashmap);

    // for state 59
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(36));
    hashmap.insert(Semicolon, Reduce(36));
    hashmap.insert(LogOp, Reduce(36));
    hashmap.insert(RelOp, Reduce(36));
    hashmap.insert(AddOp, Shift(98));
    table.push(hashmap);

    // for state 60
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(Semicolon, Reduce(38));
    hashmap.insert(LogOp, Reduce(38));
    hashmap.insert(RelOp, Reduce(38));
    hashmap.insert(AddOp, Reduce(38));
    hashmap.insert(MulOp, Shift(99));
    table.push(hashmap);

    // for state 61
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(40));
    hashmap.insert(Semicolon, Reduce(40));
    hashmap.insert(LogOp, Reduce(40));
    hashmap.insert(RelOp, Reduce(40));
    hashmap.insert(AddOp, Reduce(40));
    hashmap.insert(MulOp, Reduce(40));
    table.push(hashmap);

    // for state 62
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(100));
    table.push(hashmap);

    // for state 63
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(103));
    hashmap.insert(Rbrace, Reduce(42));
    hashmap.insert(Semicolon, Reduce(42));
    hashmap.insert(Lparen, Shift(104));
    hashmap.insert(LogOp, Reduce(42));
    hashmap.insert(RelOp, Reduce(42));
    hashmap.insert(AddOp, Reduce(42));
    hashmap.insert(MulOp, Reduce(42));
    hashmap.insert(INDEX, Goto(101));
    hashmap.insert(ARGUMENTS, Goto(102));
    table.push(hashmap);

    // for state 64
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Rbrace, Reduce(26));
    hashmap.insert(Semicolon, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp, Reduce(26));
    hashmap.insert(RelOp, Reduce(26));
    hashmap.insert(AddOp, Reduce(26));
    hashmap.insert(MulOp, Reduce(26));
    table.push(hashmap);

    // for state 65
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Rbrace, Reduce(20));
    hashmap.insert(Semicolon, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp, Reduce(20));
    hashmap.insert(RelOp, Reduce(20));
    hashmap.insert(AddOp, Reduce(20));
    hashmap.insert(MulOp, Reduce(20));
    table.push(hashmap);

    // for state 66
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Rbrace, Reduce(21));
    hashmap.insert(Semicolon, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp, Reduce(21));
    hashmap.insert(RelOp, Reduce(21));
    hashmap.insert(AddOp, Reduce(21));
    hashmap.insert(MulOp, Reduce(21));
    table.push(hashmap);

    // for state 67
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Rbrace, Reduce(22));
    hashmap.insert(Semicolon, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp, Reduce(22));
    hashmap.insert(RelOp, Reduce(22));
    hashmap.insert(AddOp, Reduce(22));
    hashmap.insert(MulOp, Reduce(22));
    table.push(hashmap);

    // for state 68
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(105));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 69
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(106));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 70
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(107));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 71
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(RELATIONAL_EXPR, Goto(108));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 72
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(ADDITIVE_EXPR, Goto(109));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 73
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(110));
    hashmap.insert(UNARY_EXPR, Goto(41));
    table.push(hashmap);

    // for state 74
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(UNARY_EXPR, Goto(111));
    table.push(hashmap);

    // for state 75
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(41));
    hashmap.insert(Semicolon, Reduce(41));
    hashmap.insert(LogOp, Reduce(41));
    hashmap.insert(RelOp, Reduce(41));
    hashmap.insert(AddOp, Reduce(41));
    hashmap.insert(MulOp, Reduce(41));
    hashmap.insert(AssignOp, Reduce(41));
    table.push(hashmap);

    // for state 76
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Rbrace, Reduce(24));
    hashmap.insert(Semicolon, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp, Reduce(24));
    hashmap.insert(RelOp, Reduce(24));
    hashmap.insert(AddOp, Reduce(24));
    hashmap.insert(MulOp, Reduce(24));
    hashmap.insert(AssignOp, Reduce(24));
    table.push(hashmap);

    // for state 77
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(Semicolon, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp, Reduce(25));
    hashmap.insert(RelOp, Reduce(25));
    hashmap.insert(AddOp, Reduce(25));
    hashmap.insert(MulOp, Reduce(25));
    hashmap.insert(AssignOp, Reduce(25));
    table.push(hashmap);

    // for state 78
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(EXPRESSION, Goto(112));
    hashmap.insert(LOGICAL_EXPR, Goto(113));
    hashmap.insert(RELATIONAL_EXPR, Goto(114));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 79
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(125));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 80
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(139));
    table.push(hashmap);

    // for state 81
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(32));
    hashmap.insert(LogOp, Shift(140));
    table.push(hashmap);

    // for state 82
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(34));
    hashmap.insert(LogOp, Reduce(34));
    hashmap.insert(RelOp, Shift(141));
    table.push(hashmap);

    // for state 83
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(36));
    hashmap.insert(LogOp, Reduce(36));
    hashmap.insert(RelOp, Reduce(36));
    hashmap.insert(AddOp, Shift(142));
    table.push(hashmap);

    // for state 84
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(38));
    hashmap.insert(LogOp, Reduce(38));
    hashmap.insert(RelOp, Reduce(38));
    hashmap.insert(AddOp, Reduce(38));
    hashmap.insert(MulOp, Shift(143));
    table.push(hashmap);

    // for state 85
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(40));
    hashmap.insert(LogOp, Reduce(40));
    hashmap.insert(RelOp, Reduce(40));
    hashmap.insert(AddOp, Reduce(40));
    hashmap.insert(MulOp, Reduce(40));
    table.push(hashmap);

    // for state 86
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(UNARY_EXPR, Goto(144));
    table.push(hashmap);

    // for state 87
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(147));
    hashmap.insert(Lparen, Shift(148));
    hashmap.insert(Rparen, Reduce(42));
    hashmap.insert(LogOp, Reduce(42));
    hashmap.insert(RelOp, Reduce(42));
    hashmap.insert(AddOp, Reduce(42));
    hashmap.insert(MulOp, Reduce(42));
    hashmap.insert(INDEX, Goto(145));
    hashmap.insert(ARGUMENTS, Goto(146));
    table.push(hashmap);

    // for state 88
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(Rparen, Reduce(26));
    hashmap.insert(LogOp, Reduce(26));
    hashmap.insert(RelOp, Reduce(26));
    hashmap.insert(AddOp, Reduce(26));
    hashmap.insert(MulOp, Reduce(26));
    table.push(hashmap);

    // for state 89
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(Rparen, Reduce(20));
    hashmap.insert(LogOp, Reduce(20));
    hashmap.insert(RelOp, Reduce(20));
    hashmap.insert(AddOp, Reduce(20));
    hashmap.insert(MulOp, Reduce(20));
    table.push(hashmap);

    // for state 90
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(Rparen, Reduce(21));
    hashmap.insert(LogOp, Reduce(21));
    hashmap.insert(RelOp, Reduce(21));
    hashmap.insert(AddOp, Reduce(21));
    hashmap.insert(MulOp, Reduce(21));
    table.push(hashmap);

    // for state 91
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(Rparen, Reduce(22));
    hashmap.insert(LogOp, Reduce(22));
    hashmap.insert(RelOp, Reduce(22));
    hashmap.insert(AddOp, Reduce(22));
    hashmap.insert(MulOp, Reduce(22));
    table.push(hashmap);

    // for state 92
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(149));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 93
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(16));
    table.push(hashmap);

    // for state 94
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(10));
    table.push(hashmap);

    // for state 95
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(51));
    hashmap.insert(Semicolon, Reduce(51));
    table.push(hashmap);

    // for state 96
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(RELATIONAL_EXPR, Goto(150));
    hashmap.insert(ADDITIVE_EXPR, Goto(59));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(60));
    hashmap.insert(UNARY_EXPR, Goto(61));
    table.push(hashmap);

    // for state 97
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(ADDITIVE_EXPR, Goto(151));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(60));
    hashmap.insert(UNARY_EXPR, Goto(61));
    table.push(hashmap);

    // for state 98
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(152));
    hashmap.insert(UNARY_EXPR, Goto(61));
    table.push(hashmap);

    // for state 99
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(65));
    hashmap.insert(Identifier, Shift(67));
    hashmap.insert(Lparen, Shift(68));
    hashmap.insert(StringLit, Shift(66));
    hashmap.insert(UnaryOp, Shift(62));
    hashmap.insert(OPERAND, Goto(64));
    hashmap.insert(PRIMARY_EXPR, Goto(63));
    hashmap.insert(UNARY_EXPR, Goto(153));
    table.push(hashmap);

    // for state 100
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(41));
    hashmap.insert(Semicolon, Reduce(41));
    hashmap.insert(LogOp, Reduce(41));
    hashmap.insert(RelOp, Reduce(41));
    hashmap.insert(AddOp, Reduce(41));
    hashmap.insert(MulOp, Reduce(41));
    table.push(hashmap);

    // for state 101
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Rbrace, Reduce(24));
    hashmap.insert(Semicolon, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp, Reduce(24));
    hashmap.insert(RelOp, Reduce(24));
    hashmap.insert(AddOp, Reduce(24));
    hashmap.insert(MulOp, Reduce(24));
    table.push(hashmap);

    // for state 102
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(Semicolon, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp, Reduce(25));
    hashmap.insert(RelOp, Reduce(25));
    hashmap.insert(AddOp, Reduce(25));
    hashmap.insert(MulOp, Reduce(25));
    table.push(hashmap);

    // for state 103
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(EXPRESSION, Goto(154));
    hashmap.insert(LOGICAL_EXPR, Goto(113));
    hashmap.insert(RELATIONAL_EXPR, Goto(114));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 104
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(155));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 105
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(156));
    table.push(hashmap);

    // for state 106
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(157));
    table.push(hashmap);

    // for state 107
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(158));
    table.push(hashmap);

    // for state 108
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(33));
    hashmap.insert(Semicolon, Reduce(33));
    hashmap.insert(LogOp, Reduce(33));
    hashmap.insert(RelOp, Shift(72));
    hashmap.insert(AssignOp, Reduce(33));
    table.push(hashmap);

    // for state 109
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(35));
    hashmap.insert(Semicolon, Reduce(35));
    hashmap.insert(LogOp, Reduce(35));
    hashmap.insert(RelOp, Reduce(35));
    hashmap.insert(AddOp, Shift(73));
    hashmap.insert(AssignOp, Reduce(35));
    table.push(hashmap);

    // for state 110
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(37));
    hashmap.insert(Semicolon, Reduce(37));
    hashmap.insert(LogOp, Reduce(37));
    hashmap.insert(RelOp, Reduce(37));
    hashmap.insert(AddOp, Reduce(37));
    hashmap.insert(MulOp, Shift(74));
    hashmap.insert(AssignOp, Reduce(37));
    table.push(hashmap);

    // for state 111
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(39));
    hashmap.insert(Semicolon, Reduce(39));
    hashmap.insert(LogOp, Reduce(39));
    hashmap.insert(RelOp, Reduce(39));
    hashmap.insert(AddOp, Reduce(39));
    hashmap.insert(MulOp, Reduce(39));
    hashmap.insert(AssignOp, Reduce(39));
    table.push(hashmap);

    // for state 112
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(159));
    table.push(hashmap);

    // for state 113
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(32));
    hashmap.insert(LogOp, Shift(160));
    table.push(hashmap);

    // for state 114
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(34));
    hashmap.insert(LogOp, Reduce(34));
    hashmap.insert(RelOp, Shift(161));
    table.push(hashmap);

    // for state 115
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(36));
    hashmap.insert(LogOp, Reduce(36));
    hashmap.insert(RelOp, Reduce(36));
    hashmap.insert(AddOp, Shift(162));
    table.push(hashmap);

    // for state 116
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(38));
    hashmap.insert(LogOp, Reduce(38));
    hashmap.insert(RelOp, Reduce(38));
    hashmap.insert(AddOp, Reduce(38));
    hashmap.insert(MulOp, Shift(163));
    table.push(hashmap);

    // for state 117
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(40));
    hashmap.insert(LogOp, Reduce(40));
    hashmap.insert(RelOp, Reduce(40));
    hashmap.insert(AddOp, Reduce(40));
    hashmap.insert(MulOp, Reduce(40));
    table.push(hashmap);

    // for state 118
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(UNARY_EXPR, Goto(164));
    table.push(hashmap);

    // for state 119
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(167));
    hashmap.insert(Rbracket, Reduce(42));
    hashmap.insert(Lparen, Shift(168));
    hashmap.insert(LogOp, Reduce(42));
    hashmap.insert(RelOp, Reduce(42));
    hashmap.insert(AddOp, Reduce(42));
    hashmap.insert(MulOp, Reduce(42));
    hashmap.insert(INDEX, Goto(165));
    hashmap.insert(ARGUMENTS, Goto(166));
    table.push(hashmap);

    // for state 120
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Rbracket, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(LogOp, Reduce(26));
    hashmap.insert(RelOp, Reduce(26));
    hashmap.insert(AddOp, Reduce(26));
    hashmap.insert(MulOp, Reduce(26));
    table.push(hashmap);

    // for state 121
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Rbracket, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(LogOp, Reduce(20));
    hashmap.insert(RelOp, Reduce(20));
    hashmap.insert(AddOp, Reduce(20));
    hashmap.insert(MulOp, Reduce(20));
    table.push(hashmap);

    // for state 122
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Rbracket, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(LogOp, Reduce(21));
    hashmap.insert(RelOp, Reduce(21));
    hashmap.insert(AddOp, Reduce(21));
    hashmap.insert(MulOp, Reduce(21));
    table.push(hashmap);

    // for state 123
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Rbracket, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(LogOp, Reduce(22));
    hashmap.insert(RelOp, Reduce(22));
    hashmap.insert(AddOp, Reduce(22));
    hashmap.insert(MulOp, Reduce(22));
    table.push(hashmap);

    // for state 124
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(169));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 125
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(170));
    table.push(hashmap);

    // for state 126
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(30));
    hashmap.insert(Comma, Shift(171));
    table.push(hashmap);

    // for state 127
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(32));
    hashmap.insert(Comma, Reduce(32));
    hashmap.insert(LogOp, Shift(172));
    table.push(hashmap);

    // for state 128
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(34));
    hashmap.insert(Comma, Reduce(34));
    hashmap.insert(LogOp, Reduce(34));
    hashmap.insert(RelOp, Shift(173));
    table.push(hashmap);

    // for state 129
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(36));
    hashmap.insert(Comma, Reduce(36));
    hashmap.insert(LogOp, Reduce(36));
    hashmap.insert(RelOp, Reduce(36));
    hashmap.insert(AddOp, Shift(174));
    table.push(hashmap);

    // for state 130
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(38));
    hashmap.insert(Comma, Reduce(38));
    hashmap.insert(LogOp, Reduce(38));
    hashmap.insert(RelOp, Reduce(38));
    hashmap.insert(AddOp, Reduce(38));
    hashmap.insert(MulOp, Shift(175));
    table.push(hashmap);

    // for state 131
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(40));
    hashmap.insert(Comma, Reduce(40));
    hashmap.insert(LogOp, Reduce(40));
    hashmap.insert(RelOp, Reduce(40));
    hashmap.insert(AddOp, Reduce(40));
    hashmap.insert(MulOp, Reduce(40));
    table.push(hashmap);

    // for state 132
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(UNARY_EXPR, Goto(176));
    table.push(hashmap);

    // for state 133
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Shift(179));
    hashmap.insert(Lparen, Shift(180));
    hashmap.insert(Rparen, Reduce(42));
    hashmap.insert(Comma, Reduce(42));
    hashmap.insert(LogOp, Reduce(42));
    hashmap.insert(RelOp, Reduce(42));
    hashmap.insert(AddOp, Reduce(42));
    hashmap.insert(MulOp, Reduce(42));
    hashmap.insert(INDEX, Goto(177));
    hashmap.insert(ARGUMENTS, Goto(178));
    table.push(hashmap);

    // for state 134
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(26));
    hashmap.insert(Lparen, Reduce(26));
    hashmap.insert(Rparen, Reduce(26));
    hashmap.insert(Comma, Reduce(26));
    hashmap.insert(LogOp, Reduce(26));
    hashmap.insert(RelOp, Reduce(26));
    hashmap.insert(AddOp, Reduce(26));
    hashmap.insert(MulOp, Reduce(26));
    table.push(hashmap);

    // for state 135
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(20));
    hashmap.insert(Lparen, Reduce(20));
    hashmap.insert(Rparen, Reduce(20));
    hashmap.insert(Comma, Reduce(20));
    hashmap.insert(LogOp, Reduce(20));
    hashmap.insert(RelOp, Reduce(20));
    hashmap.insert(AddOp, Reduce(20));
    hashmap.insert(MulOp, Reduce(20));
    table.push(hashmap);

    // for state 136
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(21));
    hashmap.insert(Lparen, Reduce(21));
    hashmap.insert(Rparen, Reduce(21));
    hashmap.insert(Comma, Reduce(21));
    hashmap.insert(LogOp, Reduce(21));
    hashmap.insert(RelOp, Reduce(21));
    hashmap.insert(AddOp, Reduce(21));
    hashmap.insert(MulOp, Reduce(21));
    table.push(hashmap);

    // for state 137
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(22));
    hashmap.insert(Lparen, Reduce(22));
    hashmap.insert(Rparen, Reduce(22));
    hashmap.insert(Comma, Reduce(22));
    hashmap.insert(LogOp, Reduce(22));
    hashmap.insert(RelOp, Reduce(22));
    hashmap.insert(AddOp, Reduce(22));
    hashmap.insert(MulOp, Reduce(22));
    table.push(hashmap);

    // for state 138
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(EXPRESSION, Goto(181));
    hashmap.insert(LOGICAL_EXPR, Goto(81));
    hashmap.insert(RELATIONAL_EXPR, Goto(82));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 139
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Rbrace, Reduce(23));
    hashmap.insert(Semicolon, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp, Reduce(23));
    hashmap.insert(RelOp, Reduce(23));
    hashmap.insert(AddOp, Reduce(23));
    hashmap.insert(MulOp, Reduce(23));
    hashmap.insert(AssignOp, Reduce(23));
    table.push(hashmap);

    // for state 140
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(RELATIONAL_EXPR, Goto(182));
    hashmap.insert(ADDITIVE_EXPR, Goto(83));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 141
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(ADDITIVE_EXPR, Goto(183));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(84));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 142
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(184));
    hashmap.insert(UNARY_EXPR, Goto(85));
    table.push(hashmap);

    // for state 143
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(89));
    hashmap.insert(Identifier, Shift(91));
    hashmap.insert(Lparen, Shift(92));
    hashmap.insert(StringLit, Shift(90));
    hashmap.insert(UnaryOp, Shift(86));
    hashmap.insert(OPERAND, Goto(88));
    hashmap.insert(PRIMARY_EXPR, Goto(87));
    hashmap.insert(UNARY_EXPR, Goto(185));
    table.push(hashmap);

    // for state 144
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(41));
    hashmap.insert(LogOp, Reduce(41));
    hashmap.insert(RelOp, Reduce(41));
    hashmap.insert(AddOp, Reduce(41));
    hashmap.insert(MulOp, Reduce(41));
    table.push(hashmap);

    // for state 145
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(Rparen, Reduce(24));
    hashmap.insert(LogOp, Reduce(24));
    hashmap.insert(RelOp, Reduce(24));
    hashmap.insert(AddOp, Reduce(24));
    hashmap.insert(MulOp, Reduce(24));
    table.push(hashmap);

    // for state 146
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(Rparen, Reduce(25));
    hashmap.insert(LogOp, Reduce(25));
    hashmap.insert(RelOp, Reduce(25));
    hashmap.insert(AddOp, Reduce(25));
    hashmap.insert(MulOp, Reduce(25));
    table.push(hashmap);

    // for state 147
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(EXPRESSION, Goto(186));
    hashmap.insert(LOGICAL_EXPR, Goto(113));
    hashmap.insert(RELATIONAL_EXPR, Goto(114));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 148
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(187));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 149
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(188));
    table.push(hashmap);

    // for state 150
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(33));
    hashmap.insert(Semicolon, Reduce(33));
    hashmap.insert(LogOp, Reduce(33));
    hashmap.insert(RelOp, Shift(97));
    table.push(hashmap);

    // for state 151
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(35));
    hashmap.insert(Semicolon, Reduce(35));
    hashmap.insert(LogOp, Reduce(35));
    hashmap.insert(RelOp, Reduce(35));
    hashmap.insert(AddOp, Shift(98));
    table.push(hashmap);

    // for state 152
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(37));
    hashmap.insert(Semicolon, Reduce(37));
    hashmap.insert(LogOp, Reduce(37));
    hashmap.insert(RelOp, Reduce(37));
    hashmap.insert(AddOp, Reduce(37));
    hashmap.insert(MulOp, Shift(99));
    table.push(hashmap);

    // for state 153
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(39));
    hashmap.insert(Semicolon, Reduce(39));
    hashmap.insert(LogOp, Reduce(39));
    hashmap.insert(RelOp, Reduce(39));
    hashmap.insert(AddOp, Reduce(39));
    hashmap.insert(MulOp, Reduce(39));
    table.push(hashmap);

    // for state 154
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(189));
    table.push(hashmap);

    // for state 155
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(190));
    table.push(hashmap);

    // for state 156
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Rbrace, Reduce(23));
    hashmap.insert(Semicolon, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp, Reduce(23));
    hashmap.insert(RelOp, Reduce(23));
    hashmap.insert(AddOp, Reduce(23));
    hashmap.insert(MulOp, Reduce(23));
    table.push(hashmap);

    // for state 157
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(192));
    hashmap.insert(BLOCK, Goto(191));
    table.push(hashmap);

    // for state 158
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(192));
    hashmap.insert(BLOCK, Goto(193));
    table.push(hashmap);

    // for state 159
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Rbrace, Reduce(27));
    hashmap.insert(Semicolon, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp, Reduce(27));
    hashmap.insert(RelOp, Reduce(27));
    hashmap.insert(AddOp, Reduce(27));
    hashmap.insert(MulOp, Reduce(27));
    hashmap.insert(AssignOp, Reduce(27));
    table.push(hashmap);

    // for state 160
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(RELATIONAL_EXPR, Goto(194));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 161
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(ADDITIVE_EXPR, Goto(195));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 162
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(196));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 163
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(UNARY_EXPR, Goto(197));
    table.push(hashmap);

    // for state 164
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(41));
    hashmap.insert(LogOp, Reduce(41));
    hashmap.insert(RelOp, Reduce(41));
    hashmap.insert(AddOp, Reduce(41));
    hashmap.insert(MulOp, Reduce(41));
    table.push(hashmap);

    // for state 165
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Rbracket, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(LogOp, Reduce(24));
    hashmap.insert(RelOp, Reduce(24));
    hashmap.insert(AddOp, Reduce(24));
    hashmap.insert(MulOp, Reduce(24));
    table.push(hashmap);

    // for state 166
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Rbracket, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(LogOp, Reduce(25));
    hashmap.insert(RelOp, Reduce(25));
    hashmap.insert(AddOp, Reduce(25));
    hashmap.insert(MulOp, Reduce(25));
    table.push(hashmap);

    // for state 167
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(EXPRESSION, Goto(198));
    hashmap.insert(LOGICAL_EXPR, Goto(113));
    hashmap.insert(RELATIONAL_EXPR, Goto(114));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 168
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(199));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 169
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(200));
    table.push(hashmap);

    // for state 170
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Rbrace, Reduce(28));
    hashmap.insert(Semicolon, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp, Reduce(28));
    hashmap.insert(RelOp, Reduce(28));
    hashmap.insert(AddOp, Reduce(28));
    hashmap.insert(MulOp, Reduce(28));
    hashmap.insert(AssignOp, Reduce(28));
    table.push(hashmap);

    // for state 171
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(201));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 172
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(RELATIONAL_EXPR, Goto(202));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 173
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(ADDITIVE_EXPR, Goto(203));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 174
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(204));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 175
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(UNARY_EXPR, Goto(205));
    table.push(hashmap);

    // for state 176
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(41));
    hashmap.insert(Comma, Reduce(41));
    hashmap.insert(LogOp, Reduce(41));
    hashmap.insert(RelOp, Reduce(41));
    hashmap.insert(AddOp, Reduce(41));
    hashmap.insert(MulOp, Reduce(41));
    table.push(hashmap);

    // for state 177
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(24));
    hashmap.insert(Lparen, Reduce(24));
    hashmap.insert(Rparen, Reduce(24));
    hashmap.insert(Comma, Reduce(24));
    hashmap.insert(LogOp, Reduce(24));
    hashmap.insert(RelOp, Reduce(24));
    hashmap.insert(AddOp, Reduce(24));
    hashmap.insert(MulOp, Reduce(24));
    table.push(hashmap);

    // for state 178
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(25));
    hashmap.insert(Lparen, Reduce(25));
    hashmap.insert(Rparen, Reduce(25));
    hashmap.insert(Comma, Reduce(25));
    hashmap.insert(LogOp, Reduce(25));
    hashmap.insert(RelOp, Reduce(25));
    hashmap.insert(AddOp, Reduce(25));
    hashmap.insert(MulOp, Reduce(25));
    table.push(hashmap);

    // for state 179
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(121));
    hashmap.insert(Identifier, Shift(123));
    hashmap.insert(Lparen, Shift(124));
    hashmap.insert(StringLit, Shift(122));
    hashmap.insert(UnaryOp, Shift(118));
    hashmap.insert(OPERAND, Goto(120));
    hashmap.insert(PRIMARY_EXPR, Goto(119));
    hashmap.insert(EXPRESSION, Goto(206));
    hashmap.insert(LOGICAL_EXPR, Goto(113));
    hashmap.insert(RELATIONAL_EXPR, Goto(114));
    hashmap.insert(ADDITIVE_EXPR, Goto(115));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(116));
    hashmap.insert(UNARY_EXPR, Goto(117));
    table.push(hashmap);

    // for state 180
    let mut hashmap = HashMap::new();
    hashmap.insert(IntLit, Shift(135));
    hashmap.insert(Identifier, Shift(137));
    hashmap.insert(Lparen, Shift(138));
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(StringLit, Shift(136));
    hashmap.insert(UnaryOp, Shift(132));
    hashmap.insert(OPERAND, Goto(134));
    hashmap.insert(PRIMARY_EXPR, Goto(133));
    hashmap.insert(EXPRESSION_LIST, Goto(207));
    hashmap.insert(EXPRESSION, Goto(126));
    hashmap.insert(LOGICAL_EXPR, Goto(127));
    hashmap.insert(RELATIONAL_EXPR, Goto(128));
    hashmap.insert(ADDITIVE_EXPR, Goto(129));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(130));
    hashmap.insert(UNARY_EXPR, Goto(131));
    table.push(hashmap);

    // for state 181
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(208));
    table.push(hashmap);

    // for state 182
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(33));
    hashmap.insert(LogOp, Reduce(33));
    hashmap.insert(RelOp, Shift(141));
    table.push(hashmap);

    // for state 183
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(35));
    hashmap.insert(LogOp, Reduce(35));
    hashmap.insert(RelOp, Reduce(35));
    hashmap.insert(AddOp, Shift(142));
    table.push(hashmap);

    // for state 184
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(37));
    hashmap.insert(LogOp, Reduce(37));
    hashmap.insert(RelOp, Reduce(37));
    hashmap.insert(AddOp, Reduce(37));
    hashmap.insert(MulOp, Shift(143));
    table.push(hashmap);

    // for state 185
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(39));
    hashmap.insert(LogOp, Reduce(39));
    hashmap.insert(RelOp, Reduce(39));
    hashmap.insert(AddOp, Reduce(39));
    hashmap.insert(MulOp, Reduce(39));
    table.push(hashmap);

    // for state 186
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(209));
    table.push(hashmap);

    // for state 187
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(210));
    table.push(hashmap);

    // for state 188
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(LogOp, Reduce(23));
    hashmap.insert(RelOp, Reduce(23));
    hashmap.insert(AddOp, Reduce(23));
    hashmap.insert(MulOp, Reduce(23));
    table.push(hashmap);

    // for state 189
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Rbrace, Reduce(27));
    hashmap.insert(Semicolon, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp, Reduce(27));
    hashmap.insert(RelOp, Reduce(27));
    hashmap.insert(AddOp, Reduce(27));
    hashmap.insert(MulOp, Reduce(27));
    table.push(hashmap);

    // for state 190
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Rbrace, Reduce(28));
    hashmap.insert(Semicolon, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp, Reduce(28));
    hashmap.insert(RelOp, Reduce(28));
    hashmap.insert(AddOp, Reduce(28));
    hashmap.insert(MulOp, Reduce(28));
    table.push(hashmap);

    // for state 191
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(52));
    hashmap.insert(Semicolon, Reduce(52));
    table.push(hashmap);

    // for state 192
    let mut hashmap = HashMap::new();
    hashmap.insert(Int, Shift(4));
    hashmap.insert(Void, Shift(5));
    hashmap.insert(IntLit, Shift(45));
    hashmap.insert(Rbrace, Reduce(12));
    hashmap.insert(Identifier, Shift(47));
    hashmap.insert(Lparen, Shift(48));
    hashmap.insert(StringLit, Shift(46));
    hashmap.insert(UnaryOp, Shift(42));
    hashmap.insert(If, Shift(35));
    hashmap.insert(While, Shift(36));
    hashmap.insert(Return, Shift(32));
    hashmap.insert(Break, Shift(33));
    hashmap.insert(Continue, Shift(34));
    hashmap.insert(TYPE, Goto(31));
    hashmap.insert(ARRAY_TYPE, Goto(6));
    hashmap.insert(POINTER_TYPE, Goto(7));
    hashmap.insert(STATEMENT_LIST, Goto(211));
    hashmap.insert(VAR_DECL, Goto(24));
    hashmap.insert(OPERAND, Goto(44));
    hashmap.insert(PRIMARY_EXPR, Goto(43));
    hashmap.insert(EXPRESSION, Goto(30));
    hashmap.insert(LOGICAL_EXPR, Goto(37));
    hashmap.insert(RELATIONAL_EXPR, Goto(38));
    hashmap.insert(ADDITIVE_EXPR, Goto(39));
    hashmap.insert(MULTIPLICATIVE_EXPR, Goto(40));
    hashmap.insert(UNARY_EXPR, Goto(41));
    hashmap.insert(STATEMENT, Goto(22));
    hashmap.insert(ASSIGNMENT, Goto(23));
    hashmap.insert(IF_STMT, Goto(28));
    hashmap.insert(WHILE_STMT, Goto(29));
    hashmap.insert(RETURN_STMT, Goto(25));
    hashmap.insert(BREAK_STMT, Goto(26));
    hashmap.insert(CONTINUE_STMT, Goto(27));
    table.push(hashmap);

    // for state 193
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(53));
    hashmap.insert(Semicolon, Reduce(53));
    table.push(hashmap);

    // for state 194
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(33));
    hashmap.insert(LogOp, Reduce(33));
    hashmap.insert(RelOp, Shift(161));
    table.push(hashmap);

    // for state 195
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(35));
    hashmap.insert(LogOp, Reduce(35));
    hashmap.insert(RelOp, Reduce(35));
    hashmap.insert(AddOp, Shift(162));
    table.push(hashmap);

    // for state 196
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(37));
    hashmap.insert(LogOp, Reduce(37));
    hashmap.insert(RelOp, Reduce(37));
    hashmap.insert(AddOp, Reduce(37));
    hashmap.insert(MulOp, Shift(163));
    table.push(hashmap);

    // for state 197
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Reduce(39));
    hashmap.insert(LogOp, Reduce(39));
    hashmap.insert(RelOp, Reduce(39));
    hashmap.insert(AddOp, Reduce(39));
    hashmap.insert(MulOp, Reduce(39));
    table.push(hashmap);

    // for state 198
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(212));
    table.push(hashmap);

    // for state 199
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(213));
    table.push(hashmap);

    // for state 200
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Rbracket, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(LogOp, Reduce(23));
    hashmap.insert(RelOp, Reduce(23));
    hashmap.insert(AddOp, Reduce(23));
    hashmap.insert(MulOp, Reduce(23));
    table.push(hashmap);

    // for state 201
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(29));
    table.push(hashmap);

    // for state 202
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(33));
    hashmap.insert(Comma, Reduce(33));
    hashmap.insert(LogOp, Reduce(33));
    hashmap.insert(RelOp, Shift(173));
    table.push(hashmap);

    // for state 203
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(35));
    hashmap.insert(Comma, Reduce(35));
    hashmap.insert(LogOp, Reduce(35));
    hashmap.insert(RelOp, Reduce(35));
    hashmap.insert(AddOp, Shift(174));
    table.push(hashmap);

    // for state 204
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(37));
    hashmap.insert(Comma, Reduce(37));
    hashmap.insert(LogOp, Reduce(37));
    hashmap.insert(RelOp, Reduce(37));
    hashmap.insert(AddOp, Reduce(37));
    hashmap.insert(MulOp, Shift(175));
    table.push(hashmap);

    // for state 205
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(39));
    hashmap.insert(Comma, Reduce(39));
    hashmap.insert(LogOp, Reduce(39));
    hashmap.insert(RelOp, Reduce(39));
    hashmap.insert(AddOp, Reduce(39));
    hashmap.insert(MulOp, Reduce(39));
    table.push(hashmap);

    // for state 206
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbracket, Shift(214));
    table.push(hashmap);

    // for state 207
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(215));
    table.push(hashmap);

    // for state 208
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(23));
    hashmap.insert(Lparen, Reduce(23));
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(Comma, Reduce(23));
    hashmap.insert(LogOp, Reduce(23));
    hashmap.insert(RelOp, Reduce(23));
    hashmap.insert(AddOp, Reduce(23));
    hashmap.insert(MulOp, Reduce(23));
    table.push(hashmap);

    // for state 209
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(Rparen, Reduce(27));
    hashmap.insert(LogOp, Reduce(27));
    hashmap.insert(RelOp, Reduce(27));
    hashmap.insert(AddOp, Reduce(27));
    hashmap.insert(MulOp, Reduce(27));
    table.push(hashmap);

    // for state 210
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(Rparen, Reduce(28));
    hashmap.insert(LogOp, Reduce(28));
    hashmap.insert(RelOp, Reduce(28));
    hashmap.insert(AddOp, Reduce(28));
    hashmap.insert(MulOp, Reduce(28));
    table.push(hashmap);

    // for state 211
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(216));
    table.push(hashmap);

    // for state 212
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Rbracket, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(LogOp, Reduce(27));
    hashmap.insert(RelOp, Reduce(27));
    hashmap.insert(AddOp, Reduce(27));
    hashmap.insert(MulOp, Reduce(27));
    table.push(hashmap);

    // for state 213
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Rbracket, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(LogOp, Reduce(28));
    hashmap.insert(RelOp, Reduce(28));
    hashmap.insert(AddOp, Reduce(28));
    hashmap.insert(MulOp, Reduce(28));
    table.push(hashmap);

    // for state 214
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(27));
    hashmap.insert(Lparen, Reduce(27));
    hashmap.insert(Rparen, Reduce(27));
    hashmap.insert(Comma, Reduce(27));
    hashmap.insert(LogOp, Reduce(27));
    hashmap.insert(RelOp, Reduce(27));
    hashmap.insert(AddOp, Reduce(27));
    hashmap.insert(MulOp, Reduce(27));
    table.push(hashmap);

    // for state 215
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbracket, Reduce(28));
    hashmap.insert(Lparen, Reduce(28));
    hashmap.insert(Rparen, Reduce(28));
    hashmap.insert(Comma, Reduce(28));
    hashmap.insert(LogOp, Reduce(28));
    hashmap.insert(RelOp, Reduce(28));
    hashmap.insert(AddOp, Reduce(28));
    hashmap.insert(MulOp, Reduce(28));
    table.push(hashmap);

    // for state 216
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(9));
    hashmap.insert(Semicolon, Reduce(9));
    table.push(hashmap);

    table
}
