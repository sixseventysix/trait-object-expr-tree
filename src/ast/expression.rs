// Root expression structure
// E ::= (C, C, C)

use crate::traits::Component;
use crate::generator::generate_component;

/// Root expression containing three components
pub struct Expression(
    pub Box<dyn Component>,
    pub Box<dyn Component>,
    pub Box<dyn Component>,
);

impl Expression {
    /// Generate a new random expression
    pub fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Self {
        Expression(
            generate_component(depth, rng),
            generate_component(depth, rng),
            generate_component(depth, rng),
        )
    }

    /// Evaluate the expression returning a 3-tuple
    pub fn evaluate(&self, x: f64, y: f64) -> (f64, f64, f64) {
        (
            self.0.evaluate(x, y),
            self.1.evaluate(x, y),
            self.2.evaluate(x, y),
        )
    }

    /// Generate code representation
    pub fn codegen(&self) -> String {
        format!(
            "({}, {}, {})",
            self.0.codegen(),
            self.1.codegen(),
            self.2.codegen()
        )
    }
}