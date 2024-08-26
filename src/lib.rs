use std::time::Duration;

use logic_form::{Clause, Lit, Var};

pub trait Satif {
    fn new_var(&mut self) -> Var;

    fn new_var_to(&mut self, var: Var) {
        while Var::new(self.num_var()) <= var {
            self.new_var();
        }
    }

    fn num_var(&self) -> usize;

    fn add_clause(&mut self, clause: &[Lit]);

    fn solve(&mut self, assumps: &[Lit]) -> bool;

    fn solve_with_limit(&mut self, _assumps: &[Lit], _limit: Duration) -> Option<bool> {
        panic!("unsupport solve with limit");
    }

    fn sat_value(&mut self, lit: Lit) -> Option<bool>;

    fn unsat_has(&mut self, _lit: Lit) -> bool {
        panic!("unsupport assumption");
    }

    fn simplify(&mut self) -> Option<bool> {
        panic!("unsupport simplify");
    }

    fn set_frozen(&mut self, _var: Var, _frozen: bool) {
        panic!("unsupport set frozen");
    }

    fn clauses(&self) -> Vec<Clause> {
        panic!("unsupport get clauses");
    }
}

pub trait Smtif {}
