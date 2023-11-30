use core::panic;

use crate::instruction::*;
use crate::parser::formatting::Tree;
use crate::parser::Node;
use crate::{lexer::*, symbol_table::*};

pub fn generate_code(tree: &Tree, symbol_table: &SymbolTable) -> Vec<Box<dyn Instruction>> {
    let mut generated_code = Vec::new();

    traverse_tree(&mut generated_code, symbol_table, &tree.0);

    generated_code
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
        code.extend(generate_block(children));
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
    let destination_memory_offset = -(4 * count);

    code.push(Box::new(IFormat::new(
        OpCode::Sw,
        source_register,
        RegisterName::SP,
        destination_memory_offset,
    )));

    code
}

fn generate_block(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[1] {
        code.extend(generate_statement_list(children));
    }

    code
}

fn generate_statement_list(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if !children.is_empty() {
        if let Node::NonTerminal(Token::STATEMENT, children) = &children[0] {
            code.extend(generate_statement(children));
        }

        if children.len() >= 3 {
            if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[2] {
                code.extend(generate_statement_list(children));
            }
        }
    }

    code
}

fn generate_statement(children: &[Node]) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    match &children[0] {
        Node::NonTerminal(Token::ASSIGNMENT, children) => {
            code.extend(generate_assignment(children));
        }
        Node::NonTerminal(Token::VAR_DECL, _) => (),
        Node::NonTerminal(Token::RETURN_STMT, children) => {
            code.extend(generate_return_stmt(children));
        }
        Node::NonTerminal(Token::BREAK_STMT, children) => {
            code.extend(generate_break_stmt(children));
        }
        Node::NonTerminal(Token::CONTINUE_STMT, children) => {
            code.extend(generate_continue_stmt(children));
        }
        Node::NonTerminal(Token::IF_STMT, children) => {
            code.extend(generate_if_stmt(children));
        }
        Node::NonTerminal(Token::WHILE_STMT, children) => {
            code.extend(generate_while_stmt(children));
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

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_return_stmt(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_break_stmt(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_continue_stmt(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_if_stmt(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_while_stmt(children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

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
            if let Node::NonTerminal(Token::LOGICAL_EXPR, children) = &children[2] {
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
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

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
        child1_offset,
    )));
    code.push(Box::new(IFormat::new(
        OpCode::Lw,
        RegisterName::T0,
        RegisterName::T2,
        child2_offset,
    )));

    code
}

fn generate_store_t1_to_offset(offset: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    code.push(Box::new(IFormat::new(
        OpCode::Lui,
        RegisterName::Zero,
        RegisterName::T0,
        4096,
    )));

    code.push(Box::new(IFormat::new(
        OpCode::Sw,
        RegisterName::T0,
        RegisterName::T1,
        offset,
    )));

    code
}

fn generate_move(this_offset: i16, child_offset: i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

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

fn generate_relational_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    let this_offset = *offset;
    *offset += 4;
    let child_offset = *offset;

    // match &children[0] {}

    code
}

fn generate_additive_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_multiplicative_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_unary_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_primary_expr(children: &[Node], offset: &mut i16) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}
