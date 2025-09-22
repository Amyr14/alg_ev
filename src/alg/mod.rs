pub mod objectives;
use crate::population::{Encoding, PopGenerator, Population};

pub trait Objective<E: Encoding> {
    type Output;
    fn eval(&self, pop: &Population<E>) -> Self::Output;
} 

pub trait Fitness<E, O>
where
    E: Encoding,
    O: Objective<E>
{
    type Score;
    fn eval(input: O::Output) -> Self::Score;
}

pub struct GeneticAlgorithm<Enc, O, P>
where
    Enc: Encoding,
    O: Objective<Enc>,
    // F: Fitness<Enc, O>,
    P: PopGenerator<E = Enc>,
{
    objective: O,
    // fitness: F,
    pop_generator: P,
    runs: u64,
    generations: u64,
}

impl<Enc, O, P> GeneticAlgorithm<Enc, O, P>
where
    Enc: Encoding,
    O: Objective<Enc>,
    // F: Fitness<Enc, O>,
    P: PopGenerator<E = Enc>,
{
    pub fn run() -> () {
        
    }
}