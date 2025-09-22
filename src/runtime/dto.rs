use serde::Deserialize;
use serde_json::{error::Category, Deserializer};
use std::{io::Read};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag="type")]
pub enum EncodingDTO {
    Binary {dim: usize},
    IntegerPermutation {dim: usize},
    Integer {dim: usize, bounds: (usize, usize)},
    Real {dim: usize, bounds: (f64, f64)},
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ConfigDTO {
    pub encoding: EncodingDTO,
    pub pop_size: usize,
    pub runs: usize,
    pub generations: usize,
}

//@TODO: tratar melhor esse category, talvez fazer um erro customizado
impl ConfigDTO {
    pub fn from_reader<R: Read>(config_reader: R) -> Result<ConfigDTO, Category> {
        let mut de: Deserializer<serde_json::de::IoRead<R>> = Deserializer::from_reader(config_reader);
        let config = ConfigDTO::deserialize(&mut de);
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

    fn assert_json_generates_expected_config(json: &str, expected_config: ConfigDTO) {
        let config_reader = Cursor::new(json);
        let config = ConfigDTO::from_reader(config_reader)
            .inspect_err(|e| {
                println!("Error: {:?}", e);
            })
            .unwrap();
        assert_eq!(config, expected_config);
    }
    
    #[test]
    fn test_create_config_from_reader() -> () {
        let config_json = r#"{
            "encoding": {
                "type": "Integer",
                "dim": 12,
                "bounds": [0, 10]
            },
            "pop_size": 30,
            "runs": 10,
            "generations": 200
        }"#;
        let expected_config = ConfigDTO {
            encoding: EncodingDTO::Integer {
                dim: 12,
                bounds: (0, 10)
            },
            pop_size: 30,
            runs: 10,
            generations: 200, 
        };
        assert_json_generates_expected_config(config_json, expected_config);
    }
}