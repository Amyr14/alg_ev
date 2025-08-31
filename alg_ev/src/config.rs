use serde::{Deserialize};
use serde_json::{error::Category, Deserializer};
use std::{io::Read};

#[derive(Deserialize, Debug, PartialEq)]
pub enum Codification {
    #[serde(rename="binary")]
    Binary {dim: usize},
    #[serde(rename="integer_permutation")]
    IntegerPermutation {dim: usize},
    #[serde(rename="integer")]
    Integer {dim: usize, bounds: (usize, usize)},
    #[serde(rename="real")]
    Real {dim: usize, bounds: (f64, f64)},
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub codification: Codification,
    pub pop_size: usize,
    pub runs: usize,
    pub generations: usize,
}

//@TODO: tratar melhor esse category, talvez fazer um erro customizado
impl Config {
    pub fn from_reader<R: Read>(config_reader: R) -> Result<Config, Category> {
        let mut de = Deserializer::from_reader(config_reader);
        let config = Config::deserialize(&mut de);
        match config {
            Err(e) => Err(e.classify()),
            Ok(config) => Ok(config)
        }
    }
}

#[cfg(test)]
mod config_tests {
    use std::io::Cursor;
    use super::*;
    
    #[test]
    fn test_create_config_from_reader() -> () {
        let config_str = r#"{
            "codification": {
                "integer": { 
                    "dim": 12,
                    "bounds": [0, 10]
                }
            },
            "pop_size": 30,
            "runs": 10,
            "generations": 200
        }"#;
        let config_reader = Cursor::new(config_str);
        let config = Config::from_reader(config_reader)
            .inspect_err(|e| {
                println!("Error: {:?}", e);
            })
            .unwrap();

        let expected_config = Config {
            codification: Codification::Integer {
                dim: 12,
                bounds: (0, 10)
            },
            pop_size: 30,
            runs: 10,
            generations: 200, 
        };

        assert_eq!(config, expected_config);
    }
}