use logic_form::{Lit, Var};
use std::fmt::Debug;

pub trait Satif {
    type Model: SatifModel;
    type Conflict: SatifConflict;

    fn new() -> Self;

    fn new_var(&mut self) -> Var;

    fn num_var(&self) -> usize;

    fn add_clause(&mut self, clause: &[Lit]);

    fn solve(&mut self, assumps: &[Lit]) -> SatResult<Self::Model, Self::Conflict>;
}

pub trait SatifModel {
    fn lit_value(&self, lit: Lit) -> Option<bool>;
}

pub trait SatifConflict {
    fn has(&self, lit: Lit) -> bool;
}

pub enum SatResult<M, C>
where
    M: SatifModel,
    C: SatifConflict,
{
    Sat(M),
    Unsat(C),
}

impl<M: SatifModel, C: SatifConflict> Debug for SatResult<M, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(_) => "Sat".fmt(f),
            Self::Unsat(_) => "Unsat".fmt(f),
        }
    }
}
