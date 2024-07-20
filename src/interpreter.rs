use crate::ast::{Expr, Function, Statement};
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Interpreter {
    functions: HashMap<String, Function>,
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Vec<Statement>) {
        for statement in program {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Function(function) => {
                self.functions.insert(function.name.clone(), function);
            }
            Statement::Expr(expr) => {
                let result = self.evaluate_expr(expr);
                println!("{}", result); // Print the result of the expression
            }
        }
    }

    pub fn evaluate_expr(&mut self, expr: Expr) -> f64 {
        match expr {
            Expr::Number(value) => value,
            Expr::Ident(name) => *self.variables.get(&name).expect("Undefined variable"),
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expr(*left);
                let right_val = self.evaluate_expr(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => panic!("Unexpected operator: {:?}", op),
                }
            }
            Expr::Call(name, args) => {
                let evaluated_args: Vec<f64> = args
                    .into_iter()
                    .map(|arg| self.evaluate_expr(arg))
                    .collect();

                if let Some(function) = self.functions.get(&name).clone() {
                    let function = function.clone();

                    let return_value = self.call_function(&function, evaluated_args).to_owned();
                    return_value
                } else {
                    panic!("Undefined function");
                }
            }
        }
    }

    fn call_function(&mut self, function: &Function, args: Vec<f64>) -> f64 {
        if function.params.len() != args.len() {
            panic!("Incorrect number of arguments");
        }
        let old_vars = self.variables.clone();
        for (param, arg) in function.params.iter().zip(args) {
            self.variables.insert(param.clone(), arg);
        }
        let result = self.evaluate_expr(function.body.clone());
        self.variables = old_vars;
        result
    }
}
