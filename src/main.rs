pub trait ExprNode {
    fn evaluate(&self, x: f64, y: f64) -> f64;
    fn codegen(&self) -> String;
    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> where Self: Sized;
    fn weight() -> f64 where Self: Sized;
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

    fn generate(_depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> {
        Box::new(Const(rng.gen_range(-1.0..1.0)))
    }

    fn weight() -> f64 {
        0.1
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

    fn generate(_depth: usize, _rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> {
        let vars = ["x", "y"];
        Box::new(Var(vars[rand::random::<usize>() % vars.len()].to_string()))
    }

    fn weight() -> f64 {
        0.1
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

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> {
        if depth == 0 {
            return Const::generate(depth, rng);
        }
        Box::new(Add(
            generate_random_expr(depth - 1, rng),
            generate_random_expr(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.4
    }
}

impl ExprNode for Mul {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y) * self.1.evaluate(x, y)
    }

    fn codegen(&self) -> String {
        format!("({} * {})", self.0.codegen(), self.1.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> {
        if depth == 0 {
            return Const::generate(depth, rng);
        }
        Box::new(Mul(
            generate_random_expr(depth - 1, rng),
            generate_random_expr(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.4
    }
}

use rand::Rng;

fn generate_random_expr(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode> {
    let generators: Vec<(f64, fn(usize, &mut rand::rngs::ThreadRng) -> Box<dyn ExprNode>)> = vec![
        (Const::weight(), Const::generate),
        (Var::weight(), Var::generate),
        (if depth > 0 { Add::weight() } else { 0.0 }, Add::generate),
        (if depth > 0 { Mul::weight() } else { 0.0 }, Mul::generate),
    ];

    let total_weight: f64 = generators.iter().map(|(w, _)| w).sum();
    if total_weight == 0.0 {
        return Const::generate(depth, rng);
    }

    let mut choice = rng.gen_range(0.0..total_weight);
    for (weight, generator) in generators {
        choice -= weight;
        if choice <= 0.0 {
            return generator(depth, rng);
        }
    }
    Const::generate(depth, rng)
}


fn main() {
    let mut rng = rand::thread_rng();

    for i in 0..3 {
        let generated_expr = generate_random_expr(20, &mut rng);
        println!("Expression {}: {}", i + 1, generated_expr.codegen());
        println!("Evaluates to: {}", generated_expr.evaluate(0.6, 0.2));
    }
}