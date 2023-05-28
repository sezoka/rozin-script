use crate::parser::Parser;

mod lexer;
mod parser;
mod token;
mod types;

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    while let Ok(len) = stdin.read_line(&mut buf) {
        if len == 0 {
            break;
        }
        let mut parser = Parser::new(&buf);
        let expression = parser.parse();
        println!("{:?}", expression);
        buf.clear();
    }
}
