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
            generated_code.push(Box::new(RFormat::new(
                Funct::Add,
                RegisterName::K0,
                RegisterName::K0,
                RegisterName::K0,
                0,
            )));
            for child in children {
                traverse_tree(generated_code, symbol_table, child);
            }
        }
        Node::Terminal(token) => {
            // println!("{:?}", token),
        }
        Node::NonTerminal(token, children) => {
            // println!("{:?}", token);
            for child in children {
                traverse_tree(generated_code, symbol_table, child);
            }
        }
    }
}
