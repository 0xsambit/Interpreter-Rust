#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Function,
    Ident(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    EOF,
}

pub struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input.chars().nth(self.pos).unwrap();
        match ch {
            '+' => {
                self.pos += 1;
                Token::Plus
            }
            '-' => {
                self.pos += 1;
                Token::Minus
            }
            '*' => {
                self.pos += 1;
                Token::Star
            }
            '/' => {
                self.pos += 1;
                Token::Slash
            }
            '(' => {
                self.pos += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                Token::RParen
            }
            ',' => {
                self.pos += 1;
                Token::Comma
            }
            '0'..='9' | '.' => self.read_number(),
            'a'..='z' | 'A'..='Z' => self.read_ident(),
            _ => panic!("Unknown character: {}", ch),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap().is_numeric()
        {
            self.pos += 1;
        }
        if self.pos < self.input.len() && self.input.chars().nth(self.pos).unwrap() == '.' {
            self.pos += 1;
            while self.pos < self.input.len()
                && self.input.chars().nth(self.pos).unwrap().is_numeric()
            {
                self.pos += 1;
            }
        }
        Token::Number(self.input[start..self.pos].parse::<f64>().unwrap())
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && self.input.chars().nth(self.pos).unwrap().is_alphanumeric()
        {
            self.pos += 1;
        }
        let ident = &self.input[start..self.pos];
        match ident {
            "function" => Token::Function,
            _ => Token::Ident(ident.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len()
            && self.input.chars().nth(self.pos).unwrap().is_whitespace()
        {
            self.pos += 1;
        }
    }
}
