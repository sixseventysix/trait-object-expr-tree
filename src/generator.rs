// PCFG generation logic

use crate::traits::Component;
use crate::ast::atoms::{VarX, VarY, RandomConst};
use crate::ast::operations::{Add, Mult, Div, Sin, Cos, Exp, Sqrt, MixUnbounded};
use rand::Rng;

/// Generate a random atom (A production rule)
pub fn generate_atom(rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
    let generators: Vec<(f64, fn(usize, &mut rand::rngs::ThreadRng) -> Box<dyn Component>)> = vec![
        (VarX::weight(), VarX::generate),
        (VarY::weight(), VarY::generate),
        (RandomConst::weight(), RandomConst::generate),
    ];

    let total_weight: f64 = generators.iter().map(|(w, _)| w).sum();
    let mut choice = rng.gen_range(0.0..total_weight);

    for (weight, generator) in generators {
        choice -= weight;
        if choice <= 0.0 {
            return generator(0, rng); // depth doesn't matter for atoms
        }
    }
    RandomConst::generate(0, rng) // fallback
}

/// Generate a random component (C production rule)
pub fn generate_component(depth: usize, rng: &mut rand::rngs::ThreadRng) -> Box<dyn Component> {
    // C ::= A | Add(C,C) | Mult(C,C) | Sin(C) | ...
    let generators: Vec<(f64, fn(usize, &mut rand::rngs::ThreadRng) -> Box<dyn Component>)> = vec![
        // A (atoms) - one choice in C rule
        (0.4, |_, rng| generate_atom(rng)),
        // Operations (only if depth > 0)
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
        return generate_atom(rng); // fallback to atoms
    }

    let mut choice = rng.gen_range(0.0..total_weight);
    for (weight, generator) in generators {
        choice -= weight;
        if choice <= 0.0 {
            return generator(depth, rng);
        }
    }
    generate_atom(rng) // fallback
}