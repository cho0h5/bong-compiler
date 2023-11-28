use std::collections::VecDeque;

use crate::parser::formatting::Tokens;
use crate::parser::Node::Terminal;

pub struct UnknownTokenError<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Token {
    // terminals
    Int,
    Void,
    Lbracket,
    IntLit,
    Rbracket,
    Star,
    Lbrace,
    Rbrace,
    Semicolon,
    Identifier,
    Lparen,
    Rparen,
    Comma,
    StringLit,
    LogOp,
    RelOp,
    AddOp,
    AddMinus,
    MulOp,
    And,
    UnaryOp,
    AssignOp,
    If,
    While,
    Return,
    Break,
    Continue,

    // for EOL
    EOL,

    // non-terminals
    PROGRAM_,
    PROGRAM,
    TYPE,
    ARRAY_TYPE,
    POINTER_TYPE,
    BLOCK,
    STATEMENT_LIST,
    VAR_DECL,
    FUNCTION_DECL,
    PARAMETERS,
    PARAMETER_LIST,
    PARAMETER_DECL,
    OPERAND,
    PRIMARY_EXPR,
    INDEX,
    ARGUMENTS,
    EXPRESSION_LIST,
    EXPRESSION,
    LOGICAL_EXPR,
    RELATIONAL_EXPR,
    ADDITIVE_EXPR,
    MULTIPLICATIVE_EXPR,
    UNARY_EXPR,
    STATEMENT,
    ASSIGNMENT,
    IF_STMT,
    WHILE_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
}

pub fn read_lexeme(contents: &str) -> Result<Tokens, UnknownTokenError> {
    let mut tokens = VecDeque::new();

    let mut iter = contents.chars();

    for word in contents.split_whitespace() {
        let token = match word {
            "int" => Token::Int,
            "void" => Token::Void,
            "lbracket" => Token::Lbracket,
            "int_lit" => Token::IntLit, // TODO : 숫자로 시작, 또는 작은 따옴표
            "rbracket" => Token::Rbracket,
            "star" => Token::Star,
            "lbrace" => Token::Lbrace,
            "rbrace" => Token::Rbrace,
            "semicolon" => Token::Semicolon,
            "identifier" => Token::Identifier, // TODO : 문자로 시작
            "lparen" => Token::Lparen,
            "rparen" => Token::Rparen,
            "comma" => Token::Comma,
            "string_lit" => Token::StringLit, // TODO : 따옴표 사이
            "log_op" => Token::LogOp,         // TODO:: || &&
            "rel_op" => Token::RelOp,         // TODO : == != < <= > >=
            "add_op" => Token::AddOp,         // TODO : | ^
            "add_minus" => Token::AddMinus,
            "mul_op" => Token::MulOp, // TODO : * / % << >> &
            "and" => Token::And,
            "unary_op" => Token::UnaryOp, // TODO : ! ~
            "assign_op" => Token::AssignOp,
            "if" => Token::If,
            "while" => Token::While,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            unknown_token => return Err(UnknownTokenError(unknown_token)),
        };
        tokens.push_back(Terminal(token));
    }

    tokens.push_back(Terminal(Token::EOL));
    Ok(Tokens(tokens))
}
