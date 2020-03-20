// The return value of eval_expr might need to be Result<f64> or Option.
struct Evaluator {
    expression: String,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add
}

#[derive(Debug, PartialEq)]
enum Expression {
    Val(f64),
    Op(Operator),
    Branch(Box<Expression>, Box<Expression>),
}

impl Evaluator {
    fn new(expression: String) -> Evaluator {
        Evaluator {expression}
    }

    fn build_expr(&self) -> Expression {
        Expression::Val(1f64)
    }

    fn eval_expr(expr: Expression) -> f64 {
        match expr {
            Expression::Val(result) => result,
            Expression::Op(op) => {

            }
            Expression::Branch(l, r) => {

            }
        }
    }


    fn eval(&self) -> f64 {
        let expr = build_expr();
        let result = eval_expr(expr);

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
    fn expression_builder_test() {

        let sub_node = Expression::Branch(Box::new(Expression::Val(1f64)), Box::new(Expression::Val(2f64)));
        let expr = Expression::Branch(Box::new(Expression::Op(Operator::Add)), Box::new(sub_node));

        let placeholder = Expression::Val(1f64);
        let evaluator = Evaluator::new("1 + 2".to_string());

        // Build expression manually, and test if the build Expression is the same
        assert_eq!(placeholder, evaluator.build_expr());
    }

    #[test]
    fn it_runs() {
        let expression = "1 + 2".to_string();

        // Todo: Remove the clone and use reference instead
        let evaluator = Evaluator::new(expression.clone());
        println!("Expression: {}", expression);

        let result = evaluator.eval();
        println!("Result = {}", result);
    }

}
