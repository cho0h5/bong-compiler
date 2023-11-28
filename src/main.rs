mod code_generator;
mod lexer;
mod parser;
mod symbol_table;

use std::env;
use std::fs;
use std::process;

use code_generator::generate_code;
use lexer::UnknownTokenError;
use parser::ParsingError;
use symbol_table::generate_symbol_table;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => {
            print!("\x1b[32m[1/6]\x1b[37m ");
            println!("File name: {}\n", filename);
            filename
        }
        None => {
            println!("\x1b[31m[1/6] error\x1b[37m: no input file");
            process::exit(1);
        }
    };

    let raw_contents = match fs::read_to_string(filename) {
        Ok(contents) => {
            print!("\x1b[32m[2/6]\x1b[37m ");
            println!("File contents: \n{}\n", contents.trim());
            contents
        }
        Err(_) => {
            println!("\x1b[31m[2/4] error\x1b[37m: something went wrong during reading file");
            process::exit(1);
        }
    };

    let tokens = match lexer::read_lexeme(&raw_contents) {
        Ok(tokens) => {
            print!("\x1b[32m[3/6]\x1b[37m ");
            println!("Read tokens:\n{}\n", tokens);
            tokens
        }
        Err(UnknownTokenError(token)) => {
            println!("\x1b[31m[3/4] error\x1b[37m: unknown token: {}", token);
            process::exit(1);
        }
    };

    let tree = match parser::parse(tokens) {
        Ok(tree) => {
            print!("\x1b[32m[4/6]\x1b[37m ");
            println!("Parse tree:\n{}\n", tree);
            tree
        }
        Err(ParsingError(found)) => {
            println!("\x1b[31m[4/4] error\x1b[37m: parsing error");
            println!("\tfound: {:?}", found);
            process::exit(1);
        }
    };

    let symbol_table = generate_symbol_table(&tree);
    print!("\x1b[32m[5/6]\x1b[37m ");
    println!("Symbol table:\n{}\n", symbol_table);

    // let assemply_code = generate_code(&tree, &symbol_table);
    print!("\x1b[32m[6/6]\x1b[37m ");
    // println!("Generated code:\n{}", assemply_code);
}
