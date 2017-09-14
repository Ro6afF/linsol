use linsol::function::Function;
use linsol::inf_num::InfNum;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum TargetValue {
    Min,
    Max,
}

impl PartialEq for TargetValue {
    fn eq(&self, other: &TargetValue) -> bool {
        let is_min: bool;
        match self {
            &TargetValue::Min => is_min = true,
            &TargetValue::Max => is_min = false,
        }
        match other {
            &TargetValue::Min => is_min,
            &TargetValue::Max => !is_min,
        }
    }
}

impl Eq for TargetValue {}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TargetFunction {
    pub function: Function,
    pub target: TargetValue,
}

impl TargetFunction {
    pub fn new() -> TargetFunction {
        TargetFunction {
            function: Function::new(),
            target: TargetValue::Max,
        }
    }

    pub fn from(func: &Function, target: TargetValue) -> TargetFunction {
        TargetFunction {
            function: func.clone(),
            target: target,
        }
    }

    pub fn change_type(&mut self, new_target: TargetValue) {
        if new_target != self.target {
            self.function *= InfNum::from(-1.0, 0.0);
            self.target = new_target;
        }
    }
}