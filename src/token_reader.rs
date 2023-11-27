use std::collections::VecDeque;

use crate::parser::Node::Terminal;
use crate::parser::formatting::Tokens;

// token을 읽는 중에 발생할 수 있는 에러를 나타내는 struct입니다.
// read_tokens(...) 함수는 parsing에 성공하면 Tokens를,
// 실패하면 이 UnknownTokenError를 return합니다.
// 이 struct는 어떤 키워드가 문제인지 &str형의 데이터를 담을 수 있습니다.
pub struct UnknownTokenError<'a>(pub &'a str);

// 토큰을 나타내는 enum입니다.
// 구현의 간결함을 위해 terminal과 non-terminal을 함께 Token으로 정의하였습니다.
// 문자열의 종료를 표현하기 위해 EOL도 추가하였습니다.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Token {
    // terminals
    Vtype,      // for the types of variables and function
    Num,        // for signed integers
    Character,  // for a single character
    Boolstr,    // for Boolean strings
    Literal,    // for literal strings
    Id,         // for the the identifiers of variables and functions
    If,         // for if statements
    Else,       // for else statements
    While,      // for while statements
    Return,     // for return statements
    Class,      // for class declarations
    Addsub,     // for + and - arithmetic
    Multdiv,    // for * and / arithmetic operators
    Assign,     // for assignment operators
    Comp,       // for comparison operators
    Semi,       // for semicolons
    Comma,      // for commas
    Lparen,     // for (
    Rparen,     // for )
    Lbrace,     // for {
    Rbrace,     // for }

    // for EOL
    EOL,

    // non-terminals
    CODE,
    CODE_,
    VDECL,
    ASSIGN,
    RHS,
    EXPR,
    EXPR_,
    EXPR__,
    FDECL,
    ARG,
    MOREARGS,
    BLOCK,
    STMT,
    COND,
    ELSE,
    RETURN,
    CDECL,
    ODECL,
}

// token을 인식하는 함수입니다.
// 이 함수는 String을 받아 whitespace로 split한 후 token을 문자열 비교하여 인식합니다.
// token 인식에 모두 성공하면 Tokens를 return하고, 실패하면 UnknownTokenError를 return합니다.
// Tokens는 src/parser/formatting.rs에 정의되어있으며 그 내용은 다음과 같습니다.
// pub struct Tokens(pub VecDeque<Node>);
// Node는 tree를 나타내기 위한 자료형이며 src/parser/mod.rs에 정의되어있습니다.
// VecDeque는 double-ended queue를 나타내는 rust의 기본 collection입니다.
// read_tokens(...) 함수는 parsing된 token을 뒤에 추가하고, parser::parse(...) 함수는
// token을 앞에서부터 읽기 때문에 VecDeque를 사용하였습니다.
pub fn read_tokens(contents: &String) -> Result<Tokens, UnknownTokenError> {
    let mut tokens = VecDeque::new();

    for word in contents.split_whitespace() {
        let token = match word {
            "vtype" => Token::Vtype,
            "num" => Token::Num,
            "character" => Token::Character,
            "boolstr" => Token::Boolstr,
            "literal" => Token::Literal,
            "id" => Token::Id,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            "class" => Token::Class,
            "addsub" => Token::Addsub,
            "multdiv" => Token::Multdiv,
            "assign" => Token::Assign,
            "comp" => Token::Comp,
            "semi" => Token::Semi,
            "comma" => Token::Comma,
            "lparen" => Token::Lparen,
            "rparen" => Token::Rparen,
            "lbrace" => Token::Lbrace,
            "rbrace" => Token::Rbrace,
            // token 인식을 실패하면 UnknownTokenError에 정보를 담아 return합니다.
            unknown_token => return Err(UnknownTokenError(unknown_token)),
        };
        tokens.push_back(Terminal(token));
    }

    // token을 모두 인식하였다면 마지막에 EOL token을 추가하고 return합니다.
    tokens.push_back(Terminal(Token::EOL));
    Ok(Tokens(tokens))
}
