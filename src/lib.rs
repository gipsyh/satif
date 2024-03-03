use logic_form::{Lit, Var};

pub trait Solver {
    fn new() -> Self;

    fn new_var(&mut self) -> Var;

    fn num_var(&self) -> usize;

    fn add_clause(&mut self, clause: &[Lit]);

    fn solve<M: Model, C: Conflict>(&mut self, assumps: &[Lit]) -> SatResult<M, C>;
}

pub trait Model {
    fn lit_value(&self, lit: Lit) -> Option<bool>;
}

pub trait Conflict {
    fn has(&self, lit: Lit) -> bool;
}

pub enum SatResult<M, C>
where
    M: Model,
    C: Conflict,
{
    Sat(M),
    Unsat(C),
}
