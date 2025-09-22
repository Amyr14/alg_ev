use rand::prelude::*;
use rand::distributions::{Uniform};

// ======================================================================
// == Encoding Marker Trait & Implementations
// ======================================================================

pub trait Encoding: Clone {}

#[derive(Clone)]
pub struct BinaryEncoding(pub Vec<bool>);
impl Encoding for BinaryEncoding {}
impl BinaryEncoding {
    pub fn to_bool_slice(&self) -> &[bool] {
        &self.0
    }
}

#[derive(Clone)]
pub struct IntPermEncoding(pub Vec<usize>);
impl Encoding for IntPermEncoding {}

#[derive(Clone)]
pub struct IntegerEncoding(pub Vec<u64>);
impl Encoding for IntegerEncoding {}

#[derive(Clone)]
pub struct RealEncoding(pub Vec<f64>);
impl Encoding for RealEncoding {}


// ======================================================================
// == Population struct, PopGenerator Trait & Implementations
// ======================================================================

pub struct Population<E: Encoding>(pub Vec<E>);
impl<E: Encoding> Population<E> {
    pub fn get_individuals(&self) -> &[E] {
        &self.0
    }
}

pub trait PopGenerator {
    type E: Encoding;
    fn gen_pop(&self) -> Population<Self::E>;
}

// ============ BinaryPopGenerator ============ 

pub struct BinaryPopGenerator {
    dim: usize,
    pop_size: usize,
}

impl PopGenerator for BinaryPopGenerator {
    type E = BinaryEncoding;
    fn gen_pop(&self) -> Population<BinaryEncoding> {
        let mut rng = rand::thread_rng();
        Population(
            (0..self.pop_size)
                .map(|_| {
                    let binary_samples = (0..self.dim).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>();
                    BinaryEncoding(binary_samples)
                })
                .collect()
        )
    }
}

// ============ IntegerPopGenerator ============
pub struct IntegerPopGenerator {
    dim: usize,
    bounds: (u64, u64),
    pop_size: usize,
}

impl PopGenerator for IntegerPopGenerator {
    type E = IntegerEncoding;
    fn gen_pop(&self) -> Population<IntegerEncoding> {
        let (lower, upper) = self.bounds;
        let mut rng = rand::thread_rng();
        let uniform_dist = Uniform::from(lower..=upper);
        Population(
            (0..self.pop_size)
                .map(|_| {
                    let int_samples = (0..self.dim).map(|_| uniform_dist.sample(&mut rng)).collect();
                    IntegerEncoding(int_samples)
                })
                .collect()
        )
    }
}

// ============ IntPermPopGenerator ============
pub struct IntPermPopGenerator {
    dim: usize,
    pop_size: usize,
}

impl PopGenerator for IntPermPopGenerator {
    type E = IntPermEncoding;
    fn gen_pop(&self) -> Population<IntPermEncoding> {
        let mut rng = rand::thread_rng();
        Population(
            (0..self.pop_size)
                .map(|_| {
                    let mut range = (0..self.dim).collect::<Vec<usize>>();
                    range.shuffle(&mut rng);
                    IntPermEncoding(range)
                })
                .collect()
        )
    }
}

// ============ RealPopGenerator ============
pub struct RealPopGenerator {
    dim: usize,
    bounds: (f64, f64),
    pop_size: usize,
}

impl PopGenerator for RealPopGenerator {
    type E = RealEncoding;
    fn gen_pop(&self) -> Population<RealEncoding> {
        let (lower, upper) = self.bounds;
        let uniform_dist = Uniform::from(lower..=upper);
        let mut rng = rand::thread_rng();
        Population(
            (0..self.pop_size)
                .map(|_| {
                    let real_samples = (0..self.dim).map(|_| uniform_dist.sample(&mut rng)).collect();
                    RealEncoding(real_samples)
                })
                .collect()
        )
    }
}

#[cfg(test)]
mod population_tests {
    use super::*;

    #[test]
    fn test_generate_binary_population() {
        let dim = 17;
        let pop_size = 120;
        let pop_generator = BinaryPopGenerator { dim, pop_size };
        let population = pop_generator.gen_pop();
        let individuals = population.0;
        assert_eq!(individuals.len(), pop_size);

        for individual in individuals {
            assert_eq!(individual.0.len(), dim);
        }
    }

    #[test]
    fn test_generate_integer_population() {
        let dim = 15;
        let bounds = (1, 10);
        let pop_size = 25;
        let pop_generator = IntegerPopGenerator { dim, bounds, pop_size };
        let population = pop_generator.gen_pop();
        let individuals = population.0;
        assert_eq!(individuals.len(), pop_size);
        
        for individual in individuals {
            assert_eq!(individual.0.len(), dim);
            for gene in individual.0 {
                assert!(gene >= bounds.0 && gene <= bounds.1);
            }
        }
    }

    #[test]
    fn test_generate_real_population() {
        let dim = 12;
        let bounds = (53.2, 105.1);
        let pop_size = 110;
        let pop_generator = RealPopGenerator { dim, bounds, pop_size };
        let population = pop_generator.gen_pop();
        let individuals = population.0;
        assert_eq!(individuals.len(), pop_size);
        
        for individual in individuals {
            assert_eq!(individual.0.len(), dim);
            for gene in individual.0 {
                assert!(gene >= bounds.0 && gene <= bounds.1);
            }
        }
    }

    #[test]
    fn test_generate_int_permutation_population() {
        let dim = 15;
        let pop_size = 10;
        let pop_generator = IntPermPopGenerator { dim, pop_size };
        let population = pop_generator.gen_pop();
        let individuals = population.0;
        let comparison_vec: Vec<usize> = (0..=dim).collect();
        assert_eq!(individuals.len(), pop_size);

        for individual in individuals {
            let mut genes = individual.0;
            assert_eq!(genes.len(), dim);
            genes.sort();
            genes
                .iter()
                .enumerate()
                .for_each(|(i, &val)| assert_eq!(val, comparison_vec[i]));
        }
    }
}