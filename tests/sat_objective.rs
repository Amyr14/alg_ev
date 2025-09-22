use std::io::Cursor;
use gen_alg::alg::{objectives::*, Objective};
use gen_alg::population::*;

#[test]
fn test_sat_objective_small_pop() {
    let dimacs_cnf = 
        r#"p cnf 3 3
        1 -3 0
        2 3 0
        1 2 0
        %"#;

    let dimacs_buffer = Cursor::new(dimacs_cnf);
    let formula = Formula::parse_from_dimacs_cnf(dimacs_buffer).unwrap();
    let objective = SATObjective { formula };

    let individuals: Vec<BinaryEncoding> = [
        vec![true, true, false],
        vec![true, false, false],
        vec![false, false, false],
        vec![true, true, true]
    ].into_iter()
        .map(|val| BinaryEncoding(val))
        .collect();

    let population = Population(individuals);
    let scores = objective.eval(&population).unwrap();
    assert_eq!(scores, vec![0, 1, 2, 0]);
}