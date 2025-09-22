use trait_object_expr_tree::Expression;
use rand;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Complex Grammar Expression Generation:");
    println!("E ::= (C, C, C)");
    println!("C ::= A | Add(C,C) | Mult(C,C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C,C) | MixUnbounded(C,C,C,C)");
    println!("A ::= x | y | random[-1,1]");
    println!();

    let expr = Expression::generate(20, &mut rng);
    println!("Expression: {}", expr.codegen());
    let result = expr.evaluate(0.6, 0.2);
    println!("Evaluates to: ({:.6}, {:.6}, {:.6})", result.0, result.1, result.2);
    println!();
}