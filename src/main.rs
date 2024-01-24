use lox_compiler::*;
use std::fs;
fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("should pass in a path to a file");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tokens = lexer::scan(contents);
    for token in tokens {
        println!("{:?}", token);
    }
}
