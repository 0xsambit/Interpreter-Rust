#[cfg(test)]
mod tests {
    // use super::*;
    use crate::ast::{Expr, Function, Statement};
    use crate::interpreter::Interpreter;
    use crate::lexer::{Lexer, Token};
    use crate::parser::Parser;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn test_lexer_basic() {
        let mut lexer = Lexer::new("function add(x, y) (x + y)".to_string());
        assert_eq!(lexer.next_token(), Token::Function);
        assert_eq!(lexer.next_token(), Token::Ident("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Ident("y".to_string()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Ident("y".to_string()));
        assert_eq!(lexer.next_token(), Token::RParen);
    }

    #[test]
    fn test_parser_function() {
        let input = "function add(x, y) (x + y)".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        if let Statement::Function(Function { name, params, body }) = &program[0] {
            assert_eq!(name, "add");
            assert_eq!(params, &vec!["x".to_string(), "y".to_string()]);
            if let Expr::BinaryOp(left, op, right) = body {
                assert_eq!(**left, Expr::Ident("x".to_string()));
                assert_eq!(*op, Token::Plus);
                assert_eq!(**right, Expr::Ident("y".to_string()));
            } else {
                panic!("Expected binary operation");
            }
        } else {
            panic!("Expected function definition");
        }
    }

    #[test]
    fn test_interpreter_arithmetic() {
        let input = "function add(x, y) (x + y) add(2, 3)".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut interpreter = Interpreter::new();
        interpreter.interpret(program);

        assert_eq!(
            interpreter.evaluate_expr(Expr::Call(
                "add".to_string(),
                vec![Expr::Number(2.0), Expr::Number(3.0)]
            )),
            5.0
        );
    }

    #[test]
    fn test_invalid_function_call() {
        let input = "invalid(2, 3)".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut interpreter = Interpreter::new();
        let result = catch_unwind(AssertUnwindSafe(|| {
            interpreter.interpret(program);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_syntax_error() {
        let input = "function invalid_syntax(x, y) (x + y".to_string();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let result = catch_unwind(AssertUnwindSafe(|| {
            parser.parse_program();
        }));
        assert!(result.is_err());
    }
}
