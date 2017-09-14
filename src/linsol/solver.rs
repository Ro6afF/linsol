use linsol::inf_num::InfNum;
use linsol::target_function::TargetFunction;
use linsol::target_function::TargetValue;
use linsol::constraint::Consraint;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Solver {
    pub target_function: TargetFunction,
    pub constraints: Vec<Consraint>,
    pub artificials: bool,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TableCell {
    Name(String),
    Value(InfNum),
}

impl PartialEq for TableCell {
    fn eq(&self, other: &TableCell) -> bool {
        match self {
            &TableCell::Name(ref x) => {
                match other {
                    &TableCell::Name(ref y) => return x == y,
                    _ => {}
                }
            }
            &TableCell::Value(ref x) => {
                match other {
                    &TableCell::Value(ref y) => return x == y,
                    _ => {}
                }
            }
        }
        false
    }
}

impl Eq for TableCell {}

#[allow(dead_code)]
type SimplexTable = Vec<Vec<TableCell>>;

#[allow(dead_code)]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            target_function: TargetFunction::new(),
            constraints: Vec::<Consraint>::new(),
            artificials: false,
        }
    }

    pub fn from(function: TargetFunction, constraints: Vec<Consraint>) -> Solver {
        Solver {
            target_function: function,
            artificials: false,
            constraints: constraints,
        }
    }

    pub fn normalize(&mut self) {
        self.target_function.change_type(TargetValue::Max);
        for i in 0..self.constraints.len() {
            match self.constraints[i].normalize() {
                Some(_) => self.artificials = true,
                _ => {}
            }
        }
    }
}
