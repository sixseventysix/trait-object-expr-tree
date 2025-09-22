// PCFG generation logic

use crate::traits::Component;
use crate::ast::atoms::{VarX, VarY, RandomConst};
use crate::ast::operations::{Add, Mult, Div, Sin, Cos, Exp, Sqrt, MixUnbounded};
use rand::Rng;

/// Generate a random component using weighted sampling
pub fn generate_component(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
    let generators: Vec<(f64, fn(usize, &mut rand::rngs::ThreadRng) -> Box<dyn Component>)> = vec![
        // Atoms (always available)
        (VarX::weight(), VarX::generate),
        (VarY::weight(), VarY::generate),
        (RandomConst::weight(), RandomConst::generate),
        // Complex operations (only if depth > 0)
        (if depth > 0 { Add::weight() } else { 0.0 }, Add::generate),
        (if depth > 0 { Mult::weight() } else { 0.0 }, Mult::generate),
        (if depth > 0 { Div::weight() } else { 0.0 }, Div::generate),
        (if depth > 0 { Sin::weight() } else { 0.0 }, Sin::generate),
        (if depth > 0 { Cos::weight() } else { 0.0 }, Cos::generate),
        (if depth > 0 { Exp::weight() } else { 0.0 }, Exp::generate),
        (if depth > 0 { Sqrt::weight() } else { 0.0 }, Sqrt::generate),
        (if depth > 0 { MixUnbounded::weight() } else { 0.0 }, MixUnbounded::generate),
    ];

    let total_weight: f64 = generators.iter().map(|(w, _)| w).sum();
    if total_weight == 0.0 {
        return RandomConst::generate(depth, rng);
    }

    let mut choice = rng.gen_range(0.0..total_weight);
    for (weight, generator) in generators {
        choice -= weight;
        if choice <= 0.0 {
            return generator(depth, rng);
        }
    }
    RandomConst::generate(depth, rng)
}