use core::panic;

use crate::lexer::*;
use crate::parser::formatting::Tree;
use crate::parser::Node;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Address {
    Label(String),
    Offset(u32),
}

#[derive(Debug)]
enum SymbolType {
    Func(Node),
    Var(Node),
}

pub struct SymbolTable(Vec<SymbolTableElement>);

impl SymbolTable {
    pub fn get_function_size(&self, name: &str) -> u32 {
        for element in &self.0 {
            if element.scope.is_empty() && element.identifier == name {
                return element.size;
            }
        }
        0
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for e in &self.0 {
            e.fmt(f).ok();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SymbolTableElement {
    scope: String,
    identifier: String,
    symbol_type: SymbolType,
    size: u32,
    address: Address,
}

impl std::fmt::Display for SymbolTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{:20}\t{:10}\t{:20?}\t{:5}\t{:?}",
            self.scope, self.identifier, self.symbol_type, self.size, self.address
        )
    }
}

impl SymbolTableElement {
    fn new(
        scope: &str,
        identifier: &str,
        symbol_type: SymbolType,
        size: u32,
        address: Address,
    ) -> Self {
        SymbolTableElement {
            scope: scope.to_string(),
            identifier: identifier.to_string(),
            symbol_type,
            size,
            address,
        }
    }
}

pub fn generate_symbol_table(syntax_tree: &mut Tree) -> SymbolTable {
    let mut symbol_table = Vec::new();
    traverse_tree(&mut symbol_table, &mut syntax_tree.0, "");

    SymbolTable(symbol_table)
}

fn traverse_tree(symbol_table: &mut Vec<SymbolTableElement>, node: &mut Node, scope: &str) {
    match node {
        Node::NonTerminal(token, children) => {
            let mut scope = scope.to_string();
            match token {
                Token::IF_STMT => {
                    if let Node::Terminal(Token::If(id)) = &children[0] {
                        scope.push('/');
                        scope.push_str(id);
                    }
                }
                Token::WHILE_STMT => {
                    if let Node::Terminal(Token::While(id)) = &children[0] {
                        scope.push('/');
                        scope.push_str(id);
                    }
                }
                Token::FUNCTION_DECL => {
                    if let Node::Terminal(Token::Identifier(id, address)) = &mut children[1] {
                        *address = Some(Address::Label(id.clone()));
                    }
                    let var_type = match &children[0] {
                        Node::NonTerminal(_, var_type) => &var_type[0],
                        _ => panic!("var_type error"),
                    };
                    let id = match &children[1] {
                        Node::Terminal(Token::Identifier(id, _)) => id,
                        _ => panic!("can't find type"),
                    };
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Func(var_type.clone()),
                        0,
                        Address::Label(id.clone()),
                    );
                    scope.push('/');
                    scope.push_str(id);
                    symbol_table.push(element);
                }
                Token::PARAMETER_DECL if !children.is_empty() => {
                    let var_type = match &children[0] {
                        Node::NonTerminal(_, var_type) => &var_type[0],
                        _ => panic!("var_type error"),
                    };
                    let id = match &children[1] {
                        Node::Terminal(Token::Identifier(id, _)) => id,
                        _ => panic!("can't find type"),
                    };
                    let func_name = find_function_name(&scope);
                    let size = calculate_type_size(var_type);
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Var(var_type.clone()),
                        size,
                        Address::Offset(search_function_size(symbol_table, &func_name)),
                    );
                    increase_function_size(symbol_table, &func_name, size);
                    symbol_table.push(element);
                }
                Token::PRIMARY_EXPR if children.len() >= 2 => {
                    if let Node::NonTerminal(Token::ARGUMENTS, _) = children[1] {
                        if let Node::NonTerminal(Token::PRIMARY_EXPR, operand) = &mut children[0] {
                            if let Node::NonTerminal(Token::OPERAND, operand) = &mut operand[0] {
                                if let Node::Terminal(Token::Identifier(id, address)) =
                                    &mut operand[0]
                                {
                                    *address = Some(Address::Label(id.clone()));
                                }
                            }
                        }
                    }
                }
                Token::VAR_DECL => {
                    let var_type = match &children[0] {
                        Node::NonTerminal(_, var_type) => &var_type[0],
                        _ => panic!("var_type error"),
                    };
                    let id = match &children[1] {
                        Node::Terminal(Token::Identifier(id, _)) => id,
                        _ => panic!("can't find type"),
                    };
                    let func_name = find_function_name(&scope);
                    let size = calculate_type_size(var_type);
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Var(var_type.clone()),
                        size,
                        Address::Offset(search_function_size(symbol_table, &func_name)),
                    );
                    increase_function_size(symbol_table, &func_name, size);
                    symbol_table.push(element);
                }
                _ => (),
            }
            for child in children {
                traverse_tree(symbol_table, child, &scope);
            }
        }
        Node::Terminal(Token::Identifier(id, address)) => {
            let scopes = scopes(scope);
            println!("{:?}:\t{}:\t{:?}", scopes, id, address);
            for e in symbol_table.iter().rev() {
                if scopes.contains(&e.scope) && id == &e.identifier {
                    *address = Some(e.address.clone());
                }
            }
        }
        _ => (),
    }
}

fn calculate_type_size(var_type: &Node) -> u32 {
    match var_type {
        Node::Terminal(_) => 4,
        Node::NonTerminal(Token::POINTER_TYPE, _) => 4,
        Node::NonTerminal(Token::ARRAY_TYPE, children) => match &children[2] {
            Node::Terminal(Token::IntLit(size)) => *size * 4,
            _ => panic!("can't find size"),
        },
        x => panic!("failed to calculate type size: {:?}", x),
    }
}

fn find_function_name(scope: &str) -> String {
    scope.split('/').nth(1).unwrap().to_string()
}

fn search_function_size(symbol_table: &Vec<SymbolTableElement>, func_name: &str) -> u32 {
    for element in symbol_table {
        if element.scope.is_empty() && element.identifier == func_name {
            return element.size;
        }
    }
    panic!("function ({}) not found", func_name)
}

fn increase_function_size(symbol_table: &mut Vec<SymbolTableElement>, func_name: &str, gap: u32) {
    for element in symbol_table {
        if element.scope.is_empty() && element.identifier == func_name {
            element.size += gap;
            break;
        }
    }
}
fn scopes(scope: &str) -> Vec<String> {
    let mut tmp = String::new();
    let mut iter = scope.split('/');
    let mut scopes = Vec::new();
    iter.next();
    for s in iter {
        tmp.push('/');
        tmp.push_str(s);
        scopes.push(tmp.clone());
    }
    scopes
}
