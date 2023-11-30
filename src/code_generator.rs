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
        code.extend(generate_block(symbol_table, children));
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

fn generate_block(symbol_table: &SymbolTable, children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[1] {
        code.extend(generate_statement_list(symbol_table, children));
    }

    code
}

fn generate_statement_list(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if !children.is_empty() {
        if let Node::NonTerminal(Token::STATEMENT, children) = &children[0] {
            code.extend(generate_statement(symbol_table, children));
        }

        if children.len() >= 3 {
            if let Node::NonTerminal(Token::STATEMENT_LIST, children) = &children[2] {
                code.extend(generate_statement_list(symbol_table, children));
            }
        }
    }

    code
}

fn generate_statement(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    match &children[0] {
        Node::NonTerminal(Token::ASSIGNMENT, children) => {
            code.extend(generate_assignment(symbol_table, children));
        }
        Node::NonTerminal(Token::VAR_DECL, _) => (),
        Node::NonTerminal(Token::RETURN_STMT, children) => {
            code.extend(generate_return_stmt(symbol_table, children));
        }
        Node::NonTerminal(Token::BREAK_STMT, children) => {
            code.extend(generate_break_stmt(symbol_table, children));
        }
        Node::NonTerminal(Token::CONTINUE_STMT, children) => {
            code.extend(generate_continue_stmt(symbol_table, children));
        }
        Node::NonTerminal(Token::IF_STMT, children) => {
            code.extend(generate_if_stmt(symbol_table, children));
        }
        Node::NonTerminal(Token::WHILE_STMT, children) => {
            code.extend(generate_while_stmt(symbol_table, children));
        }
        Node::NonTerminal(Token::EXPRESSION, children) => {
            code.extend(generate_expression(symbol_table, children));
        }
        node => panic!("unvalid parse tree: {:?}", node),
    }

    code
}

fn generate_assignment(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_return_stmt(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_break_stmt(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_continue_stmt(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_if_stmt(symbol_table: &SymbolTable, children: &Vec<Node>) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_while_stmt(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}

fn generate_expression(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    for child in children {
        traverse_debug(child);
    }

    code
}
