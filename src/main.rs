mod code_generator;
mod instruction;
mod lexer;
mod parser;
mod symbol_table;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

use code_generator::generate_code;
use lexer::UnknownTokenError;
use parser::ParsingError;
use symbol_table::generate_symbol_table;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => {
            if check_file_extension(&filename) {
                print!("\x1b[32m[1/7]\x1b[37m ");
                println!("File name: {}\n", filename);
                filename
            } else {
                println!("\x1b[31m[1/7] error\x1b[37m: file extension is not .bong");
                process::exit(1);
            }
        }
        None => {
            println!("\x1b[31m[1/7] error\x1b[37m: no input file");
            process::exit(1);
        }
    };

    let raw_contents = match fs::read_to_string(&filename) {
        Ok(contents) => {
            print!("\x1b[32m[2/7]\x1b[37m ");
            println!("File contents: \n{}\n", contents.trim());
            contents
        }
        Err(_) => {
            println!("\x1b[31m[2/7] error\x1b[37m: something went wrong during reading file");
            process::exit(1);
        }
    };

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

    let mut target_filename = Path::new(&filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    target_filename.push_str(".s");
    if let Ok(mut file) = File::create(&target_filename) {
        print!("\x1b[32m[7/7]\x1b[37m ");
        println!("Writing code to {}...", target_filename);
        for line in &assemply_code {
            file.write(line.to_string().as_bytes()).ok();
            file.write(b"\n").ok();
        }
    } else {
        println!("\x1b[31m[7/7] error\x1b[37m: file open error");
        process::exit(1);
    }
}

fn check_file_extension(filename: &str) -> bool {
    let splited_str = filename.split('.').last();
    if let Some(extension) = splited_str {
        return extension == "bong";
    }
    false
}
