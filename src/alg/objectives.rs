use std::io::{self, BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::collections::HashSet;
use crate::alg::Objective;
use crate::population::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Literal {
    Var(u64),
    NegatedVar(u64),
}

#[derive(Debug, Clone)]
pub struct Clause(Vec<Literal>);
impl Clause {
    fn literals(&self) -> &[Literal] {
        &self.0
    }

    fn as_set(&self) -> HashSet<Literal> {
        self.0.iter().cloned().collect()
    }

    fn evaluate(&self, valoration: &[bool]) -> Option<bool> {
        let evaluation_opt: Option<Vec<bool>> = self.literals()
            .iter()
            .map(|literal| {
                match literal {
                    Literal::Var(index) => valoration.get((index - 1) as usize).copied(),
                    Literal::NegatedVar(index) => valoration.get((index - 1) as usize).map(|v| !*v)
                }
            })
            .collect();

        match evaluation_opt {
            Some(evaluation) => Some(evaluation.iter().any(|v| *v)),
            None => None
        }
    }
}

impl PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        let lit_set_1 = self.as_set();
        let lit_set_2 = other.as_set();
        lit_set_1.eq(&lit_set_2)
    }
}

#[derive(Debug)]
pub struct Formula {
    num_vars: u64,
    num_clauses: u64,
    clauses: Vec<Clause>,
}

#[derive(Debug)]
pub enum FormulaParsingError {
    IO(io::Error),
    Parsing(ParseIntError),
    NoHeader,
    InvalidHeader,
    EmptyClause,
    InconsistentNumOfVars,
    InconsistentNumOfClauses,
    VarOutOfBounds,
}

impl From<io::Error> for FormulaParsingError {
    fn from(value: io::Error) -> Self {
        FormulaParsingError::IO(value)
    }
}

impl From<ParseIntError> for FormulaParsingError {
    fn from(value: ParseIntError) -> Self {
        FormulaParsingError::Parsing(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FormulaEvaluation {
    pub solved: bool,
    pub num_true: usize,
    pub num_false: usize,
}

impl Formula {
    pub fn evaluate(&self, valoration: &[bool]) -> Option<FormulaEvaluation> {
        if self.num_vars as usize != valoration.len()
        { return None }

        let evaluation_opt: Option<Vec<bool>> = self.clauses
            .iter()
            .map(|clause| clause.evaluate(valoration))
            .collect();

        match evaluation_opt {
            Some(evaluation) => {
                let num_true = evaluation
                    .iter()
                    .fold(0u64, |num_true, &v| if v { num_true + 1 } else { num_true });
                
                let formula_evaluation = FormulaEvaluation {
                        solved: num_true == self.num_clauses,
                        num_true: num_true as usize,
                        num_false: (self.num_clauses - num_true) as usize
                    };

                return Some(formula_evaluation)
            },
            None => return None
        }
    }

    pub fn parse_from_dimacs_cnf(reader: impl Read) -> Result<Formula, FormulaParsingError> {
        let reader = BufReader::new(reader);
        let mut var_set: HashSet<u64> = HashSet::new();
        let mut clauses = Vec::new();
        let mut lines = reader.lines();
        
        // Lê o header
        let header = lines
            .next()
            .ok_or(FormulaParsingError::NoHeader)??
            .trim()
            .to_string();

        // cabeçalho "p cnf"
        let header_parts: Vec<_> = header.split_whitespace().collect();
        let header_values: Result<(u64, u64), _> = 
            if let ["p", "cnf", num_vars, num_clauses, ..] = header_parts.as_slice() {
                Ok((num_vars.parse()?, num_clauses.parse()?))
            } else {
                Err(FormulaParsingError::InvalidHeader)
            };
        let (num_vars, num_clauses) = header_values?;

        // parseia as cláusulas
        for line in lines {
            let line = line?;
            let trimmed_line = line.trim();

            // verifica se é o caracter terminador
            if trimmed_line.starts_with('%')
            { break }

            // ignora linhas vazias e comentários
            if trimmed_line.is_empty() || trimmed_line.starts_with('c')
            { continue }

            let mut lits: Vec<Literal> = Vec::new();
            for tok in trimmed_line.split_whitespace() {
                let val: i64 = tok.parse()?;

                if val == 0
                { break } // fim da cláusula

                if val > 0 { 
                    let variable = val as u64;
                    var_set.insert(variable);
                    lits.push(Literal::Var(val as u64));
                } else { 
                    let variable = (-val) as u64;
                    var_set.insert(variable);
                    lits.push(Literal::NegatedVar((-val) as u64));
                }
            }

            if lits.is_empty()
            { return Err(FormulaParsingError::EmptyClause) }

            clauses.push(Clause(lits));
        }

        if clauses.len() != (num_clauses as usize)
        { return Err(FormulaParsingError::InconsistentNumOfClauses) }

        if var_set.len() != (num_vars as usize)
        { return Err(FormulaParsingError::InconsistentNumOfVars) }

        if *var_set.iter().max().unwrap() != num_vars
        { return Err(FormulaParsingError::VarOutOfBounds) }

        Ok(
            Formula {
            num_vars,
            num_clauses,
            clauses,
        })
    }

    pub fn get_num_vars(&self) -> u64 {
        self.num_vars
    }

    pub fn get_num_clauses(&self) -> u64 {
        self.num_clauses
    }

    pub fn get_clauses(&self) -> &[Clause] {
        &self.clauses
    }
}

pub struct SATObjective { pub formula: Formula }
impl Objective<BinaryEncoding> for SATObjective {
    type Output = Option<Vec<usize>>;

    fn eval(&self, pop: &Population<BinaryEncoding>) -> Self::Output {
        let individuals = pop.get_individuals();

        let scores: Option<Vec<usize>> = individuals
            .iter()
            .map(|ind: &BinaryEncoding| ind.to_bool_slice())
            .map(|val| {
                let result = self.formula.evaluate(val);
                if let Some(evaluation) = result
                { Some(evaluation.num_false) }
                else 
                { None } 
            })
            .collect();

        scores
    }
}

#[cfg(test)]
mod sat_objective_tests {
    use std::io::Cursor;
    use Literal::*;
    use super::*;

    #[test]
    fn test_cnf_small_correct() {
        let dimacs_cnf =
            r#"p cnf 3 2
            1 -3 0
            2 3 0
            %"#;
        let expected_clauses = vec![
            Clause(vec![Var(1), NegatedVar(3)]),
            Clause(vec![Var(2), Var(3)])
        ];
        let dimacs_buffer = Cursor::new(dimacs_cnf);
        let formula = Formula::parse_from_dimacs_cnf(dimacs_buffer).unwrap();
        let actual_num_vars = formula.get_num_vars();
        let actual_num_clauses = formula.get_num_clauses();
        
        // Verificando propriedades
        assert_eq!(3, actual_num_vars);
        assert_eq!(2, actual_num_clauses);
        
        // Verificando validade das cláusulas
        for (expected_clause, actual_clause) in expected_clauses.iter().zip(formula.get_clauses().iter()) {
            assert_eq!(expected_clause, actual_clause);
        }

        // Verificando satisfazibilidade
        let expected_evaluation = FormulaEvaluation {
            solved: true,
            num_true: 2,
            num_false: 0
        };
        let valoration = [true, true, false];
        let evaluation = formula.evaluate(&valoration).unwrap();
        assert_eq!(evaluation, expected_evaluation);
    }

    #[test]
    fn test_cnf_invalid_header() {
        let dimacs_cnf = 
            r#"p 2 1
            1 -2 0
            %"#;
        let dimacs_buffer = Cursor::new(dimacs_cnf); 
        let result = Formula::parse_from_dimacs_cnf(dimacs_buffer);
        match result {
            Err(FormulaParsingError::InvalidHeader) => {},
            Err(err) => panic!("Expected FormulaParsingError::InvalidHeader, got {:?}", err),
            _ => panic!("Expected an error")
        }
    }

    #[test]
    fn test_cnf_empty_clause() {
        let dimacs_cnf = 
            r#"p cnf 3 2
            1 -3 0
            0
            %"#;
        let dimacs_buffer = Cursor::new(dimacs_cnf); 
        let result = Formula::parse_from_dimacs_cnf(dimacs_buffer);
        match result {
            Err(FormulaParsingError::EmptyClause) => {},
            Err(err) => panic!("Expected FormulaParsingError::EmptyClause, got {:?}", err),
            _ => panic!("Expected an error")
        }
    }

    #[test]
    fn test_cnf_inconsistent_number_of_variables() {
        let dimacs_cnf = 
            r#"p cnf 10 2
            1 -3 0
            1 2 0
            %"#;
        let dimacs_buffer = Cursor::new(dimacs_cnf); 
        let result = Formula::parse_from_dimacs_cnf(dimacs_buffer);
        match result {
            Err(FormulaParsingError::InconsistentNumOfVars) => {},
            Err(err) => panic!("Expected FormulaParsingError::InconsistentNumOfVars, got {:?}", err),
            _ => panic!("Expected an error")
        }
    }

    #[test]
    fn test_cnf_inconsistent_number_of_clauses() {
        let dimacs_cnf = 
            r#"p cnf 3 10
            1 -3 0
            1 2 0
            %"#;
        let dimacs_buffer = Cursor::new(dimacs_cnf); 
        let result = Formula::parse_from_dimacs_cnf(dimacs_buffer);
        match result {
            Err(FormulaParsingError::InconsistentNumOfClauses) => {},
            Err(err) => panic!("Expected FormulaParsingError::InconsistentNumOfClauses, got {:?}", err),
            _ => panic!("Expected an error")
        }
    }

    #[test]
    fn test_cnf_var_out_of_bounds() {
        let dimacs_cnf = 
            r#"p cnf 3 2
            1 -40 0
            1 2 0
            %"#;
        let dimacs_buffer = Cursor::new(dimacs_cnf); 
        let result = Formula::parse_from_dimacs_cnf(dimacs_buffer);
        match result {
            Err(FormulaParsingError::VarOutOfBounds) => {},
            Err(err) => panic!("Expected FormulaParsingError::VarOutOfBounds, got {:?}", err),
            _ => panic!("Expected an error")
        }
    }
}