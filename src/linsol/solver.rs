use linsol::inf_num::InfNum;
use linsol::function::Function;
use linsol::constraint::Sign;
use linsol::constraint::Consraint;

#[allow(dead_code)]
pub enum TargetValue {
    Min,
    Max,
}

#[allow(dead_code)]
pub struct Solver {
    targetFunction: Function,
    targetValue: TargetValue,
    constraints: Vec<Consraint>,
}

#[allow(dead_code)]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            targetFunction: Function::new(),
            targetValue: TargetValue::Min,
            constraints: Vec::<Consraint>::new()
        }
    }
}