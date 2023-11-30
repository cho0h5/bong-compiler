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
    children: &Vec<Node>,
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
        code.extend(generate_parameters(symbol_table, children));
    }

    for child in children {
        traverse_debug(child);
    }

    code.push(Box::new(RFormat::new(
        Funct::Jr,
        RegisterName::RA,
        RegisterName::Zero,
        RegisterName::Zero,
        0,
    )));

    code
}

fn generate_parameters(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();

    if let Node::NonTerminal(Token::PARAMETER_LIST, children) = &children[1] {
        code.extend(generate_parameter_list(symbol_table, children, 0));
    }

    code
}
fn generate_parameter_list(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
    count: i16,
) -> Vec<Box<dyn Instruction>> {
    let mut code: Vec<Box<dyn Instruction>> = Vec::new();
    if !children.is_empty() {
        if let Node::NonTerminal(Token::PARAMETER_DECL, children) = &children[0] {
            code.extend(generate_parameter_decl(symbol_table, children, count));
        }
        if children.len() >= 3 {
            if let Node::NonTerminal(Token::PARAMETER_LIST, children) = &children[2] {
                code.extend(generate_parameter_list(symbol_table, children, count + 1));
            }
        }
    }
    code
}

fn generate_parameter_decl(
    symbol_table: &SymbolTable,
    children: &Vec<Node>,
    count: i16,
) -> Vec<Box<dyn Instruction>> {
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
