use super::token::Token;

#[derive(Clone)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
    token_start: &'a str,
    had_error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            token_start: source,
            had_error: false,
        }
    }

    pub fn get_lexeme(&self) -> &'a str {
        let curr_pos = self.token_start.len() - self.chars.as_str().len();
        &self.token_start[0..curr_pos]
    }

    fn get_src_position(&mut self) -> (char, usize, usize) {
        let err_pos = self.source.len() - self.chars.as_str().len() - 1;
        let ch = self.source.as_bytes()[err_pos] as char;
        let mut line = 1;
        let mut row = 1;
        for (i, c) in self.source.chars().enumerate() {
            if err_pos <= i {
                break;
            } else if c == '\n' {
                line += 1;
                row = 1;
            } else {
                row += 1;
            }
        }

        (ch, line, row)
    }

    fn error_unexpected_char(&mut self) {
        self.had_error = true;
        let (ch, line, row) = self.get_src_position();
        eprintln!("Error: unexpected symbol '{ch}' at line: {line}, row: {row}.",);
    }

    fn error_expected(&mut self, c: char) {
        self.had_error = true;
        let (ch, line, row) = self.get_src_position();
        eprintln!("Error: expected '{c}' got '{ch}' at line: {line}, row: {row}.",);
    }

    fn error_msg(&mut self, msg: &str) {
        self.had_error = true;
        let (_, line, row) = self.get_src_position();
        eprintln!("Error: {msg} at line: {line}, row: {row}.",);
    }

    fn matches(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, c: char, token: Token) -> Option<Token> {
        if self.peek() == c {
            self.advance();
            Some(token)
        } else {
            self.error_expected(c);
            None
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.peek().is_whitespace() {
                self.advance();
            } else if self.peek() == '/' && self.peek_next() == '/' {
                while self.peek() != '\n' && self.peek() != '\0' {
                    self.advance();
                }
                self.advance();
            } else {
                break;
            };
        }
    }

    fn advance(&mut self) -> char {
        self.chars.next().unwrap_or_default()
    }

    fn peek_next(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        return chars.next().unwrap_or_default();
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    fn start_new_token(&mut self) {
        self.token_start = self.chars.as_str();
    }

    fn identifier(&mut self) -> Token {
        while is_identifier(self.peek()) {
            self.advance();
        }

        match self.get_lexeme() {
            "fn" => Token::Fn,
            "while" => Token::While,
            "for" => Token::For,
            _ => Token::Identifier,
        }
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();
            if !self.peek().is_ascii_digit() {
                self.error_msg("expect digits after '.'");
                return None;
            }
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            return Some(Token::Float);
        }

        return Some(Token::Int);
    }

    fn char(&mut self) -> Option<Token> {
        self.advance();
        self.consume('\'', Token::Char)
    }

    fn builtin(&mut self) -> Option<Token> {
        if self.peek().is_ascii_alphabetic() {
            while is_identifier(self.peek()) {
                self.advance();
            }
            Some(Token::Builtin)
        } else {
            None
        }
    }

    fn atom(&mut self) -> Option<Token> {
        if is_identifier(self.peek()) {
            while is_identifier(self.peek()) {
                self.advance();
            }
            Some(Token::Atom)
        } else {
            None
        }
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek() != '"' && self.peek() != '\0' {
            self.advance();
        }

        if self.peek() == '\0' {
            self.error_msg("unterminated string");
            None
        } else {
            self.advance();
            Some(Token::String)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.start_new_token();

        let c = self.advance();
        if c == '\0' {
            return None;
        }

        Some(match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '\\' => Token::BackSlash,
            '~' => Token::Tilda,
            '&' => Token::Ampersand,
            '|' => Token::Bar,
            '^' => Token::Caret,
            '!' if self.matches('=') => Token::NotEqual,
            '=' if self.matches('=') => Token::EqualEqual,
            '=' if self.matches('>') => Token::EqualGreater,
            '=' => Token::Equal,
            '>' if self.matches('=') => Token::GreaterEqual,
            '>' if self.matches('=') => Token::GreaterGreater,
            '>' => Token::Greater,
            '<' if self.matches('=') => Token::LessEqual,
            '<' if self.matches('<') => Token::LessLess,
            '<' => Token::Less,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '#' => Token::Hash,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            ':' => self.atom()?,
            '.' => Token::Dot,
            '@' => self.builtin()?,
            '\'' => self.char()?,
            '\"' => self.string()?,
            _ => {
                if c.is_ascii_digit() {
                    self.number()?
                } else if is_identifier(c) {
                    self.identifier()
                } else {
                    self.error_unexpected_char();
                    return None;
                }
            }
        })
    }
}

fn is_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
