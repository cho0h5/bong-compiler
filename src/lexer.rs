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
                Token::AddMinus
            }
            Some('-') => {
                char = iter.next();
                Token::AddMinus
            }
            Some('~') => {
                char = iter.next();
                Token::UnaryOp
            }
            Some('/') => {
                char = iter.next();
                Token::MulOp
            }
            Some('%') => {
                char = iter.next();
                Token::MulOp
            }
            Some('^') => {
                char = iter.next();
                Token::AddOp
            }
            Some('=') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp
                    }
                    _ => Token::AssignOp,
                }
            }
            Some('&') => {
                char = iter.next();
                match char {
                    Some('&') => {
                        char = iter.next();
                        Token::LogOp
                    }
                    _ => Token::And,
                }
            }
            Some('|') => {
                char = iter.next();
                match char {
                    Some('|') => {
                        char = iter.next();
                        Token::LogOp
                    }
                    _ => Token::AddOp,
                }
            }
            Some('!') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp
                    }
                    _ => Token::UnaryOp,
                }
            }
            Some('<') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp
                    }
                    Some('<') => {
                        char = iter.next();
                        Token::MulOp
                    }
                    _ => Token::RelOp,
                }
            }
            Some('>') => {
                char = iter.next();
                match char {
                    Some('=') => {
                        char = iter.next();
                        Token::RelOp
                    }
                    Some('>') => {
                        char = iter.next();
                        Token::MulOp
                    }
                    _ => Token::RelOp,
                }
            }
            Some('\"') => {
                char = iter.next();
                loop {
                    match char {
                        Some('\"') => {
                            char = iter.next();
                            temp.clear();
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
                char = iter.next();
                loop {
                    match char {
                        Some('\'') => {
                            char = iter.next();
                            temp.clear();
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
            Some(c) if c.is_alphanumeric() => {
                temp.push(c);
                char = iter.next();
                loop {
                    match char {
                        Some(c) if c.is_alphanumeric() => {
                            temp.push(c);
                            char = iter.next();
                        }
                        _ => break,
                    }
                }
                Token::Identifier
            }
            Some(c) if c.is_ascii_whitespace() => {
                char = iter.next();
                continue;
            }
            Some(unexpected_char) => panic!("unexpected_char: {}", unexpected_char),
            _ => break,
        };
        tokens.push_back(Terminal(token));
    }

    //    for word in contents.split_whitespace() {
    //        let token = match word {
    //            "int" => Token::Int,
    //            "void" => Token::Void,
    //            "if" => Token::If,
    //            "while" => Token::While,
    //            "return" => Token::Return,
    //            "break" => Token::Break,
    //            "continue" => Token::Continue,
    //
    //            "identifier" => Token::Identifier, // TODO : 문자로 시작
    //            unknown_token => return Err(UnknownTokenError(unknown_token)),
    //        };
    //        tokens.push_back(Terminal(token));
    //    }

    tokens.push_back(Terminal(Token::EOL));
    Ok(Tokens(tokens))
}
