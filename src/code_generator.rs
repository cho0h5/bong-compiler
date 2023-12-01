use core::panic;
use std::boxed;

use crate::instruction::*;
use crate::parser::formatting::Tree;
use crate::parser::Node;
use crate::{lexer::*, symbol_table::*};

pub fn generate_code(tree: &Tree, symbol_table: &SymbolTable) -> Vec<Box<dyn Instruction>> {
    let mut generated_code: Vec<Box<dyn Instruction>> = vec![
        Box::new(JFormat::new_label(OpCode::Jal, "bong".to_string())),
        Box::new(JFormat::new_label(OpCode::Jump, "exit".to_string())),
    ];

    traverse_tree(&mut generated_code, symbol_table, &tree.0);
    generated_code.extend(generate_print_int());
    generated_code.extend(generate_print_char());
    generated_code.extend(generate_exit());

    generated_code
}

fn generate_print_int() -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::label_new(
            "print_int",
            OpCode::Addi,
            RegisterName::Zero,
            RegisterName::V0,
            1,
        )),
        Box::new(RFormat::new(
            Funct::Syscall,
            RegisterName::Zero,
            RegisterName::Zero,
            RegisterName::Zero,
            0,
        )),
        Box::new(RFormat::new(
            Funct::Jr,
            RegisterName::RA,
            RegisterName::Zero,
            RegisterName::Zero,
            0,
        )),
    ];

    code
}

fn generate_print_char() -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::label_new(
            "print_char",
            OpCode::Addi,
            RegisterName::Zero,
            RegisterName::V0,
            11,
        )),
        Box::new(RFormat::new(
            Funct::Syscall,
            RegisterName::Zero,
            RegisterName::Zero,
            RegisterName::Zero,
            0,
        )),
        Box::new(RFormat::new(
            Funct::Jr,
            RegisterName::RA,
            RegisterName::Zero,
            RegisterName::Zero,
            0,
        )),
    ];

    code
}

fn generate_exit() -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::label_new(
            "exit",
            OpCode::Addi,
            RegisterName::Zero,
            RegisterName::V0,
            10,
        )),
        Box::new(RFormat::new(
            Funct::Syscall,
            RegisterName::Zero,
            RegisterName::Zero,
            RegisterName::Zero,
            0,
        )),
    ];

    code
}

fn traverse_tree(
    generated_code: &mut Vec<Box<dyn Instruction>>,
    symbol_table: &SymbolTable,
    node: &Node,
) {
    match node {
        Node::NonTerminal(Token::FUNCTION_DECL, children) => {
            generated_code.extend(generate_function_decl(symbol_table, children));
        }
        Node::NonTerminal(Token::PROGRAM, children) => {
            for child in children {
                traverse_tree(generated_code, symbol_table, child);
            }
        }
        token => panic!("unexpected token: {:?}", token),
    }
}

fn traverse_debug(node: &Node) {
    match node {
        Node::NonTerminal(Token::PARAMETERS, _) => (),
        Node::Terminal(token) => {
            println!("{:?}", token)
        }
        Node::NonTerminal(token, children) => {
            println!("{:?}", token);
            for child in children {
                traverse_debug(child);
            }
        }
    }
}

fn generate_function_decl(
    symbol_table: &SymbolTable,
    children: &[Node],
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let func_name = match &children[1] {
        Node::Terminal(Token::Identifier(name, _)) => name,
        _ => panic!("no identifier?"),
    };
    let func_size = symbol_table.get_function_size(func_name) as i16;

    code.push(Box::new(IFormat::label_new(
        func_name,
        OpCode::Addi,
        RegisterName::SP,
        RegisterName::SP,
        -func_size,
    )));

    if let Node::NonTerminal(Token::PARAMETERS, children) = &children[2] {
        code.extend(generate_parameters(children));
    }

    if let Node::NonTerminal(Token::BLOCK, children) = &children[3] {
        code.extend(generate_block(children, func_size, "exit", "exit"));
    }

    code.push(Box::new(IFormat::new(
        OpCode::Addi,
        RegisterName::SP,
        RegisterName::SP,
        func_size,
    )));

    code.push(Box::new(RFormat::new(
        Funct::Jr,
        RegisterName::RA,
        RegisterName::Zero,
        RegisterName::Zero,
        0,
    )));

    code
}

fn generate_parameters(children: &[Node]) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if let Node::NonTerminal(Token::PARAMETER_LIST, children) = &children[1] {
        code.extend(generate_parameter_list(children, 0));
    }

    code
}
fn generate_parameter_list(children: &[Node], count: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    if !children.is_empty() {
        if let Node::NonTerminal(Token::PARAMETER_DECL, _) = &children[0] {
            code.extend(generate_parameter_decl(count));
        }
        if children.len() >= 3 {
            if let Node::NonTerminal(Token::PARAMETER_LIST, children) = &children[2] {
                code.extend(generate_parameter_list(children, count + 1));
            }
        }
    }
    code
}

fn generate_parameter_decl(count: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    let source_register = match count {
        0 => RegisterName::A0,
        1 => RegisterName::A1,
        2 => RegisterName::A2,
        3 => RegisterName::A3,
        _ => panic!(
            "function parameter must be less than 4 (current: {})",
            count
        ),
    };
    let destination_memory_offset = 4 * count;

    code.push(Box::new(IFormat::new(
        OpCode::Sw,
        RegisterName::SP,
        source_register,
        destination_memory_offset,
    )));

    code
}

fn generate_block(
    children: &Vec<Node>,
    func_size: i16,
    start_label: &str,
    end_label: &str,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[1] {
        code.extend(generate_statement_list(
            children,
            func_size,
            start_label,
            end_label,
        ));
    }

    code
}

fn generate_statement_list(
    children: &Vec<Node>,
    func_size: i16,
    start_label: &str,
    end_label: &str,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if !children.is_empty() {
        if let Node::NonTerminal(Token::STATEMENT, children) = &children[0] {
            code.extend(generate_statement(
                children,
                func_size,
                start_label,
                end_label,
            ));
        }

        if children.len() >= 3 {
            if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[2] {
                code.extend(generate_statement_list(
                    children,
                    func_size,
                    start_label,
                    end_label,
                ));
            }
        }
    }

    code
}

fn generate_statement(
    children: &[Node],
    func_size: i16,
    start_label: &str,
    end_label: &str,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    match &children[0] {
        Node::NonTerminal(Token::ASSIGNMENT, children) => {
            code.extend(generate_assignment(children));
        }
        Node::NonTerminal(Token::VAR_DECL, _) => (),
        Node::NonTerminal(Token::RETURN_STMT, children) => {
            code.extend(generate_return_stmt(children, func_size));
        }
        Node::NonTerminal(Token::BREAK_STMT, children) => {
            code.extend(generate_break_stmt(children, end_label));
        }
        Node::NonTerminal(Token::CONTINUE_STMT, children) => {
            code.extend(generate_continue_stmt(children, start_label));
        }
        Node::NonTerminal(Token::IF_STMT, children) => {
            code.extend(generate_if_stmt(children, func_size));
        }
        Node::NonTerminal(Token::WHILE_STMT, children) => {
            code.extend(generate_while_stmt(children, func_size));
        }
        Node::NonTerminal(Token::EXPRESSION, children) => {
            let mut offset = 0;
            code.extend(generate_expression(children, &mut offset));
        }
        node => panic!("unvalid parse tree: {:?}", node),
    }

    code
}

fn generate_assignment(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    // 변수 offset 가져오기
    let var_offset = match &children[0] {
        Node::NonTerminal(Token::EXPRESSION, children1) => match &children1[0] {
            Node::NonTerminal(Token::LOGICAL_EXPR, children2) => match &children2[0] {
                Node::NonTerminal(Token::RELATIONAL_EXPR, children3) => match &children3[0] {
                    Node::NonTerminal(Token::ADDITIVE_EXPR, children4) => match &children4[0] {
                        Node::NonTerminal(Token::MULTIPLICATIVE_EXPR, children5) => {
                            match &children5[0] {
                                Node::NonTerminal(Token::UNARY_EXPR, children6) => {
                                    match &children6[0] {
                                        Node::NonTerminal(Token::PRIMARY_EXPR, children7) => {
                                            match &children7[0] {
                                                Node::NonTerminal(Token::OPERAND, children8) => {
                                                    match &children8[0] {
                                                        Node::Terminal(Token::Identifier(
                                                            _,
                                                            Some(Address::Offset(offset)),
                                                        )) => offset,
                                                        _ => panic!("no way"),
                                                    }
                                                }
                                                _ => panic!("no way"),
                                            }
                                        }
                                        _ => panic!("no way"),
                                    }
                                }
                                _ => panic!("no way"),
                            }
                        }
                        _ => panic!("no way"),
                    },
                    _ => panic!("no way"),
                },
                _ => panic!("no way"),
            },
            _ => panic!("no way"),
        },
        _ => panic!("no way"),
    };

    // evaluate expression
    let mut offset = 0;
    if let Node::NonTerminal(Token::EXPRESSION, children) = &children[2] {
        code.extend(generate_expression(children, &mut offset));
    } else {
        panic!("no way");
    }

    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T1,
        0,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Sw,
        RegisterName::SP,
        RegisterName::T1,
        *var_offset as i16,
    )));

    code
}

fn generate_return_stmt(children: &Vec<Node>, func_size: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let mut offset = 0;

    if let Node::NonTerminal(Token::EXPRESSION, children) = &children[1] {
        code.extend(generate_expression(children, &mut offset));
    } else {
        panic!("no way");
    }

    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T1,
        0,
    )));
    code.push(Box::new(RFormat::new(
        Funct::Add,
        RegisterName::T1,
        RegisterName::Zero,
        RegisterName::V0,
        0,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Addi,
        RegisterName::SP,
        RegisterName::SP,
        func_size,
    )));

    code.push(Box::new(RFormat::new(
        Funct::Jr,
        RegisterName::RA,
        RegisterName::Zero,
        RegisterName::Zero,
        0,
    )));

    code
}

fn generate_break_stmt(children: &Vec<Node>, end_label: &str) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = vec![Box::new(JFormat::new_label(
        OpCode::Jump,
        end_label.to_string(),
    ))];
    code
}

fn generate_continue_stmt(children: &Vec<Node>, start_label: &str) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = vec![Box::new(JFormat::new_label(
        OpCode::Jump,
        start_label.to_string(),
    ))];
    code
}

fn generate_if_stmt(children: &Vec<Node>, func_size: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    let if_label = match &children[0] {
        Node::Terminal(Token::If(label)) => label,
        _ => panic!("no way: {:?}", children[0]),
    };
    let mut ifend_label = if_label.to_string();
    ifend_label.push_str("end");

    code.push(Box::new(RFormat::label_new(
        if_label,
        Funct::Add,
        RegisterName::T0,
        RegisterName::Zero,
        RegisterName::T0,
        0,
    )));

    let mut offset = 0;
    if let Node::NonTerminal(Token::EXPRESSION, children) = &children[2] {
        code.extend(generate_expression(children, &mut offset));
    } else {
        panic!("no way");
    }
    // expression 평가 결과를 T1으로 가져옴
    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T1,
        0,
    )));

    // beq
    code.push(Box::new(IFormat::new_label(
        OpCode::Beq,
        RegisterName::T1,
        RegisterName::Zero,
        ifend_label.clone(),
    )));

    // block
    if let Node::NonTerminal(Token::BLOCK, children) = &children[4] {
        code.extend(generate_block(children, func_size, &if_label, &ifend_label));
    } else {
        panic!("no way");
    }

    code.push(Box::new(RFormat::label_new(
        &ifend_label,
        Funct::Add,
        RegisterName::T0,
        RegisterName::Zero,
        RegisterName::T0,
        0,
    )));

    code
}

fn generate_while_stmt(children: &Vec<Node>, func_size: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    let while_label = match &children[0] {
        Node::Terminal(Token::While(label)) => label,
        _ => panic!("no way: {:?}", children[0]),
    };
    let mut whileend_label = while_label.to_string();
    whileend_label.push_str("end");

    code.push(Box::new(RFormat::label_new(
        while_label,
        Funct::Add,
        RegisterName::T0,
        RegisterName::Zero,
        RegisterName::T0,
        0,
    )));

    let mut offset = 0;
    if let Node::NonTerminal(Token::EXPRESSION, children) = &children[2] {
        code.extend(generate_expression(children, &mut offset));
    } else {
        panic!("no way");
    }

    // expression 평가 결과를 T1으로 가져옴
    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T1,
        0,
    )));
    // beq
    code.push(Box::new(IFormat::new_label(
        OpCode::Beq,
        RegisterName::T1,
        RegisterName::Zero,
        whileend_label.clone(),
    )));

    // block
    if let Node::NonTerminal(Token::BLOCK, children) = &children[4] {
        code.extend(generate_block(
            children,
            func_size,
            &while_label,
            &whileend_label,
        ));
    } else {
        panic!("no way");
    }
    code.push(Box::new(JFormat::new_label(
        OpCode::Jump,
        while_label.to_string(),
    )));

    code.push(Box::new(RFormat::label_new(
        &whileend_label,
        Funct::Add,
        RegisterName::T0,
        RegisterName::Zero,
        RegisterName::T0,
        0,
    )));

    code
}

fn generate_expression(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    let child_offset = *offset + 4;
    *offset += 4;

    if let Node::NonTerminal(Token::LOGICAL_EXPR, children) = &children[0] {
        code.extend(generate_logical_expr(children, offset))
    }

    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T1,
        child_offset,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Sw,
        RegisterName::T0,
        RegisterName::T1,
        this_offset,
    )));

    code
}

fn generate_logical_expr(children: &[Node], mut offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::NonTerminal(Token::LOGICAL_EXPR, children2) => {
            code.extend(generate_logical_expr(children2, offset));

            let child2_offset = *offset;
            if let Node::NonTerminal(Token::RELATIONAL_EXPR, children) = &children[2] {
                code.extend(generate_relational_expr(children, offset));
            }
            code.extend(generate_load_2children_to_t1_t2(
                child_offset,
                child2_offset,
            ));

            match children[1] {
                // TODO bitwise or 말고 logical or로 바꿔야함
                Node::Terminal(Token::LogOp(LogicalOperator::Or)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Or,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                // TODO bitwise and 말고 logical and로 바꿔야함
                Node::Terminal(Token::LogOp(LogicalOperator::And)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::And,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                _ => panic!("no way"),
            }
            code.extend(generate_store_t1_to_offset(this_offset));
        }
        Node::NonTerminal(Token::RELATIONAL_EXPR, children) => {
            code.extend(generate_relational_expr(children, offset));
            code.extend(generate_move(this_offset, child_offset));
        }
        _ => (),
    }

    code
}

fn generate_load_2children_to_t1_t2(
    child1_offset: i16,
    child2_offset: i16,
) -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::new(
            OpCode::Lui,
            RegisterName::Zero,
            RegisterName::T0,
            4096,
        )),
        Box::new(IFormat::new(
            OpCode::Lw,
            RegisterName::T0,
            RegisterName::T1,
            child1_offset,
        )),
        Box::new(IFormat::new(
            OpCode::Lw,
            RegisterName::T0,
            RegisterName::T2,
            child2_offset,
        )),
    ];

    code
}

fn generate_store_t1_to_offset(offset: i16) -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::new(
            OpCode::Lui,
            RegisterName::Zero,
            RegisterName::T0,
            4096,
        )),
        Box::new(IFormat::new(
            OpCode::Sw,
            RegisterName::T0,
            RegisterName::T1,
            offset,
        )),
    ];

    code
}

fn generate_move(this_offset: i16, child_offset: i16) -> Vec<Box<dyn Instruction>> {
    let code: Vec<Box<dyn Instruction>> = vec![
        Box::new(IFormat::new(
            OpCode::Lui,
            RegisterName::Zero,
            RegisterName::T0,
            4096,
        )),
        Box::new(IFormat::new(
            OpCode::Lw,
            RegisterName::T0,
            RegisterName::T1,
            child_offset,
        )),
        Box::new(IFormat::new(
            OpCode::Sw,
            RegisterName::T0,
            RegisterName::T1,
            this_offset,
        )),
    ];

    code
}

fn generate_relational_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::NonTerminal(Token::RELATIONAL_EXPR, children2) => {
            code.extend(generate_relational_expr(children2, offset));

            let child2_offset = *offset;
            if let Node::NonTerminal(Token::ADDITIVE_EXPR, children) = &children[2] {
                code.extend(generate_additive_expr(children, offset));
            }
            code.extend(generate_load_2children_to_t1_t2(
                child_offset,
                child2_offset,
            ));

            match children[1] {
                Node::Terminal(Token::RelOp(RelativeOperator::Equal)) => {
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        0,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Bne,
                        RegisterName::T1,
                        RegisterName::T2,
                        1,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        1,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T3,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::RelOp(RelativeOperator::NotEqual)) => {
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        0,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Beq,
                        RegisterName::T1,
                        RegisterName::T2,
                        1,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        1,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T3,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::RelOp(RelativeOperator::Less)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Slt,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::RelOp(RelativeOperator::LessEqual)) => {
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        0,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Slt,
                        RegisterName::T2,
                        RegisterName::T1,
                        RegisterName::T1,
                        0,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Bne,
                        RegisterName::T1,
                        RegisterName::Zero,
                        1,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        1,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T3,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::RelOp(RelativeOperator::Greater)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Slt,
                        RegisterName::T2,
                        RegisterName::T1,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::RelOp(RelativeOperator::GreaterEqual)) => {
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        0,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Slt,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Bne,
                        RegisterName::T1,
                        RegisterName::Zero,
                        1,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        1,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T3,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                _ => panic!("no way"),
            }
            code.extend(generate_store_t1_to_offset(this_offset));
        }
        Node::NonTerminal(Token::ADDITIVE_EXPR, children) => {
            code.extend(generate_additive_expr(children, offset));
            code.extend(generate_move(this_offset, child_offset));
        }
        _ => (),
    }

    code
}

fn generate_additive_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::NonTerminal(Token::ADDITIVE_EXPR, children2) => {
            code.extend(generate_additive_expr(children2, offset));

            let child2_offset = *offset;
            if let Node::NonTerminal(Token::MULTIPLICATIVE_EXPR, children) = &children[2] {
                code.extend(generate_multiplicative_expr(children, offset));
            }
            code.extend(generate_load_2children_to_t1_t2(
                child_offset,
                child2_offset,
            ));

            match children[1] {
                Node::Terminal(Token::AddMinus(AddMinusOperator::Add)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::AddMinus(AddMinusOperator::Minus)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Sub,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::AddOp(AdditiveOperator::BitwiseOr)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Or,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::AddOp(AdditiveOperator::BitwiseAnd)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::And,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                _ => panic!("no way"),
            }
            code.extend(generate_store_t1_to_offset(this_offset));
        }
        Node::NonTerminal(Token::MULTIPLICATIVE_EXPR, children) => {
            code.extend(generate_multiplicative_expr(children, offset));
            code.extend(generate_move(this_offset, child_offset));
        }
        _ => (),
    }

    code
}

fn generate_multiplicative_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::NonTerminal(Token::MULTIPLICATIVE_EXPR, children2) => {
            code.extend(generate_multiplicative_expr(children2, offset));

            let child2_offset = *offset;
            if let Node::NonTerminal(Token::UNARY_EXPR, children) = &children[2] {
                code.extend(generate_unary_expr(children, offset));
            }
            code.extend(generate_load_2children_to_t1_t2(
                child_offset,
                child2_offset,
            ));

            match children[1] {
                Node::Terminal(Token::MulOp(MultiplicativeOperator::Div)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Div,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::Zero,
                        0,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Mflo,
                        RegisterName::Zero,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::MulOp(MultiplicativeOperator::Mod)) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Div,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::Zero,
                        0,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Mflo,
                        RegisterName::Zero,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::MulOp(MultiplicativeOperator::LeftShift)) => {
                    panic!("shift is not implemented")
                }
                Node::Terminal(Token::MulOp(MultiplicativeOperator::RightShift)) => {
                    panic!("shift is not implemented")
                }
                Node::Terminal(Token::Star) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Mult,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::Zero,
                        0,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Mflo,
                        RegisterName::Zero,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Node::Terminal(Token::And) => {
                    code.push(Box::new(RFormat::new(
                        Funct::And,
                        RegisterName::T1,
                        RegisterName::T2,
                        RegisterName::T1,
                        0,
                    )));
                }
                _ => panic!("no way"),
            }
            code.extend(generate_store_t1_to_offset(this_offset));
        }
        Node::NonTerminal(Token::UNARY_EXPR, children) => {
            code.extend(generate_unary_expr(children, offset));
            code.extend(generate_move(this_offset, child_offset));
        }
        _ => (),
    }

    code
}

fn generate_unary_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::NonTerminal(Token::PRIMARY_EXPR, children) => {
            code.extend(generate_primary_expr(children, offset));
            code.extend(generate_move(this_offset, child_offset));
        }
        Node::Terminal(token) => {
            if let Node::NonTerminal(Token::UNARY_EXPR, children2) = &children[1] {
                code.extend(generate_unary_expr(children2, offset));
            } else {
                panic!("no way");
            }
            code.push(Box::new(IFormat::new(
                OpCode::Lui,
                RegisterName::Zero,
                RegisterName::T0,
                4096,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Lw,
                RegisterName::T0,
                RegisterName::T1,
                child_offset,
            )));

            match token {
                Token::UnaryOp(UnaryOperator::Not) => {
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        0,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Bne,
                        RegisterName::T1,
                        RegisterName::Zero,
                        1,
                    )));
                    code.push(Box::new(IFormat::new(
                        OpCode::Addi,
                        RegisterName::Zero,
                        RegisterName::T3,
                        1,
                    )));
                    code.push(Box::new(RFormat::new(
                        Funct::Add,
                        RegisterName::T3,
                        RegisterName::Zero,
                        RegisterName::T1,
                        0,
                    )));
                }
                Token::UnaryOp(UnaryOperator::BitwiseNot) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Nor,
                        RegisterName::T1,
                        RegisterName::T1,
                        RegisterName::T1,
                        0,
                    )));
                }
                Token::Star => unimplemented!(),
                Token::And => unimplemented!(),
                Token::AddMinus(AddMinusOperator::Add) => {}
                Token::AddMinus(AddMinusOperator::Minus) => {
                    code.push(Box::new(RFormat::new(
                        Funct::Sub,
                        RegisterName::Zero,
                        RegisterName::T1,
                        RegisterName::T1,
                        0,
                    )));
                }
                _ => panic!("no way"),
            }
            code.extend(generate_store_t1_to_offset(this_offset));
        }
        _ => panic!("no way"),
    }

    code
}

fn generate_primary_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    if children.len() == 2 {
        if let Node::NonTerminal(Token::INDEX, _) = children[1] {
            unimplemented!()
        }
        if let Node::NonTerminal(Token::ARGUMENTS, children2) = &children[1] {
            let mut offsets = Vec::new();
            if let Node::NonTerminal(Token::EXPRESSION_LIST, children3) = &children2[1] {
                code.extend(generate_expression_list(children3, offset, &mut offsets));
            }
            let label: String = match &children[0] {
                Node::NonTerminal(Token::PRIMARY_EXPR, children) => match &children[0] {
                    Node::NonTerminal(Token::OPERAND, children) => match &children[0] {
                        Node::Terminal(Token::Identifier(id, _)) => id.to_string(),
                        _ => panic!("no way"),
                    },
                    _ => panic!("no way"),
                },
                _ => panic!("no way"),
            };
            if let Some(arg1_offset) = offsets.first() {
                code.push(Box::new(IFormat::new(
                    OpCode::Lui,
                    RegisterName::Zero,
                    RegisterName::T0,
                    4096,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Lw,
                    RegisterName::T0,
                    RegisterName::T1,
                    *arg1_offset,
                )));
                code.push(Box::new(RFormat::new(
                    Funct::Add,
                    RegisterName::T1,
                    RegisterName::Zero,
                    RegisterName::A0,
                    0,
                )));
            }
            if let Some(arg2_offset) = offsets.get(1) {
                code.push(Box::new(IFormat::new(
                    OpCode::Lui,
                    RegisterName::Zero,
                    RegisterName::T0,
                    4096,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Lw,
                    RegisterName::T0,
                    RegisterName::T1,
                    *arg2_offset,
                )));
                code.push(Box::new(RFormat::new(
                    Funct::Add,
                    RegisterName::T1,
                    RegisterName::Zero,
                    RegisterName::A1,
                    0,
                )));
            }
            if let Some(arg3_offset) = offsets.get(2) {
                code.push(Box::new(IFormat::new(
                    OpCode::Lui,
                    RegisterName::Zero,
                    RegisterName::T0,
                    4096,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Lw,
                    RegisterName::T0,
                    RegisterName::T1,
                    *arg3_offset,
                )));
                code.push(Box::new(RFormat::new(
                    Funct::Add,
                    RegisterName::T1,
                    RegisterName::Zero,
                    RegisterName::A2,
                    0,
                )));
            }
            if let Some(arg4_offset) = offsets.get(3) {
                code.push(Box::new(IFormat::new(
                    OpCode::Lui,
                    RegisterName::Zero,
                    RegisterName::T0,
                    4096,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Lw,
                    RegisterName::T0,
                    RegisterName::T1,
                    *arg4_offset,
                )));
                code.push(Box::new(RFormat::new(
                    Funct::Add,
                    RegisterName::T1,
                    RegisterName::Zero,
                    RegisterName::A3,
                    0,
                )));
            }

            code.push(Box::new(IFormat::new(
                OpCode::Addi,
                RegisterName::SP,
                RegisterName::SP,
                -4,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Sw,
                RegisterName::SP,
                RegisterName::RA,
                0,
            )));
            code.push(Box::new(JFormat::new_label(OpCode::Jal, label)));
            code.push(Box::new(IFormat::new(
                OpCode::Lw,
                RegisterName::SP,
                RegisterName::RA,
                0,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Addi,
                RegisterName::SP,
                RegisterName::SP,
                4,
            )));

            // 반환값
            code.push(Box::new(IFormat::new(
                OpCode::Lui,
                RegisterName::Zero,
                RegisterName::T0,
                4096,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Sw,
                RegisterName::T0,
                RegisterName::V0,
                this_offset,
            )));
        }
    }
    if let Node::NonTerminal(Token::OPERAND, children) = &children[0] {
        code.extend(generate_operand(children, offset));
        code.extend(generate_move(this_offset, child_offset));
    }

    code
}

fn generate_expression_list(
    children: &[Node],
    offset: &mut i16,
    offsets: &mut Vec<i16>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    if !children.is_empty() {
        if let Node::NonTerminal(Token::EXPRESSION, children) = &children[0] {
            offsets.push(child_offset);
            code.extend(generate_expression(children, offset));
        }
        if children.len() >= 3 {
            if let Node::NonTerminal(Token::EXPRESSION_LIST, children) = &children[2] {
                code.extend(generate_expression_list(children, offset, offsets));
            }
        }
    }

    code
}

fn generate_operand(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    match &children[0] {
        Node::Terminal(Token::StringLit(_)) => unimplemented!(),
        Node::Terminal(Token::IntLit(number)) => {
            code.push(Box::new(IFormat::new(
                OpCode::Lui,
                RegisterName::Zero,
                RegisterName::T0,
                4096,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Lui,
                RegisterName::Zero,
                RegisterName::T1,
                (number >> 16) as i16,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Ori,
                RegisterName::T1,
                RegisterName::T1,
                *number as i16,
            )));
            code.push(Box::new(IFormat::new(
                OpCode::Sw,
                RegisterName::T0,
                RegisterName::T1,
                this_offset,
            )));
        }
        Node::Terminal(Token::Identifier(_, address)) => {
            if let Some(Address::Offset(address)) = address {
                code.push(Box::new(IFormat::new(
                    OpCode::Lui,
                    RegisterName::Zero,
                    RegisterName::T0,
                    4096,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Lw,
                    RegisterName::SP,
                    RegisterName::T1,
                    *address as i16,
                )));
                code.push(Box::new(IFormat::new(
                    OpCode::Sw,
                    RegisterName::T0,
                    RegisterName::T1,
                    this_offset,
                )));
            } else {
                panic!("no way");
            }
        }
        Node::Terminal(Token::Lparen) => {
            if let Node::NonTerminal(Token::EXPRESSION, children1) = &children[1] {
                code.extend(generate_expression(children1, offset));
                code.extend(generate_move(this_offset, child_offset));
            }
        }
        _ => panic!("no way"),
    }

    code
}
