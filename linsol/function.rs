use linsol::inf_num::InfNum;
use std::collections::HashMap;

pub struct Function {
    coeficients: Vec<InfNum>,
    variables: Vec<String>,
}

impl Function {
    pub fn new() -> Function {
        Function {
            coeficients: Vec::<InfNum>::new(),
            variables: Vec::<String>::new(),
        }
    }

    pub fn add_variable(&mut self, name: String, coeficient: InfNum) -> bool {
        if self.variables.contains(&name) {
            return false;
        }
        self.coeficients.push(coeficient);
        self.variables.push(name);
        true
    }

    pub fn get_value(self, values: HashMap<String, f64>) -> InfNum {
        //TODO: DO THIS
        InfNum::new()
    }
}