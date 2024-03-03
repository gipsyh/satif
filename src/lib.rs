use logic_form::{Lit, Var};
use std::fmt::Debug;

pub trait Satif {
    type Sat: SatifSat;
    type Unsat: SatifUnsat;

    fn new() -> Self;

    fn new_var(&mut self) -> Var;

    fn num_var(&self) -> usize;

    fn add_clause(&mut self, clause: &[Lit]);

    fn solve(&mut self, assumps: &[Lit]) -> SatResult<Self::Sat, Self::Unsat>;
}

pub trait SatifSat {
    fn lit_value(&self, lit: Lit) -> Option<bool>;
}

pub trait SatifUnsat {
    fn has(&self, lit: Lit) -> bool;
}

pub enum SatResult<S, U>
where
    S: SatifSat,
    U: SatifUnsat,
{
    Sat(S),
    Unsat(U),
}

impl<S: SatifSat, U: SatifUnsat> Debug for SatResult<S, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sat(_) => "Sat".fmt(f),
            Self::Unsat(_) => "Unsat".fmt(f),
        }
    }
}
