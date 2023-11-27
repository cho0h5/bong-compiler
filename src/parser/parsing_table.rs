// 이 파일은 parsing을 위해 필요한 정보인 CFG와 parsing table의 정보가
// hard coding되어있습니다.

use std::collections::HashMap;
use TableElement::*;
use crate::token_reader::Token;
use crate::token_reader::Token::*;

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
        Reduction { left: left, right: right }
    }
}

// 총 39개의 reduction rule 정보를 return합니다.
pub fn get_reduction_table() -> Vec<Reduction> {
    let mut table = vec![];

    table.push(Reduction::from(CODE_, 1));      //  0
    table.push(Reduction::from(CODE, 2));
    table.push(Reduction::from(CODE, 2));
    table.push(Reduction::from(CODE, 2));
    table.push(Reduction::from(CODE, 0));
    table.push(Reduction::from(VDECL, 3));
    table.push(Reduction::from(VDECL, 3));
    table.push(Reduction::from(ASSIGN, 3));
    table.push(Reduction::from(RHS, 1));
    table.push(Reduction::from(RHS, 1));
    table.push(Reduction::from(RHS, 1));        // 10
    table.push(Reduction::from(RHS, 1));
    table.push(Reduction::from(EXPR, 3));
    table.push(Reduction::from(EXPR, 1));
    table.push(Reduction::from(EXPR_, 3));
    table.push(Reduction::from(EXPR_, 1));
    table.push(Reduction::from(EXPR__, 3));
    table.push(Reduction::from(EXPR__, 1));
    table.push(Reduction::from(EXPR__, 1));
    table.push(Reduction::from(FDECL, 9));
    table.push(Reduction::from(ARG, 3));        // 20
    table.push(Reduction::from(ARG, 0));
    table.push(Reduction::from(MOREARGS, 4));
    table.push(Reduction::from(MOREARGS, 0));
    table.push(Reduction::from(BLOCK, 2));
    table.push(Reduction::from(BLOCK, 0));
    table.push(Reduction::from(STMT, 1));
    table.push(Reduction::from(STMT, 2));
    table.push(Reduction::from(STMT, 8));
    table.push(Reduction::from(STMT, 7));
    table.push(Reduction::from(COND, 3));       // 30
    table.push(Reduction::from(COND, 1));
    table.push(Reduction::from(ELSE, 4));
    table.push(Reduction::from(ELSE, 0));
    table.push(Reduction::from(RETURN, 3));
    table.push(Reduction::from(CDECL, 5));
    table.push(Reduction::from(ODECL, 2));
    table.push(Reduction::from(ODECL, 2));
    table.push(Reduction::from(ODECL, 0));      // 38

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
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Class, Shift(6));
    hashmap.insert(EOL, Reduce(4));
    hashmap.insert(CODE, Goto(1));
    hashmap.insert(VDECL, Goto(2));
    hashmap.insert(FDECL, Goto(3));
    hashmap.insert(CDECL, Goto(4));
    table.push(hashmap);

    // for state 1
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Accepted);
    table.push(hashmap);

    // for state 2
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Class, Shift(6));
    hashmap.insert(EOL, Reduce(4));
    hashmap.insert(CODE, Goto(7));
    hashmap.insert(VDECL, Goto(2));
    hashmap.insert(FDECL, Goto(3));
    hashmap.insert(CDECL, Goto(4));
    table.push(hashmap);

    // for state 3
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Class, Shift(6));
    hashmap.insert(EOL, Reduce(4));
    hashmap.insert(CODE, Goto(8));
    hashmap.insert(VDECL, Goto(2));
    hashmap.insert(FDECL, Goto(3));
    hashmap.insert(CDECL, Goto(4));
    table.push(hashmap);

    // for state 4
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Class, Shift(6));
    hashmap.insert(EOL, Reduce(4));
    hashmap.insert(CODE, Goto(9));
    hashmap.insert(VDECL, Goto(2));
    hashmap.insert(FDECL, Goto(3));
    hashmap.insert(CDECL, Goto(4));
    table.push(hashmap);

    // for state 5
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(10));
    hashmap.insert(ASSIGN, Goto(11));
    table.push(hashmap);

    // for state 6
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(12));
    table.push(hashmap);

    // for state 7
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Reduce(1));
    table.push(hashmap);

    // for state 8
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Reduce(2));
    table.push(hashmap);

    // for state 9
    let mut hashmap = HashMap::new();
    hashmap.insert(EOL, Reduce(3));
    table.push(hashmap);

    // for state 10
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Shift(13));
    hashmap.insert(Assign, Shift(15));
    hashmap.insert(Lparen, Shift(14));
    table.push(hashmap);

    // for state 11
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Shift(16));
    table.push(hashmap);

    // for state 12
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(17));
    table.push(hashmap);

    // for state 13
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(5));
    hashmap.insert(Id, Reduce(5));
    hashmap.insert(Rbrace, Reduce(5));
    hashmap.insert(If, Reduce(5));
    hashmap.insert(While, Reduce(5));
    hashmap.insert(Return, Reduce(5));
    hashmap.insert(Class, Reduce(5));
    hashmap.insert(EOL, Reduce(5));
    table.push(hashmap);

    // for state 14
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(19));
    hashmap.insert(Rparen, Reduce(21));
    hashmap.insert(ARG, Goto(18));
    table.push(hashmap);

    // for state 15
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(28));
    hashmap.insert(Literal, Shift(22));
    hashmap.insert(Character, Shift(23));
    hashmap.insert(Boolstr, Shift(24));
    hashmap.insert(Lparen, Shift(27));
    hashmap.insert(Num, Shift(29));
    hashmap.insert(RHS, Goto(20));
    hashmap.insert(EXPR, Goto(21));
    hashmap.insert(EXPR_, Goto(25));
    hashmap.insert(EXPR__, Goto(26));
    table.push(hashmap);

    // for state 16
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(6));
    hashmap.insert(Id, Reduce(6));
    hashmap.insert(Rbrace, Reduce(6));
    hashmap.insert(If, Reduce(6));
    hashmap.insert(While, Reduce(6));
    hashmap.insert(Return, Reduce(6));
    hashmap.insert(Class, Reduce(6));
    hashmap.insert(EOL, Reduce(6));
    table.push(hashmap);

    // for state 17
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(VDECL, Goto(31));
    hashmap.insert(FDECL, Goto(32));
    hashmap.insert(ODECL, Goto(30));
    table.push(hashmap);

    // for state 18
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(33));
    table.push(hashmap);

    // for state 19
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(34));
    table.push(hashmap);

    // for state 20
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(7));
    table.push(hashmap);

    // for state 21
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(8));
    hashmap.insert(Addsub, Shift(35));
    table.push(hashmap);

    // for state 22
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(9));
    table.push(hashmap);

    // for state 23
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(10));
    table.push(hashmap);

    // for state 24
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(11));
    table.push(hashmap);

    // for state 25
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(13));
    hashmap.insert(Addsub, Reduce(13));
    hashmap.insert(Multdiv, Shift(36));
    hashmap.insert(Rparen, Reduce(13));
    table.push(hashmap);

    // for state 26
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(15));
    hashmap.insert(Addsub, Reduce(15));
    hashmap.insert(Multdiv, Reduce(15));
    hashmap.insert(Rparen, Reduce(15));
    table.push(hashmap);

    // for state 27
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(28));
    hashmap.insert(Lparen, Shift(27));
    hashmap.insert(Num, Shift(29));
    hashmap.insert(EXPR, Goto(37));
    hashmap.insert(EXPR_, Goto(25));
    hashmap.insert(EXPR__, Goto(26));
    table.push(hashmap);

    // for state 28
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(17));
    hashmap.insert(Addsub, Reduce(17));
    hashmap.insert(Multdiv, Reduce(17));
    hashmap.insert(Rparen, Reduce(17));
    table.push(hashmap);

    // for state 29
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(18));
    hashmap.insert(Addsub, Reduce(18));
    hashmap.insert(Multdiv, Reduce(18));
    hashmap.insert(Rparen, Reduce(18));
    table.push(hashmap);

    // for state 30
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(38));
    table.push(hashmap);

    // for state 31
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(VDECL, Goto(31));
    hashmap.insert(FDECL, Goto(32));
    hashmap.insert(ODECL, Goto(39));
    table.push(hashmap);

    // for state 32
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(5));
    hashmap.insert(Rbrace, Reduce(38));
    hashmap.insert(VDECL, Goto(31));
    hashmap.insert(FDECL, Goto(32));
    hashmap.insert(ODECL, Goto(40));
    table.push(hashmap);

    // for state 33
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(41));
    table.push(hashmap);

    // for state 34
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(Comma, Shift(43));
    hashmap.insert(MOREARGS, Goto(42));
    table.push(hashmap);

    // for state 35
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(28));
    hashmap.insert(Lparen, Shift(27));
    hashmap.insert(Num, Shift(29));
    hashmap.insert(EXPR_, Goto(44));
    hashmap.insert(EXPR__, Goto(26));
    table.push(hashmap);

    // for state 36
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(28));
    hashmap.insert(Lparen, Shift(27));
    hashmap.insert(Num, Shift(29));
    hashmap.insert(EXPR__, Goto(45));
    table.push(hashmap);

    // for state 37
    let mut hashmap = HashMap::new();
    hashmap.insert(Addsub, Shift(35));
    hashmap.insert(Rparen, Shift(46));
    table.push(hashmap);

    // for state 38
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(35));
    hashmap.insert(Class, Reduce(35));
    hashmap.insert(EOL, Reduce(35));
    table.push(hashmap);

    // for state 39
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(36));
    table.push(hashmap);

    // for state 40
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(37));
    table.push(hashmap);

    // for state 41
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(53));
    hashmap.insert(Id, Shift(54));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(If, Shift(51));
    hashmap.insert(While, Shift(52));
    hashmap.insert(Return, Reduce(25));
    hashmap.insert(VDECL, Goto(49));
    hashmap.insert(ASSIGN, Goto(50));
    hashmap.insert(BLOCK, Goto(47));
    hashmap.insert(STMT, Goto(48));
    table.push(hashmap);

    // for state 42
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(20));
    table.push(hashmap);

    // for state 43
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(55));
    table.push(hashmap);

    // for state 44
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(12));
    hashmap.insert(Addsub, Reduce(12));
    hashmap.insert(Multdiv, Shift(36));
    hashmap.insert(Rparen, Reduce(12));
    table.push(hashmap);

    // for state 45
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(14));
    hashmap.insert(Addsub, Reduce(14));
    hashmap.insert(Multdiv, Reduce(14));
    hashmap.insert(Rparen, Reduce(14));
    table.push(hashmap);

    // for state 46
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Reduce(16));
    hashmap.insert(Addsub, Reduce(16));
    hashmap.insert(Multdiv, Reduce(16));
    hashmap.insert(Rparen, Reduce(16));
    table.push(hashmap);

    // for state 47
    let mut hashmap = HashMap::new();
    hashmap.insert(Return, Shift(57));
    hashmap.insert(RETURN, Goto(56));
    table.push(hashmap);

    // for state 48
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(53));
    hashmap.insert(Id, Shift(54));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(If, Shift(51));
    hashmap.insert(While, Shift(52));
    hashmap.insert(Return, Reduce(25));
    hashmap.insert(VDECL, Goto(49));
    hashmap.insert(ASSIGN, Goto(50));
    hashmap.insert(BLOCK, Goto(58));
    hashmap.insert(STMT, Goto(48));
    table.push(hashmap);

    // for state 49
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(26));
    hashmap.insert(Id, Reduce(26));
    hashmap.insert(Rbrace, Reduce(26));
    hashmap.insert(If, Reduce(26));
    hashmap.insert(While, Reduce(26));
    hashmap.insert(Return, Reduce(26));
    table.push(hashmap);

    // for state 50
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Shift(59));
    table.push(hashmap);

    // for state 51
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(60));
    table.push(hashmap);

    // for state 52
    let mut hashmap = HashMap::new();
    hashmap.insert(Lparen, Shift(61));
    table.push(hashmap);

    // for state 53
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(62));
    hashmap.insert(ASSIGN, Goto(11));
    table.push(hashmap);

    // for state 54
    let mut hashmap = HashMap::new();
    hashmap.insert(Assign, Shift(15));
    table.push(hashmap);

    // for state 55
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(63));
    table.push(hashmap);

    // for state 56
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(64));
    table.push(hashmap);

    // for state 57
    let mut hashmap = HashMap::new();
    hashmap.insert(Id, Shift(28));
    hashmap.insert(Literal, Shift(22));
    hashmap.insert(Character, Shift(23));
    hashmap.insert(Boolstr, Shift(24));
    hashmap.insert(Lparen, Shift(27));
    hashmap.insert(Num, Shift(29));
    hashmap.insert(RHS, Goto(65));
    hashmap.insert(EXPR, Goto(21));
    hashmap.insert(EXPR_, Goto(25));
    hashmap.insert(EXPR__, Goto(26));
    table.push(hashmap);

    // for state 58
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(24));
    hashmap.insert(Return, Reduce(24));
    table.push(hashmap);

    // for state 59
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(27));
    hashmap.insert(Id, Reduce(27));
    hashmap.insert(Rbrace, Reduce(27));
    hashmap.insert(If, Reduce(27));
    hashmap.insert(While, Reduce(27));
    hashmap.insert(Return, Reduce(27));
    table.push(hashmap);

    // for state 60
    let mut hashmap = HashMap::new();
    hashmap.insert(Boolstr, Shift(67));
    hashmap.insert(COND, Goto(66));
    table.push(hashmap);

    // for state 61
    let mut hashmap = HashMap::new();
    hashmap.insert(Boolstr, Shift(67));
    hashmap.insert(COND, Goto(68));
    table.push(hashmap);

    // for state 62
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Shift(13));
    hashmap.insert(Assign, Shift(15));
    table.push(hashmap);

    // for state 63
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(23));
    hashmap.insert(Comma, Shift(43));
    hashmap.insert(MOREARGS, Goto(69));
    table.push(hashmap);

    // for state 64
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(19));
    hashmap.insert(Rbrace, Reduce(19));
    hashmap.insert(Class, Reduce(19));
    hashmap.insert(EOL, Reduce(19));
    table.push(hashmap);

    // for state 65
    let mut hashmap = HashMap::new();
    hashmap.insert(Semi, Shift(70));
    table.push(hashmap);

    // for state 66
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(71));
    hashmap.insert(Comp, Shift(72));
    table.push(hashmap);

    // for state 67
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(31));
    hashmap.insert(Comp, Reduce(31));
    table.push(hashmap);

    // for state 68
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Shift(73));
    hashmap.insert(Comp, Shift(72));
    table.push(hashmap);

    // for state 69
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(22));
    table.push(hashmap);

    // for state 70
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Reduce(34));
    table.push(hashmap);

    // for state 71
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(74));
    table.push(hashmap);

    // for state 72
    let mut hashmap = HashMap::new();
    hashmap.insert(Boolstr, Shift(75));
    table.push(hashmap);

    // for state 73
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(76));
    table.push(hashmap);

    // for state 74
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(53));
    hashmap.insert(Id, Shift(54));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(If, Shift(51));
    hashmap.insert(While, Shift(52));
    hashmap.insert(Return, Reduce(25));
    hashmap.insert(VDECL, Goto(49));
    hashmap.insert(ASSIGN, Goto(50));
    hashmap.insert(BLOCK, Goto(77));
    hashmap.insert(STMT, Goto(48));
    table.push(hashmap);

    // for state 75
    let mut hashmap = HashMap::new();
    hashmap.insert(Rparen, Reduce(30));
    hashmap.insert(Comp, Reduce(30));
    table.push(hashmap);

    // for state 76
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(53));
    hashmap.insert(Id, Shift(54));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(If, Shift(51));
    hashmap.insert(While, Shift(52));
    hashmap.insert(Return, Reduce(25));
    hashmap.insert(VDECL, Goto(49));
    hashmap.insert(ASSIGN, Goto(50));
    hashmap.insert(BLOCK, Goto(78));
    hashmap.insert(STMT, Goto(48));
    table.push(hashmap);

    // for state 77
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(79));
    table.push(hashmap);

    // for state 78
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(80));
    table.push(hashmap);

    // for state 79
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(33));
    hashmap.insert(Id, Reduce(33));
    hashmap.insert(Rbrace, Reduce(33));
    hashmap.insert(If, Reduce(33));
    hashmap.insert(While, Reduce(33));
    hashmap.insert(Else, Shift(82));
    hashmap.insert(Return, Reduce(33));
    hashmap.insert(ELSE, Goto(81));
    table.push(hashmap);

    // for state 80
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(29));
    hashmap.insert(Id, Reduce(29));
    hashmap.insert(Rbrace, Reduce(29));
    hashmap.insert(If, Reduce(29));
    hashmap.insert(While, Reduce(29));
    hashmap.insert(Return, Reduce(29));
    table.push(hashmap);

    // for state 81
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(28));
    hashmap.insert(Id, Reduce(28));
    hashmap.insert(Rbrace, Reduce(28));
    hashmap.insert(If, Reduce(28));
    hashmap.insert(While, Reduce(28));
    hashmap.insert(Return, Reduce(28));
    table.push(hashmap);

    // for state 82
    let mut hashmap = HashMap::new();
    hashmap.insert(Lbrace, Shift(83));
    table.push(hashmap);

    // for state 83
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Shift(53));
    hashmap.insert(Id, Shift(54));
    hashmap.insert(Rbrace, Reduce(25));
    hashmap.insert(If, Shift(51));
    hashmap.insert(While, Shift(52));
    hashmap.insert(Return, Reduce(25));
    hashmap.insert(VDECL, Goto(49));
    hashmap.insert(ASSIGN, Goto(50));
    hashmap.insert(BLOCK, Goto(84));
    hashmap.insert(STMT, Goto(48));
    table.push(hashmap);

    // for state 84
    let mut hashmap = HashMap::new();
    hashmap.insert(Rbrace, Shift(85));
    table.push(hashmap);

    // for state 85
    let mut hashmap = HashMap::new();
    hashmap.insert(Vtype, Reduce(32));
    hashmap.insert(Id, Reduce(32));
    hashmap.insert(Rbrace, Reduce(32));
    hashmap.insert(If, Reduce(32));
    hashmap.insert(While, Reduce(32));
    hashmap.insert(Return, Reduce(32));
    table.push(hashmap);

    table
}

// 잘 못 입력된 rule이 없는지 검사하는 코드입니다.
// cargo가 설치되어있다면
// $ cargo test
// 를 통해 실행 가능합니다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_number_of_state() {
        let table = get_parsing_table();
        assert_eq!(table.len(), 86);
    }

    #[test]
    fn check_number_of_action_and_goto_for_state0() {
        let table = get_parsing_table();
        assert_eq!(table[0].len(), 7);
    }

    #[test]
    fn check_number_of_action_and_goto_for_state18() {
        let table = get_parsing_table();
        assert_eq!(table[18].len(), 1);
    }

    #[test]
    fn check_number_of_action_and_goto_for_state40() {
        let table = get_parsing_table();
        assert_eq!(table[40].len(), 1);
    }

    #[test]
    fn check_number_of_action_and_goto_for_state85() {
        let table = get_parsing_table();
        assert_eq!(table[85].len(), 6);
    }

    #[test]
    fn check_action_and_goto_for_0_vtype() {
        let table = get_parsing_table();
        assert_eq!(table[0][&Vtype], Shift(5));
    }

    #[test]
    fn check_action_and_goto_for_1_eol() {
        let table = get_parsing_table();
        assert_eq!(table[1][&EOL], Accepted);
    }

    #[test]
    fn check_action_and_goto_for_4_code() {
        let table = get_parsing_table();
        assert_eq!(table[4][&CODE], Goto(9));
    }

    #[test]
    fn check_action_and_goto_for_15_expr__() {
        let table = get_parsing_table();
        assert_eq!(table[15][&EXPR__], Goto(26));
    }

    #[test]
    fn check_action_and_goto_for_32_rbrace() {
        let table = get_parsing_table();
        assert_eq!(table[32][&Rbrace], Reduce(38));
    }

    #[test]
    fn check_action_and_goto_for_41_id() {
        let table = get_parsing_table();
        assert_eq!(table[41][&Id], Shift(54));
    }

    #[test]
    fn check_action_and_goto_for_46_rparen() {
        let table = get_parsing_table();
        assert_eq!(table[46][&Rparen], Reduce(16));
    }

    #[test]
    fn check_action_and_goto_for_85_while() {
        let table = get_parsing_table();
        assert_eq!(table[85][&While], Reduce(32));
    }

    #[test]
    fn check_length_of_reduction_table() {
        let table = get_reduction_table();
        assert_eq!(table.len(), 39);
    }
}
