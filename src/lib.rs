use logic_form::{Lit, Var};
use std::fmt::Debug;

pub trait Satif {
    fn new() -> Self;

    fn new_var(&mut self) -> Var;

    fn new_var_to(&mut self, var: Var) {
        while Var::new(self.num_var()) <= var {
            self.new_var();
        }
    }

    fn num_var(&self) -> usize;

    fn add_clause(&mut self, clause: &[Lit]);

    fn solve(&mut self, assumps: &[Lit]) -> bool;

    fn sat_value(&mut self, lit: Lit) -> Option<bool>;

    fn unsat_has(&mut self, lit: Lit) -> bool;

    fn simplify(&mut self);
}
