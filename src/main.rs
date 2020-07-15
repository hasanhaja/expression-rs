// The return value of eval_expr might need to be Result<f64> or Option.
/// The order of operation should be handled by the structure of the tree.
/// So since the tree is recursive, the structure should enable correctness.
/// However, it does not inherently ensure simplicity of the tree
///
/// Decimal numbers can't be represented as a char, there it must be a string to &str
struct Evaluator {
    expression: String,
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(f64),
    Add,
    Subtract,
    Divide,
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

    /// This function takes an expression, and tokenizes every char into a vector element.
    /// i.e. "12 + 232" -> ["1", "2", "+", "2", "3", "2"]
    /// and not "12 + 232" -> ["12", "+", "232"]
    fn extract_chars(&self) -> Vec<char> {
        self.expression.chars().into_iter()
            .filter(|x| !x.is_ascii_whitespace())
            .collect()
    }

    /// This takes a vector of individual chars and it groups them into numbers and operators
    fn extract_chars_grouped(&self) -> Vec<String> {
        let chars = self.extract_chars();

        let mut grouped: Vec<String> = Vec::new();

        let mut temp = "".to_string();

        for (i, char) in chars.iter().enumerate() {
            match char {
                '+' | '-' | '*' | '/' => {
                    if !temp.is_empty() {
                        grouped.push(temp.clone());
                        temp = "".to_string();
                    }

                    grouped.push(char.to_string());
                },
                _ => {
                    temp.push(*char);

                    if i == chars.len() - 1 {
                        grouped.push(temp.clone());
                    }
                },
            }
        }
        grouped
    }

    /// This function takes an expression, and tokenizes every number into a vector element.
    /// i.e. "12 + 232" -> ["12", "+", "232"]
    /// and not "12 + 232" -> ["1", "2", "+", "2", "3", "2"]
    /// /// and not ["1", "2", "+", "2", "3", "2"] -> ["12", "+", "232"]
    fn extract_tokens(&self) -> Vec<Token> {
        let chars = self.extract_chars_grouped();

        // let mut tokens: Vec<Token> = Vec::new();

        let mut tokens: Vec<Token> = chars.iter()
            .map(|symbol|
                match symbol.as_str() {
                "+" | "-" | "*" | "/" => {
                    let op = Evaluator::select_operator(&symbol.as_str().chars().last().unwrap()).unwrap();
                    op
                },
                _ => {
                    let value: f64 = symbol.parse().unwrap();
                    Token::Value(value)
                }
            })
            .collect();



        // append the chars unless it's a token

        tokens
    }

    /// 1 + 2 -> (1 + 2)
    /// 1 + 2 * 3 -> (1 + (2 * 3))
    fn insert_parens(tokens: Vec<Token>) -> Vec<Token> {

        let mut resultant: Vec<Token> = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            if i == 0 {
                match token {
                    Token::Value(_) => todo!("Parentheses needs implementing"),
                    _ => todo!()
                }
            }

            match token {
                _ => todo!("Find an operator and put a parentheses surrounding the elements around it.")
            }
        }

        tokens
    }

    /// (1 + 2) -> (+ 1 2)
    /// (1 + (2 * 3)) -> (+ * 2 3 1)
    fn into_polish(char_tokens: Vec<char>) -> Vec<char> {
        // TODO pending implementation
        char_tokens
    }

    fn select_operator(token: &char) -> Option<Token> {
        match token {
            '+' => Some(Token::Add),
            '*' => Some(Token::Mult),
            '-' => Some(Token::Subtract),
            '/' => Some(Token::Divide),
            _ => None,
        }
    }

    fn is_operator(token: &char) -> bool {
        match Evaluator::select_operator(token) {
            Some(_) => true,
            None => false,
        }
    }

    /// This will have to be a recursive function
    /// (+ * 2 3 1) -> Expr(add, left(1), right(Expr(mult), l(2), r(3)))
    fn polish_to_expr(polish_tokens: Vec<char>) -> Expr {

        // Todo: if the vec is empty, return error
        // if the vec is of size 1 and is value, then return value wrapped in Expr

        // Todo: strip parentheses, drop first and last elems
        // Should the input be mutable, or can we just slice it?

        let op: Token;

        let first = polish_tokens.first().unwrap();
        let last = polish_tokens.last().unwrap();


        if !Evaluator::is_operator(first) {

            // parse the first elem and convert it to f64

            Expr {
                value: Token::Value(1f64),
                left: None,
                right: None,
            }
        } else {
            Expr {
                value: Token::Add,  // Logic to identify and select the correct operator
                left: None,         // recursively pass the remainder of the vec here
                right: None,        // pop last here
            }
        }


    }

    // TODO this is placeholder
    fn build_expr(&self) -> Expr {
        Expr {
            value: Token::Value(0f64),
            left: None,
            right: None
        }
    }

    // TODO: This function is hacked on and there is a lot of repetition.
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
                    Token::Subtract => {
                        0f64
                    },

                    Token::Divide => {
                        0f64
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

                    Token::Subtract => {
                        0f64
                    },

                    Token::Divide => {
                        0f64
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

                    Token::Subtract => {
                        0f64
                    },

                    Token::Divide => {
                        0f64
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

    let expression = "12.12 + 232 - 2.1".to_string();

    // Todo: Remove the clone and use reference instead
    // let evaluator = Evaluator::new(expression.clone());
    // println!("Expression: {}", expression);
    //
    // let result = evaluator.eval();
    // println!("Result = {}", result);

    let eval = Evaluator::new(expression.clone());

    println!("{:?}", eval.extract_chars_grouped());

    println!("{:?}", eval.extract_tokens());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn extract_char_with_spaces() {
        let expr = "12 + 2";
        let eval = Evaluator::new(expr.to_string());

        assert_eq!(3usize, eval.extract_chars().len());
    }

    #[test]
    fn extract_char_without_spaces() {
        let expr = "1+2";
        let eval = Evaluator::new(expr.to_string());

        assert_eq!(3usize, eval.extract_chars().len());
    }

    #[test]
    fn tokenize_string_to_vector_of_chars() {
        let expression = "12 + 232".to_string();
        let result = vec!['1', '2', '+', '2', '3', '2'];

        let eval = Evaluator::new(expression.clone());

        assert_eq!(result, eval.extract_chars());
    }

    #[test]
    fn tokenize_string_to_vector_of_string() {
        let expression = "12 + 232".to_string();
        let result = vec!["12".to_string(), "+".to_string(), "232".to_string()];

        let eval = Evaluator::new(expression.clone());

        assert_eq!(result, eval.extract_chars_grouped());
    }

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
