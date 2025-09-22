// Binary and unary operations with macros
// C ::= Add(C, C) | Mult(C, C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C, C) | MixUnbounded(C, C, C, C)

use crate::traits::Component;
use crate::generator::{generate_component, generate_atom};

macro_rules! binop {
    ($name:ident, $weight:expr, $symbol:literal, $op:expr) => {
        pub struct $name(pub Box<dyn Component>, pub Box<dyn Component>);

        impl Component for $name {
            fn evaluate(&self, x: f64, y: f64) -> f64 {
                let a = self.0.evaluate(x, y);
                let b = self.1.evaluate(x, y);
                ($op)(a, b)
            }

            fn codegen(&self) -> String {
                format!("({} {} {})", self.0.codegen(), $symbol, self.1.codegen())
            }

            fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
                if depth == 0 {
                    return generate_atom(rng);
                }
                Box::new($name(
                    generate_component(depth - 1, rng),
                    generate_component(depth - 1, rng),
                ))
            }

            fn weight() -> f64 {
                $weight
            }
        }
    };
}

macro_rules! unop {
    ($name:ident, $weight:expr, $op:expr) => {
        pub struct $name(pub Box<dyn Component>);

        impl Component for $name {
            fn evaluate(&self, x: f64, y: f64) -> f64 {
                ($op)(self.0.evaluate(x, y))
            }

            fn codegen(&self) -> String {
                format!("{}({})", stringify!($name).to_lowercase(), self.0.codegen())
            }

            fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
                if depth == 0 {
                    return generate_atom(rng);
                }
                Box::new($name(generate_component(depth - 1, rng)))
            }

            fn weight() -> f64 {
                $weight
            }
        }
    };
}

binop!(Add, 0.15, "+", |a: f64, b: f64| a + b);
binop!(Mult, 0.15, "*", |a: f64, b: f64| a * b);
binop!(Div, 0.1, "/", |a: f64, b: f64| if b.abs() < 1e-10 { 1.0 } else { a / b });

unop!(Sin, 0.1, |x: f64| x.sin());
unop!(Cos, 0.1, |x: f64| x.cos());
unop!(Exp, 0.05, |x: f64| x.exp().min(1e6));
unop!(Sqrt, 0.05, |x: f64| x.abs().sqrt());

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