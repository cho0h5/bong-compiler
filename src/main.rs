mod token_reader;
mod parser;

use std::process;
use std::env;
use std::fs;

use token_reader::UnknownTokenError;
use parser::ParsingError;

// 이 main함수는 총 4개의 과정을 거쳐 parsing tree를 생성합니다.
// 1. 파일 이름 가져오기
// 2. 파일 읽기
// 3. token 인식하기
// 4. parsing

fn main() {
    // 1. 입력받은 파일 이름이 있다면 filename에 저장합니다.
    let filename = match env::args().nth(1) {
        Some(filename) => {
            print!("\x1b[32m[1/4]\x1b[37m ");
            println!("File name: {}\n", filename);
            filename
        },
        None => {
            // 입력된 argument가 없다면 에러 내용을 출력하고 종료합니다.
            println!("\x1b[31m[1/4] error\x1b[37m: no input file");
            process::exit(1);
        },
    };

    // 2. 파일 읽기에 성공하면 raw_contents에 저장합니다.
    let raw_contents = match fs::read_to_string(filename) {
        Ok(contents) => {
            print!("\x1b[32m[2/4]\x1b[37m ");
            println!("File contents: \n{}\n", contents.trim());
            contents
        },
        Err(_) => {
            // 파일 읽는 중에 에러가 발생했다면 에러를 출력하고 종료합니다.
            println!("\x1b[31m[2/4] error\x1b[37m: something went wrong during reading file");
            process::exit(1);
        },
    };

    // 3. token을 인식하는데 성공하면 tokens의 배열을 tokens에 저장합니다.
    //    token_reader::reader_token(...) 함수는 src/token_reader.rs 에 작성되어 있습니다.
    let tokens = match token_reader::read_tokens(&raw_contents) {
        Ok(tokens) => {
            print!("\x1b[32m[3/4]\x1b[37m ");
            println!("Read tokens:\n{}\n", tokens);
            tokens
        },
        Err(UnknownTokenError(token)) => {
            // token을 인식하는 중 에러가 발생했다면 인식할 수 없는 키워드를 출력한 후 종료합니다.
            println!("\x1b[31m[3/4] error\x1b[37m: unknown token: {}", token);
            process::exit(1);
        },
    };

    // 4. 입력된 tokens를 parsing하고 parse tree를 생성한 후 출력합니다.
    //    parser::parse(...) 함수는 src/parser/mod.rs 에 작성되어 있습니다.
    match parser::parse(tokens) {
        Ok(tree) => {
            print!("\x1b[32m[4/4]\x1b[37m ");
            println!("Parse tree:\n{}", tree);
            println!("Accepted!");
            tree
        },
        Err(ParsingError(expected, found)) => {
            // parsing에 실패했다면 어느 token을 parsing하다가 실패하였는지, 어떤 token이
            // expected되는지 출력합니다.
            println!("\x1b[31m[4/4] error\x1b[37m: parsing error");
            println!("\texpected: {:?}", expected);
            println!("\tbut found: {:?}", found);
            process::exit(1);
        },
    };
}
