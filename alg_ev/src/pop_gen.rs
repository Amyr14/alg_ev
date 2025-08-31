use rand::prelude::*;
use rand::distributions::{Uniform};
use crate::config::{Codification};


enum Population {
    Binary(Vec<Vec<bool>>),
    IntegerPermutation(Vec<Vec<usize>>),
    Integer(Vec<Vec<usize>>),
    Real(Vec<Vec<f64>>)
}

impl Population {
    pub fn generate_population(codification: Codification, pop_size: usize) -> Population {
        let mut rng = rand::thread_rng();

        match codification {
            Codification::Binary { dim } => {
                Population::Binary(
                    (0..pop_size)
                        .map(|_| (0..dim).map(|_| rng.gen_bool(0.5)).collect::<Vec<bool>>())
                        .collect()
                )
            },
            Codification::Integer { dim, bounds } => {
                let (lower, upper) = bounds;
                let uniform_dist = Uniform::from(lower..=upper);
                Population::Integer(
                    (0..pop_size)
                        .map(|_| (0..dim).map(|_| uniform_dist.sample(&mut rng)).collect())
                        .collect()
                )
            },
            Codification::IntegerPermutation { dim } => {
                Population::IntegerPermutation(
                    (0..pop_size)
                        .map(|_| {
                            let mut range = (0..dim).collect::<Vec<usize>>();
                            range.shuffle(&mut rng);
                            range
                        })
                        .collect()
                )
            },
            Codification::Real { dim, bounds } => {
                let (lower, upper) = bounds;
                let uniform_dist = Uniform::from(lower..=upper);

                Population::Real(
                    (0..pop_size)
                        .map(|_| (0..dim).map(|_| uniform_dist.sample(&mut rng)).collect())
                        .collect()
                )
            }
        }
    }
}   

#[cfg(test)]
mod population_tests {
    use super::*;

    #[test]
    fn test_generate_binary_population() {
        let dim = 10;
        let codification = Codification::Binary { dim };
        let pop_size = 15;
        let population = Population::generate_population(codification, pop_size);

        if let Population::Binary(vec) = population {
            assert_eq!(vec.len(), pop_size);
            for solution in vec {
                assert_eq!(solution.len(), dim);
            }
        } else {
            panic!("Expected population to be Population::Binary");
        }
    }

    #[test]
    fn test_generate_integer_population() {
        let dim = 15;
        let bounds = (1, 10);
        let pop_size = 25;
        let codification = Codification::Integer { dim, bounds };
        let population = Population::generate_population(codification, pop_size);
        
        if let Population::Integer(vec) = population {
            assert_eq!(vec.len(), pop_size);
            for solution in vec {
                assert_eq!(solution.len(), dim);
                for gene in solution {
                    let is_in_bound = gene >= bounds.0 && gene <= bounds.1;
                    assert!(is_in_bound);
                }
            }
        } else {
            panic!("Expected population to be Population::Integer");
        }
    }

    #[test]
    fn test_generate_real_population() {
        let dim = 15;
        let bounds = (0.5, 1.5);
        let pop_size = 10;
        let codification = Codification::Real { dim, bounds };
        let population = Population::generate_population(codification, pop_size);
        
        if let Population::Real(vec) = population {
            assert_eq!(vec.len(), pop_size);
            for solution in vec {
                assert_eq!(solution.len(), dim);
                for gene in solution {
                    let is_in_bound = gene >= bounds.0 && gene <= bounds.1;
                    assert!(is_in_bound);
                }
            }
        } else {
            panic!("Expected population to be Population::Real");
        }
    }

    #[test]
    fn test_generate_int_permutation_population() {
        let dim = 15;
        let pop_size = 10;
        let codification = Codification::IntegerPermutation { dim };
        let population = Population::generate_population(codification, pop_size);
        let comparison_vec: Vec<usize> = (0..=15).collect();

        if let Population::IntegerPermutation(vec) = population {
            assert_eq!(vec.len(), pop_size);
            for mut solution in vec {
                assert_eq!(solution.len(), dim);
                solution.sort();
                solution
                    .iter()
                    .enumerate()
                    .for_each(|(i, &val)| assert_eq!(val, comparison_vec[i]));
            }
        } else {
            panic!("Expected population to be Population::IntegerPermutation");
        }
    }
}