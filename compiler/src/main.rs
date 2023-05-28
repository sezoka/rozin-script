// use crate::parser::Parser;
use crate::{lexer::Lexer, token::Token};

mod lexer;
mod parser;
mod token;
mod types;

fn main() {
    let source = include_str!("../examples/monkey/lexer.mil");

    let mut lexer = Lexer::new(source);

    while let Some(token) = lexer.next() {
        match token {
            Token::Identifier
            | Token::Char
            | Token::String
            | Token::Int
            | Token::Float
            | Token::Atom => {
                println!("{:?} -> {}", token, lexer.get_lexeme())
            }
            _ => println!("{:?}", token),
        }
    } 
}
