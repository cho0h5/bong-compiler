use crate::parser::formatting::Tree;
use crate::parser::Node;
use crate::{lexer::*, symbol_table};

#[derive(Debug)]
enum Address {
    Label(String),
    Offset(i32),
}

#[derive(Debug)]
enum SymbolType {
    Func(Node),
    Var(Node),
}

pub struct SymbolTable(Vec<SymbolTableElement>);

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for e in &self.0 {
            e.fmt(f);
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
    address: Option<Address>,
}

impl std::fmt::Display for SymbolTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:20}\t{:10}\t{:20?}\t{:5}\t{:<20?}\n",
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
        address: Option<Address>,
    ) -> Self {
        SymbolTableElement {
            scope: scope.to_string(),
            identifier: identifier.to_string(),
            symbol_type: symbol_type,
            size: size,
            address: address,
        }
    }
}

pub fn generate_symbol_table(syntax_tree: &Tree) -> SymbolTable {
    let mut symbol_table = Vec::new();
    traverse_tree(&mut symbol_table, &syntax_tree.0, "");

    SymbolTable(symbol_table)
}

fn traverse_tree(symbol_table: &mut Vec<SymbolTableElement>, node: &Node, scope: &str) {
    match node {
        Node::Terminal(token) => println!("{}\t\t{:?}", scope, token),
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
                    let var_type = match &children[0] {
                        Node::NonTerminal(_, var_type) => &var_type[0],
                        _ => panic!("var_type error"),
                    };
                    let id = match &children[1] {
                        Node::Terminal(Token::Identifier(id)) => id,
                        _ => panic!("can't find type"),
                    };
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Func(var_type.clone()),
                        0,
                        None,
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
                        Node::Terminal(Token::Identifier(id)) => id,
                        _ => panic!("can't find type"),
                    };
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Var(var_type.clone()),
                        4,
                        None,
                    );
                    symbol_table.push(element);
                }
                Token::VAR_DECL => {
                    let var_type = match &children[0] {
                        Node::NonTerminal(_, var_type) => &var_type[0],
                        _ => panic!("var_type error"),
                    };
                    let id = match &children[1] {
                        Node::Terminal(Token::Identifier(id)) => id,
                        _ => panic!("can't find type"),
                    };
                    let element = SymbolTableElement::new(
                        &scope,
                        id,
                        SymbolType::Var(var_type.clone()),
                        4,
                        None,
                    );
                    symbol_table.push(element);
                }
                _ => (),
            }
            // println!("{}\t\t{:?}", scope, token);
            for child in children {
                traverse_tree(symbol_table, child, &scope);
            }
        }
    }
}
