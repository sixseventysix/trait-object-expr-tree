// Terminal nodes (atoms) in the grammar
// A ::= x | y | random number in [-1, 1]

use crate::traits::Component;
use rand::Rng;

/// Variable X atom
pub struct VarX;

impl Component for VarX {
    fn evaluate(&self, x: f64, _y: f64) -> f64 {
        x
    }

    fn codegen(&self) -> String {
        "x".to_string()
    }

    fn generate(_depth: usize, _rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        Box::new(VarX)
    }

    fn weight() -> f64 {
        0.33
    }
}

/// Variable Y atom
pub struct VarY;

impl Component for VarY {
    fn evaluate(&self, _x: f64, y: f64) -> f64 {
        y
    }

    fn codegen(&self) -> String {
        "y".to_string()
    }

    fn generate(_depth: usize, _rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        Box::new(VarY)
    }

    fn weight() -> f64 {
        0.33
    }
}

/// Random constant in [-1, 1]
pub struct RandomConst(pub f64);

impl Component for RandomConst {
    fn evaluate(&self, _x: f64, _y: f64) -> f64 {
        self.0
    }

    fn codegen(&self) -> String {
        format!("{:.3}", self.0)
    }

    fn generate(_depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
        Box::new(RandomConst(rng.gen_range(-1.0..1.0)))
    }

    fn weight() -> f64 {
        0.33
    }
}