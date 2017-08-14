use linsol::inf_num::InfNum;
use std::collections::HashMap;
use std::ops;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Function {
    pub coefficients: Vec<InfNum>,
    pub variables: Vec<String>,
}
#[allow(dead_code)]
impl Function {
    pub fn new() -> Function {
        Function {
            coefficients: Vec::<InfNum>::new(),
            variables: Vec::<String>::new(),
        }
    }


    pub fn add_variable(&mut self, name: String, coefficient: InfNum) -> bool {
        if self.variables.contains(&name) {
            return false;
        }
        self.coefficients.push(coefficient);
        self.variables.push(name);
        true
    }

    pub fn change_coefficient(&mut self, name: String, coefficient: InfNum) {
        if self.variables.contains(&name) {
            let index = self.variables
                .iter()
                .enumerate()
                .find(|&r| *r.1 == name)
                .unwrap()
                .0;
            self.variables[index] = name;
            self.coefficients[index] = coefficient;

        } else {
            self.variables.push(name);
            self.coefficients.push(coefficient);
        }
    }

    pub fn get_coefficient(&self, variable: String) -> InfNum {
        if self.variables.contains(&variable) {
            let index = self.variables
                .iter()
                .enumerate()
                .find(|&r| *r.1 == variable)
                .unwrap()
                .0;
            self.coefficients[index]
        } else {
            InfNum::from(0.0, 0.0)
        }
    }

    pub fn get_value(&self, values: &HashMap<String, InfNum>) -> InfNum {
        let mut result: InfNum = InfNum::new();
        for i in 0..self.variables.len() {
            if values.contains_key(&self.variables[i]) {
                result += *&self.coefficients[i] * *&values[&self.variables[i]];
            }
        }
        result
    }
}
impl ops::DivAssign<InfNum> for Function {
    fn div_assign(&mut self, num: InfNum) {
        for i in 0..self.coefficients.len() {
            self.coefficients[i] /= num;
        }
    }
}

impl ops::MulAssign<InfNum> for Function {
    fn mul_assign(&mut self, num: InfNum) {
        for i in 0..self.coefficients.len() {
            self.coefficients[i] *= num;
        }
    }
}
