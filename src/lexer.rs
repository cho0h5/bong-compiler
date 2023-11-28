use std::collections::VecDeque;

use crate::parser::formatting::Tokens;
use crate::parser::Node::Terminal;

pub struct UnknownTokenError<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LogicalOperator {
    Or,  // ||
    And, // &&
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RelativeOperator {
    Equal,        // =
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AdditiveOperator {
    BitwiseOr,  // |
    BitwiseAnd, // &
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MultiplicativeOperator {
    Div,        // /
    Mod,        // %
    LeftShift,  // <<
    RightShift, // >>
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Not,        // !
    BitwiseNot, // ~
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AddMinusOperator {
    Add,   // +
    Minus, // -
}

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
    LogOp(LogicalOperator),
    RelOp(RelativeOperator),
    AddOp(AdditiveOperator),
    AddMinus(AddMinusOperator),
    MulOp(MultiplicativeOperator),
    And,
    UnaryOp(UnaryOperator),
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

    let mut temp = String::new();
    let mut iter = contents.chars();
    let mut char = iter.next();
    loop {
        let token = match char {
            Some('[') => {
                char = iter.next();
                Token::Lbracket
            }
            Some(']') => {
                char = iter.next();

                Token::Rbracket
            }
            Some('*') => {
                char = iter.next();

                Token::Star
            }
            Some('{') => {
                char = iter.next();

                Token::Lbrace
            }
            Some('}') => {
                char = iter.next();

                Token::Rbrace
            }
            Some(';') => {
                char = iter.next();

                Token::Semicolon
            }
            Some('(') => {
                char = iter.next();
                Token::Lparen
            }
            Some(')') => {
                char = iter.next();
                Token::Rparen
            }
            Some(',') => {
                char = iter.next();
                Token::Comma
            }
            Some('+') => {
                char = iter.next();
                Token::AddMinus(AddMinusOperator::Add)
            }
            Some('-') => {
                char = iter.next();
                Token::AddMinus(AddMinusOperator::Minus)
            }
            Some('~') => {
                char = iter.next();
                Token::UnaryOp(UnaryOperator::BitwiseNot)
            }
            Some('/') => {
                char = iter.next();
                Token::MulOp(MultiplicativeOperator::Div)
            }
            Some('%') => {
                char = iter.next();
                Token::MulOp(MultiplicativeOperator::Mod)
            }
            Some('^') => {
                char = iter.next();
                Token::AddOp(AdditiveOperator::BitwiseAnd)
            }
            Some('=') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp(RelativeOperator::Equal)
                    }
                    _ => Token::AssignOp,
                }
            }
            Some('&') => {
                char = iter.next();
                match char {
                    Some('&') => {
                        char = iter.next();
                        Token::LogOp(LogicalOperator::And)
                    }
                    _ => Token::And,
                }
            }
            Some('|') => {
                char = iter.next();
                match char {
                    Some('|') => {
                        char = iter.next();
                        Token::LogOp(LogicalOperator::Or)
                    }
                    _ => Token::AddOp(AdditiveOperator::BitwiseOr),
                }
            }
            Some('!') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp(RelativeOperator::NotEqual)
                    }
                    _ => Token::UnaryOp(UnaryOperator::Not),
                }
            }
            Some('<') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp(RelativeOperator::LessEqual)
                    }
                    Some('<') => {
                        char = iter.next();
                        Token::MulOp(MultiplicativeOperator::LeftShift)
                    }
                    _ => Token::RelOp(RelativeOperator::Less),
                }
            }
            Some('>') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp(RelativeOperator::GreaterEqual)
                    }
                    Some('>') => {
                        char = iter.next();
                        Token::MulOp(MultiplicativeOperator::RightShift)
                    }
                    _ => Token::RelOp(RelativeOperator::Greater),
                }
            }
            Some('\"') => {
                temp.clear();
                char = iter.next();
                loop {
                    match char {
                        Some('\"') => {
                            char = iter.next();
                            break;
                        }
                        Some(c) => {
                            temp.push(c);
                            char = iter.next();
                        }
                        _ => panic!("lexer error"),
                    }
                }
                Token::StringLit
            }
            Some('\'') => {
                temp.clear();
                char = iter.next();
                loop {
                    match char {
                        Some('\'') => {
                            char = iter.next();
                            break;
                        }
                        Some(c) => {
                            temp.push(c);
                            char = iter.next();
                        }
                        _ => panic!("lexer error"),
                    }
                }
                Token::IntLit
            }
            Some(c) if c.is_numeric() => {
                temp.clear();
                temp.push(c);
                char = iter.next();
                loop {
                    match char {
                        Some(c) if c.is_numeric() => {
                            temp.push(c);
                            char = iter.next();
                        }
                        _ => break,
                    }
                }
                Token::IntLit
            }
            Some(c) if c.is_alphanumeric() || c == '_' => {
                temp.clear();
                temp.push(c);
                char = iter.next();
                loop {
                    match char {
                        Some(c) if c.is_alphanumeric() || c == '_' => {
                            temp.push(c);
                            char = iter.next();
                        }
                        _ => break,
                    }
                }
                println!("keyward: \"{}\"", temp);
                match temp.as_str() {
                    "int" => Token::Int,
                    "void" => Token::Void,
                    "if" => Token::If,
                    "while" => Token::While,
                    "return" => Token::Return,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    _ => Token::Identifier,
                }
            }
            Some(c) if c.is_whitespace() => {
                char = iter.next();
                continue;
            }
            Some(unexpected_char) => panic!("unexpected_char: {}", unexpected_char),
            _ => break,
        };
        tokens.push_back(Terminal(token));
    }

    tokens.push_back(Terminal(Token::EOL));
    Ok(Tokens(tokens))
}
