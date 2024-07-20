use crate::ast::{Expr, Function, Statement};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token(&mut self, token: Token) {
        if self.current_token == token {
            self.next_token();
        } else {
            panic!("Expected {:?}, got {:?}", token, self.current_token);
        }
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token {
            Token::Function => self.parse_function(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_function(&mut self) -> Statement {
        self.expect_token(Token::Function);
        let name = if let Token::Ident(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            panic!("Expected function name, got {:?}", self.current_token);
        };
        self.expect_token(Token::LParen);
        let mut params = Vec::new();
        if self.current_token != Token::RParen {
            params.push(self.parse_ident());
            while self.current_token == Token::Comma {
                self.next_token();
                params.push(self.parse_ident());
            }
        }
        self.expect_token(Token::RParen);
        self.expect_token(Token::LParen);
        let body = self.parse_expr();
        self.expect_token(Token::RParen);
        Statement::Function(Function { name, params, body })
    }

    fn parse_expr_statement(&mut self) -> Statement {
        let expr = self.parse_expr();
        Statement::Expr(expr)
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Expr {
        let mut node = self.parse_mul_div();
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_mul_div();
            node = Expr::BinaryOp(Box::new(node), op, Box::new(right));
        }
        node
    }

    fn parse_mul_div(&mut self) -> Expr {
        let mut node = self.parse_primary();
        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_primary();
            node = Expr::BinaryOp(Box::new(node), op, Box::new(right));
        }
        node
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token.clone() {
            Token::Number(value) => {
                self.next_token();
                Expr::Number(value)
            }
            Token::Ident(name) => {
                self.next_token();
                if self.current_token == Token::LParen {
                    self.next_token();
                    let mut args = Vec::new();
                    if self.current_token != Token::RParen {
                        args.push(self.parse_expr());
                        while self.current_token == Token::Comma {
                            self.next_token();
                            args.push(self.parse_expr());
                        }
                    }
                    self.expect_token(Token::RParen);
                    Expr::Call(name, args)
                } else {
                    Expr::Ident(name)
                }
            }
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expr();
                self.expect_token(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_ident(&mut self) -> String {
        if let Token::Ident(name) = self.current_token.clone() {
            self.next_token();
            name
        } else {
            panic!("Expected identifier, got {:?}", self.current_token);
        }
    }
}
