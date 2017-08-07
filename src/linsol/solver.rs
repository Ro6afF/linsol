use linsol::inf_num::InfNum;
use linsol::function::Function;
use linsol::constraint::Sign;
use linsol::constraint::Consraint;
use linsol::constraint::get_random_name;
//use std::collections::HashMap;

#[allow(dead_code)]
pub enum TargetValue {
    Min,
    Max,
}

#[allow(dead_code)]
pub struct Solver {
    pub target_function: Function,
    pub target_value: TargetValue,
    pub constraints: Vec<Consraint>,
}

#[allow(dead_code)]
enum TableCell {
    Name(String),
    Value(InfNum),
}

#[allow(dead_code)]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            target_function: Function::new(),
            target_value: TargetValue::Min,
            constraints: Vec::<Consraint>::new(),
        }
    }

    pub fn from(function: Function, value: TargetValue, constraints: Vec<Consraint>) -> Solver {
        Solver {
            target_function: function,
            target_value: value,
            constraints: constraints,
        }
    }

    fn canonical_form(&mut self) {
        for i in 0..self.constraints.len() {
            let curr = &mut self.constraints[i];
            match curr.sign {
                Sign::GreaterOrEqual => {
                    while curr.left.add_variable(
                        get_random_name(10),
                        InfNum::from(-1.0, 0.0),
                    )
                    {}
                }
                Sign::SmallerOrEqual => {
                    while curr.left.add_variable(
                        get_random_name(10),
                        InfNum::from(1.0, 0.0),
                    )
                    {}
                }
                _ => {}
            }
            curr.sign = Sign::Equal;
            if curr.right < InfNum::from(0.0, 0.0) {
                curr.right *= InfNum::from(-1.0, 1.0);
                curr.left *= InfNum::from(-1.0, 1.0);
            }
        }
    }

    fn base_form(&mut self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        for i in &mut self.constraints {
            let mut curr = get_random_name(10);
            while i.left.add_variable(curr.clone(), InfNum::from(1.0, 0.0)) {
                curr = get_random_name(10);
            }
            res.push(curr.clone());
            self.target_function.add_variable(
                curr,
                InfNum::from(0.0, 1.0),
            );
        }
        res
    }

    fn get_simplex_table(&mut self) -> Vec<Vec<TableCell>> {
        let bases = self.base_form();
        let mut res = Vec::<Vec<TableCell>>::new();
        for i in 0..bases.len() + 3 {
            res.push(Vec::<TableCell>::new());
            for _ in 0..self.target_function.variables.len() + 3 {
                res[i].push(TableCell::Name(String::from("UNDEFINED")));
            }
        }
        res[0][0] = TableCell::Name(String::from("MIN"));
        res[0][1] = TableCell::Name(String::from("MIN"));
        res[0][2] = TableCell::Value(InfNum::from(0.0, 0.0));
        res[1][0] = TableCell::Name(String::from("Cb"));
        res[1][1] = TableCell::Name(String::from("Vb"));
        res[1][2] = TableCell::Name(String::from("b"));
        for i in 0..self.target_function.variables.len() {
            res[0][i + 3] = TableCell::Value(self.target_function.coeficients[i]);
            res[1][i + 3] = TableCell::Name(self.target_function.variables[i].clone());
        }
        for i in 0..bases.len() {
            res[i + 3][0] = TableCell::Value(InfNum::from(0.0, 1.0));
            res[i + 3][1] = TableCell::Name(bases[i].clone());
            res[i + 3][2] = TableCell::Value(self.constraints[i].right);
        }

        for i in 0..bases.len() {
            for j in 0..self.target_function.variables.len() {
                res[i + 3][j + 3] = TableCell::Value(self.constraints[i].left.get_coeficient(
                    self.target_function.variables[i].clone(),
                ));
            }
        }

        res
    }
}
