// Binary and unary operations
// C ::= Add(C, C) | Mult(C, C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C, C) | MixUnbounded(C, C, C, C)

use crate::traits::Component;
use crate::generator::{generate_component, generate_atom};

// Binary operations
pub struct Add(pub Box<dyn Component>, pub Box<dyn Component>);
pub struct Mult(pub Box<dyn Component>, pub Box<dyn Component>);
pub struct Div(pub Box<dyn Component>, pub Box<dyn Component>);

impl Component for Add {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y) + self.1.evaluate(x, y)
    }

    fn codegen(&self) -> String {
        format!("({} + {})", self.0.codegen(), self.1.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Add(
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.15
    }
}

impl Component for Mult {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y) * self.1.evaluate(x, y)
    }

    fn codegen(&self) -> String {
        format!("({} * {})", self.0.codegen(), self.1.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Mult(
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.15
    }
}

impl Component for Div {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        let denominator = self.1.evaluate(x, y);
        if denominator.abs() < 1e-10 {
            1.0
        } else {
            self.0.evaluate(x, y) / denominator
        }
    }

    fn codegen(&self) -> String {
        format!("({} / {})", self.0.codegen(), self.1.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Div(
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.1
    }
}

// Unary operations
pub struct Sin(pub Box<dyn Component>);
pub struct Cos(pub Box<dyn Component>);
pub struct Exp(pub Box<dyn Component>);
pub struct Sqrt(pub Box<dyn Component>);

impl Component for Sin {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y).sin()
    }

    fn codegen(&self) -> String {
        format!("sin({})", self.0.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Sin(generate_component(depth - 1, rng)))
    }

    fn weight() -> f64 {
        0.1
    }
}

impl Component for Cos {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y).cos()
    }

    fn codegen(&self) -> String {
        format!("cos({})", self.0.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Cos(generate_component(depth - 1, rng)))
    }

    fn weight() -> f64 {
        0.1
    }
}

impl Component for Exp {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y).exp().min(1e6) // Clamp to prevent overflow
    }

    fn codegen(&self) -> String {
        format!("exp({})", self.0.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Exp(generate_component(depth - 1, rng)))
    }

    fn weight() -> f64 {
        0.05
    }
}

impl Component for Sqrt {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(x, y).abs().sqrt()
    }

    fn codegen(&self) -> String {
        format!("sqrt(abs({}))", self.0.codegen())
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(Sqrt(generate_component(depth - 1, rng)))
    }

    fn weight() -> f64 {
        0.05
    }
}

// Complex operation
pub struct MixUnbounded(
    pub Box<dyn Component>,
    pub Box<dyn Component>,
    pub Box<dyn Component>,
    pub Box<dyn Component>,
);

impl Component for MixUnbounded {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        let a = self.0.evaluate(x, y);
        let b = self.1.evaluate(x, y);
        let c = self.2.evaluate(x, y);
        let d = self.3.evaluate(x, y);
        (a * b + c * d) / (1.0 + a.abs() + b.abs())
    }

    fn codegen(&self) -> String {
        format!(
            "mix({}, {}, {}, {})",
            self.0.codegen(),
            self.1.codegen(),
            self.2.codegen(),
            self.3.codegen()
        )
    }

    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        if depth == 0 {
            return generate_atom(rng);
        }
        Box::new(MixUnbounded(
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
            generate_component(depth - 1, rng),
        ))
    }

    fn weight() -> f64 {
        0.05
    }
}