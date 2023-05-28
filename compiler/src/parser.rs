use std::{fmt::Debug, fs::write};

use crate::{lexer::Lexer, token::Token, types::Int};

type Precedence = u8;
const PREC_NONE: u8 = 0;
const PREC_ASSIGNMENT: u8 = 1;
const PREC_OR: u8 = 2;
const PREC_AND: u8 = 3;
const PREC_EQUALITY: u8 = 4;
const PREC_COMPARISON: u8 = 5;
const PREC_TERM: u8 = 6;
const PREC_FACTOR: u8 = 7;
const PREC_UNARY: u8 = 8;
const PREC_CALL: u8 = 9;
const PREC_PRIMARY: u8 = 10;

#[derive(PartialEq, Eq)]
pub enum Expression<'a> {
    Assign { name: &'a str, value: Node<'a> },
    Add { a: Node<'a>, b: Node<'a> },
    Sub { a: Node<'a>, b: Node<'a> },
    Mul { a: Node<'a>, b: Node<'a> },
    Div { a: Node<'a>, b: Node<'a> },
    Rem { a: Node<'a>, b: Node<'a> },
    Mod { a: Node<'a>, b: Node<'a> },
    Power { a: Node<'a>, b: Node<'a> },
    Equal { a: Node<'a>, b: Node<'a> },
    NotEqual { a: Node<'a>, b: Node<'a> },
    Less { a: Node<'a>, b: Node<'a> },
    Greater { a: Node<'a>, b: Node<'a> },
    LessEqual { a: Node<'a>, b: Node<'a> },
    GreaterEqual { a: Node<'a>, b: Node<'a> },
    And { a: Node<'a>, b: Node<'a> },
    Or { a: Node<'a>, b: Node<'a> },
    Xor { a: Node<'a>, b: Node<'a> },
    Shl { a: Node<'a>, b: Node<'a> },
    Shr { a: Node<'a>, b: Node<'a> },
    BitAnd { a: Node<'a>, b: Node<'a> },
    BitOr { a: Node<'a>, b: Node<'a> },
    BitXor { a: Node<'a>, b: Node<'a> },

    Identifier(&'a str),
    Int(Int),
    None,
}

type Node<'a> = Box<Expression<'a>>;

impl<'a> Debug for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Assign { name, value } => write!(f, "(= {:?} {:?})", name, value),
            Add { a, b } => write!(f, "(+ {:?} {:?})", a, b),
            Identifier(name) => write!(f, "{}", name),
            Int(int) => write!(f, "{}", int),
            Sub { a, b } => write!(f, "(- {:?} {:?})", a, b),
            Mul { a, b } => write!(f, "(* {:?} {:?})", a, b),
            Div { a, b } => write!(f, "(/ {:?} {:?})", a, b),
            Rem { a, b } => write!(f, "(% {:?} {:?})", a, b),
            Mod { a, b } => write!(f, "(mod {:?} {:?})", a, b),
            Power { a, b } => write!(f, "(** {:?} {:?})", a, b),
            Equal { a, b } => write!(f, "(== {:?} {:?})", a, b),
            NotEqual { a, b } => write!(f, "(!= {:?} {:?})", a, b),
            Less { a, b } => write!(f, "(< {:?} {:?})", a, b),
            Greater { a, b } => write!(f, "(> {:?} {:?})", a, b),
            LessEqual { a, b } => write!(f, "(<= {:?} {:?})", a, b),
            GreaterEqual { a, b } => write!(f, "(>= {:?} {:?})", a, b),
            And { a, b } => write!(f, "(and {:?} {:?})", a, b),
            Or { a, b } => write!(f, "(or {:?} {:?})", a, b),
            Xor { a, b } => write!(f, "(xor {:?} {:?})", a, b),
            Shl { a, b } => write!(f, "(<< {:?} {:?})", a, b),
            Shr { a, b } => write!(f, "(>> {:?} {:?})", a, b),
            BitAnd { a, b } => write!(f, "(& {:?} {:?})", a, b),
            BitOr { a, b } => write!(f, "(| {:?} {:?})", a, b),
            BitXor { a, b } => write!(f, "(^ {:?} {:?})", a, b),

            None => write!(f, "XXX"),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    prev_token: Token,
    at_end: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source),
            curr_token: Token::Illegal,
            prev_token: Token::Illegal,
            at_end: false,
        };

        parser.advance();

        parser
    }

    fn advance(&mut self) {
        self.prev_token = self.curr_token;
        if let Some(token) = self.lexer.next() {
            self.curr_token = token;
        } else {
            self.at_end = true;
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Option<Expression<'a>> {
        // println!("parse_precedence");

        use Token::*;
        let mut expression = match self.curr_token {
            Identifier => self.identifier(),
            Int => self.int(),
            _ => panic!("{:?}", self.curr_token),
        };

        // println!("a: {:?}", expression);

        self.advance();

        while precedence <= get_precedence(self.curr_token) && !self.at_end {
            let op = self.curr_token;
            self.advance();
            expression = self.run_infix(expression, op)?;
        }

        Some(expression)
    }

    fn run_infix(&mut self, a: Expression<'a>, op: Token) -> Option<Expression<'a>> {
        // println!("run_infix {:?}", op);
        let precedence = get_precedence(op);
        let b = self.parse_precedence(precedence + 1)?;
        // println!("a: {:?}, b: {:?}", a, b);

        let a_box = Box::new(a);
        let b_box = Box::new(b);

        use Expression::*;
        Some(match op {
            Token::Plus => Add { a: a_box, b: b_box },
            Token::Minus => Sub { a: a_box, b: b_box },
            // Token::Assign => Assign {
            //     name: a_box,
            //     value: b_box,
            // },
            Token::Plus => Add { a: a_box, b: b_box },
            Token::Star => Mul { a: a_box, b: b_box },
            Token::Pow => Power { a: a_box, b: b_box },
            Token::Slash => Div { a: a_box, b: b_box },
            Token::Reminder => Rem { a: a_box, b: b_box },
            Token::Equal => Equal { a: a_box, b: b_box },
            Token::NotEqual => NotEqual { a: a_box, b: b_box },
            Token::And => And { a: a_box, b: b_box },
            Token::Or => Or { a: a_box, b: b_box },
            Token::Xor => Xor { a: a_box, b: b_box },
            Token::BitAnd => BitAnd { a: a_box, b: b_box },
            Token::BitOr => BitOr { a: a_box, b: b_box },
            Token::BitXor => BitXor { a: a_box, b: b_box },
            Token::Shl => Shl { a: a_box, b: b_box },
            Token::Shr => Shr { a: a_box, b: b_box },
            Token::Less => Less { a: a_box, b: b_box },
            Token::LessEqual => LessEqual { a: a_box, b: b_box },
            Token::Greater => Greater { a: a_box, b: b_box },
            Token::GreaterEqual => GreaterEqual { a: a_box, b: b_box },
            _ => unimplemented!(),
        })
    }

    fn int(&mut self) -> Expression<'a> {
        let int = self.lexer.get_lexeme().parse::<Int>().unwrap();
        Expression::Int(int)
    }

    fn identifier(&mut self) -> Expression<'a> {
        Expression::Identifier(self.lexer.get_lexeme())
    }

    pub fn parse(&mut self) -> Expression {
        self.parse_precedence(0).unwrap_or(Expression::None)
    }
}

fn get_precedence(token: Token) -> Precedence {
    use Token::*;
    match token {
        Assign => PREC_ASSIGNMENT,
        Plus => PREC_TERM,
        Minus => PREC_TERM,
        Identifier => PREC_PRIMARY,
        Int => PREC_PRIMARY,
        Mul => PREC_FACTOR,
        Div => PREC_FACTOR,
        Rem => PREC_FACTOR,
        Mod => PREC_FACTOR,
        Power => PREC_FACTOR,
        Equal => PREC_EQUALITY,
        NotEqual => PREC_EQUALITY,
        Less => PREC_COMPARISON,
        Greater => PREC_COMPARISON,
        LessEqual => PREC_COMPARISON,
        GreaterEqual => PREC_COMPARISON,
        And => PREC_AND,
        Or => PREC_OR,
        Xor => PREC_OR,
        Shl => PREC_FACTOR,
        Shr => PREC_FACTOR,
        BitAnd => PREC_FACTOR,
        BitOr => PREC_FACTOR,
        BitXor => PREC_FACTOR,
    }
}
