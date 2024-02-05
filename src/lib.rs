mod code_generator;
mod instruction;
mod lexer;
mod parser;
mod symbol_table;

use std::process;

use code_generator::generate_code;
use lexer::UnknownTokenError;
use parser::ParsingError;
use symbol_table::generate_symbol_table;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(raw_contents: &str) -> String {
    let tokens = match lexer::read_lexeme(&raw_contents) {
        Ok(tokens) => {
            print!("\x1b[32m[3/7]\x1b[37m ");
            println!("Read tokens:\n{}\n", tokens);
            tokens
        }
        Err(UnknownTokenError(token)) => {
            println!("\x1b[31m[3/7] error\x1b[37m: unknown token: {}", token);
            process::exit(1);
        }
    };

    let mut tree = match parser::parse(tokens) {
        Ok(tree) => {
            print!("\x1b[32m[4/7]\x1b[37m ");
            println!("Parse tree:\n{}\n", tree);
            tree
        }
        Err(ParsingError(found)) => {
            println!("\x1b[31m[4/7] error\x1b[37m: parsing error");
            println!("\tfound: {:?}", found);
            process::exit(1);
        }
    };

    let symbol_table = generate_symbol_table(&mut tree);
    print!("\x1b[32m[5/7]\x1b[37m ");
    println!("Symbol table:\n{}\n", symbol_table);
    print!("\x1b[32m[5/7]\x1b[37m ");
    println!("Parse tree:\n{}", tree);

    let assemply_code = generate_code(&tree, &symbol_table);
    print!("\x1b[32m[6/7]\x1b[37m ");
    println!("Generated code:");
    for line in &assemply_code {
        println!("{}", line.to_string());
    }
    println!();

    print!("\x1b[32m[7/7]\x1b[37m ");
    let mut assembly_code = String::new();
    println!("Writing code...");
    for line in &assemply_code {
        assembly_code.push_str(&line.to_string());
        assembly_code.push_str("\n");
    }
    assembly_code
}
