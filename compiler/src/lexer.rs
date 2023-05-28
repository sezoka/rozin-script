use super::token::Token;

#[derive(Clone)]
pub struct Lexer<'a> {
    char_iter: std::str::Chars<'a>,
    token_start: &'a str,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            char_iter: source.chars(),
            token_start: source,
            line: 1,
        }
    }

    fn peek_next(&mut self) -> char {
        let mut iter_clone = self.char_iter.clone();
        iter_clone.next();
        iter_clone.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        self.char_iter.clone().next().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        self.char_iter.next().unwrap_or('\0')
    }

    pub fn get_lexeme(&self) -> &'a str {
        let len = self.token_start.len() - self.char_iter.as_str().len();
        &self.token_start[0..len]
    }

    fn matches(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), ' ' | '\t' | '\n' | '\r') {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let lexeme = self.get_lexeme();

        use Token::*;
        match lexeme {
            "and" => And,
            "or" => Or,
            "xor" => Xor,
            "mod" => Mod,
            "fn" => Fn,
            _ => Identifier,
        }
    }

    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
            Token::Float
        } else {
            Token::Int
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.token_start = self.char_iter.as_str();

        let c = self.advance();

        if c == '\0' {
            return None;
        }

        if is_digit(c) {
            return Some(self.number());
        }

        if is_alpha(c) {
            return Some(self.identifier());
        }

        use Token::*;
        Some(match c {
            '+' => Plus,
            '-' => Minus,
            '*' => {
                if self.matches('*') {
                    Pow
                } else {
                    Star
                }
            }
            '/' => Slash,
            '%' => Reminder,
            '=' => {
                if self.matches('=') {
                    Equal
                } else {
                    Assign
                }
            }
            '~' => BitNot,
            '&' => BitAnd,
            '|' => BitOr,
            '^' => BitXor,
            '<' => {
                if self.matches('<') {
                    Shl
                } else if self.matches('=') {
                    LessEqual
                } else {
                    Less
                }
            }
            '>' => {
                if self.matches('>') {
                    Shr
                } else if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                }
            }
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '!' => {
                if self.matches('=') {
                    NotEqual
                } else {
                    Not
                }
            }
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            _ => Illegal,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        use Token::*;

        let input = " ! + - * / % = ~ & | ^ < > ( ) { } ;
<= >= == != **

name = fn(a, b) {
    a = a mod b;
}

name = 123.12345;
name = 1234 * -2;

module.fun;
";

        let expected = [
            Not,
            Plus,
            Minus,
            Star,
            Slash,
            Reminder,
            Assign,
            BitNot,
            BitAnd,
            BitOr,
            BitXor,
            Less,
            Greater,
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            Semicolon,
            LessEqual,
            GreaterEqual,
            Equal,
            NotEqual,
            Pow,
            Identifier,
            Assign,
            Fn,
            LeftParen,
            Identifier,
            Comma,
            Identifier,
            RightParen,
            LeftBrace,
            Identifier,
            Assign,
            Identifier,
            Mod,
            Identifier,
            Semicolon,
            RightBrace,
            Identifier,
            Assign,
            Float,
            Semicolon,
            Identifier,
            Assign,
            Int,
            Star,
            Minus,
            Int,
            Semicolon,
            Identifier,
            Dot,
            Identifier,
            Semicolon,
            Identifier,
            Dot,
            Identifier,
            Semicolon,
        ];

        let mut lexer = Lexer::new(input);

        let mut i = 0;
        while let Some(t) = lexer.next() {
            let e = expected[i];
            println!("'{}'", lexer.get_lexeme());
            assert_eq!(t, e, "Wrong token at idx '{i}', '{}'", lexer.get_lexeme());
            i += 1;
        }
    }
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
