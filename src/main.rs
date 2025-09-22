pub trait ExprNode {
    fn evaluate(&self, x: f64, y: f64) -> f64;
    fn codegen(&self) -> String;
}

pub struct Const(pub f64);
pub struct Var(pub String);

impl ExprNode for Const {
    fn evaluate(&self, _x: f64, _y: f64) -> f64 {
        self.0
    }

    fn codegen(&self) -> String {
        format!("{}", self.0)
    }
}

impl ExprNode for Var {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        match self.0.as_str() {
            "x" => x,
            "y" => y,
            _ => panic!("Unknown variable: {}", self.0),
        }
    }

    fn codegen(&self) -> String {
        self.0.clone()
    }
}

pub struct Add(pub Box<dyn ExprNode>, pub Box<dyn ExprNode>);
pub struct Mul(pub Box<dyn ExprNode>, pub Box<dyn ExprNode>);

impl ExprNode for Add {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y) + self.1.evaluate(x, y)
    }

    fn codegen(&self) -> String {
        format!("({} + {})", self.0.codegen(), self.1.codegen())
    }
}

impl ExprNode for Mul {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y) * self.1.evaluate(x, y)
    }

    fn codegen(&self) -> String {
        format!("({} * {})", self.0.codegen(), self.1.codegen())
    }
}

fn main() {
    let expr: Box<dyn ExprNode> = Box::new(Add(
        Box::new(Mul(
            Box::new(Const(2.0)),
            Box::new(Var("x".to_string())),
        )),
        Box::new(Mul(
            Box::new(Const(3.0)),
            Box::new(Var("y".to_string())),
        )),
    ));

    println!("Codegen: {}", expr.codegen());
    println!("Evaluate with x=1.5, y=2.0: {}", expr.evaluate(1.5, 2.0));
}