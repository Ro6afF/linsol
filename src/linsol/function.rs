use linsol::inf_num::InfNum;
use std::collections::HashMap;
use std::ops;

#[allow(dead_code)]
pub struct Function {
    coeficients: Vec<InfNum>,
    variables: Vec<String>,
}

impl Function {
    #[allow(dead_code)]
    pub fn new() -> Function {
        Function {
            coeficients: Vec::<InfNum>::new(),
            variables: Vec::<String>::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_variable(&mut self, name: String, coeficient: InfNum) -> bool {
        if self.variables.contains(&name) {
            return false;
        }
        self.coeficients.push(coeficient);
        self.variables.push(name);
        true
    }

    #[allow(dead_code)]
    pub fn change_coeficient(&mut self, name: String, coeficient: InfNum) {
        if self.variables.contains(&name) {
            let index = self.variables
                .iter()
                .enumerate()
                .find(|&r| *r.1 == name)
                .unwrap()
                .0;
            self.variables[index] = name;
            self.coeficients[index] = coeficient;

        } else {
            self.variables.push(name);
            self.coeficients.push(coeficient);
        }
    }

    #[allow(dead_code)]
    pub fn get_value(&self, values: &HashMap<String, InfNum>) -> InfNum {
        let mut result: InfNum = InfNum::new();
        for i in 0..self.variables.len() {
            if values.contains_key(&self.variables[i]) {
                result += *&self.coeficients[i] * *&values[&self.variables[i]];
            }
        }
        result
    }
}

impl ops::DivAssign<InfNum> for Function {
    fn div_assign(&mut self, num: InfNum) {
        for i in 0..self.coeficients.len() {
            self.coeficients[i] /= num;
        }
    }
}

impl ops::MulAssign<InfNum> for Function {
    fn mul_assign(&mut self, num: InfNum) {
        for i in 0..self.coeficients.len() {
            self.coeficients[i] *= num;
        }
    }
}