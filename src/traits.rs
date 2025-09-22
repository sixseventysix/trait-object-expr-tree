// Core trait definitions

/// Component trait for grammar elements that can be evaluated and generated
pub trait Component {
    fn evaluate(&self, x: f64, y: f64) -> f64;
    fn codegen(&self) -> String;
    fn generate(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> where Self: Sized;
    fn weight() -> f64 where Self: Sized;
}