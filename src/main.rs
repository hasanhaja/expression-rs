// The return value of eval_expr might need to be Result<f64> or Option.
/// The order of operation should be handled by the structure of the tree.
/// So since the tree is recursive, the structure should enable correctness.
/// However, it does not inherently ensure simplicity of the tree
struct Evaluator {
    expression: String,
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(f64),
    Add,
    Mult,
    //LParen,
    //RParen,
}

#[derive(Debug, PartialEq)]
struct Expr {
    value: Token,
    left: Option<Box<Expr>>,
    right: Option<Box<Expr>>,
}

impl Evaluator {
    fn new(expression: String) -> Evaluator {
        Evaluator {expression}
    }

    /// This function expects an expression seperated by spaces
    fn extract_chars(&self) -> Vec<char> {
        self.expression.chars().into_iter()
            .filter(|x| !x.is_ascii_whitespace())
            .collect()
    }

    /// 1 + 2 -> (1 + 2)
    /// 1 + 2 * 3 -> (1 + (2 * 3))
    fn insert_parens(char_tokens: Vec<char>) -> Vec<char> {

        char_tokens
    }

    /// (1 + 2) -> (+ 1 2)
    /// (1 + (2 * 3)) -> (+ * 2 3 1)
    fn into_polish(char_tokens: Vec<char>) -> Vec<char> {
        char_tokens
    }

    /// (+ * 2 3 1) -> Expr(add, left(1), right(Expr(mult), l(2), r(3)))
    fn polish_to_expr(polish_tokens: Vec<Char>) -> Expr {


        Expr {
            value: Token::Value(1f64),
            left: None,
            right: None,
        }
    }

    fn build_expr(&self) -> Expr {
        Expr {
            value: Token::Value(1f64),
            left: None,
            right: None
        }
    }

    fn eval_expr(expr: Expr) -> f64 {

        match expr {
            Expr {value: value, left: Some(left_expr), right: Some(right_expr) } => {

                let left_res = Evaluator::eval_expr(*left_expr);
                let right_res = Evaluator::eval_expr(*right_expr);

                match value {
                    Token::Value(num) => {
                        // should be an error
                        0f64
                    },
                    Token::Mult => {
                        left_res * right_res
                    },
                    Token::Add => {
                        left_res + right_res
                    },
                }
            },

            Expr {value: value, left: None, right: Some(right_expr)} => {

                let right_res = Evaluator::eval_expr(*right_expr);

                match value {
                    Token::Value(num) => {
                        // should be an error
                        0f64
                    },
                    Token::Mult => {
                        // should be an error
                        0f64
                    },
                    Token::Add => {
                        right_res
                    },
                }
            },

            Expr {value: value, left: Some(left_expr), right: None} => {
                let left_res = Evaluator::eval_expr(*left_expr);

                match value {
                    Token::Value(num) => {
                        // should be an error
                        0f64
                    },
                    Token::Mult => {
                        // should be an error
                        0f64
                    },
                    Token::Add => {
                        left_res
                    },
                }
            },
            Expr {value: Token::Value(result), left: None, right: None} => {
                result
            },
            _ => {
                // The function should return a result
                // Expression is not valid
                println!("Some error has occurred");
                0f64
            }
        }
    }

    fn eval(&self) -> f64 {
        let expr = self.build_expr();
        let result = Evaluator::eval_expr(expr);

        result
        //10.0
    }

}

fn main() {

    let expression = "1 + 2".to_string();

    // Todo: Remove the clone and use reference instead
    let evaluator = Evaluator::new(expression.clone());
    println!("Expression: {}", expression);

    let result = evaluator.eval();
    println!("Result = {}", result);

}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn eval_expr_with_just_value() {
        let expr = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        assert_eq!(2f64, Evaluator::eval_expr(expr));
    }

    #[test]
    fn eval_expr_with_simple_mult() {
        let left = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        let right = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        let expr = Expr {
            value: Token::Mult,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        assert_eq!(4f64, Evaluator::eval_expr(expr));
    }

    #[test]
    fn eval_expr_with_nested_mult() {
        // (1 + 1) + (1 + 1)

        let l_left = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        let l_right = Expr {
            value: Token::Value(3f64),
            left: None,
            right: None,
        };

        let r_left = Expr {
            value: Token::Value(4f64),
            left: None,
            right: None,
        };

        let r_right = Expr {
            value: Token::Value(5f64),
            left: None,
            right: None,
        };

        let left_expr = Expr {
            value: Token::Mult,
            left: Some(Box::new(l_left)),
            right: Some(Box::new(l_right)),
        };

        let right_expr = Expr {
            value: Token::Mult,
            left: Some(Box::new(r_left)),
            right: Some(Box::new(r_right)),
        };

        let expr = Expr {
            value: Token::Mult,
            left: Some(Box::new(left_expr)),
            right: Some(Box::new(right_expr)),
        };

        assert_eq!(120f64, Evaluator::eval_expr(expr));
    }

    #[test]
    fn eval_expr_with_complex_mult() {
        /// The tree is built to represent this: 1 + (2 * 3) + 4
        /// However, an alternative structure would be 1 + 4 + (2 * 3)
        /// and the tree does not handle that
        let l_left = Expr {
            value: Token::Value(1f64),
            left: None,
            right: None,
        };

        let l_right_l = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        let l_right_r = Expr {
            value: Token::Value(3f64),
            left: None,
            right: None,
        };

        let r_left = Expr {
            value: Token::Value(4f64),
            left: None,
            right: None,
        };

        let l_right_expr = Expr {
            value: Token::Mult,
            left: Some(Box::new(l_right_l)),
            right: Some(Box::new(l_right_r)),
        };

        let left_expr = Expr {
            value: Token::Add,
            left: Some(Box::new(l_left)),
            right: Some(Box::new(l_right_expr)),
        };

        let expr = Expr {
            value: Token::Add,
            left: Some(Box::new(left_expr)),
            right: Some(Box::new(r_left)),
        };

        assert_eq!(11f64, Evaluator::eval_expr(expr));
    }

    #[test]
    fn eval_expr_with_simple_add() {

        let left = Expr {
            value: Token::Value(1f64),
            left: None,
            right: None,
        };

        let right = Expr {
            value: Token::Value(1f64),
            left: None,
            right: None,
        };

        let expr = Expr {
            value: Token::Add,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        assert_eq!(2f64, Evaluator::eval_expr(expr));
    }

    #[test]
    fn eval_expr_with_nested_add() {

        // (1 + 1) + (1 + 1)

        let l_left = Expr {
            value: Token::Value(1f64),
            left: None,
            right: None,
        };

        let l_right = Expr {
            value: Token::Value(2f64),
            left: None,
            right: None,
        };

        let r_left = Expr {
            value: Token::Value(3f64),
            left: None,
            right: None,
        };

        let r_right = Expr {
            value: Token::Value(4f64),
            left: None,
            right: None,
        };

        let left_expr = Expr {
            value: Token::Add,
            left: Some(Box::new(l_left)),
            right: Some(Box::new(l_right)),
        };

        let right_expr = Expr {
            value: Token::Add,
            left: Some(Box::new(r_left)),
            right: Some(Box::new(r_right)),
        };

        let expr = Expr {
            value: Token::Add,
            left: Some(Box::new(left_expr)),
            right: Some(Box::new(right_expr)),
        };

        assert_eq!(10f64, Evaluator::eval_expr(expr));
    }
}
