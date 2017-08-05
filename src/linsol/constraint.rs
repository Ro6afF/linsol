use linsol::function::Function;
use linsol::inf_num::InfNum;
use std::collections::HashMap;

extern crate rand;

#[allow(dead_code)]
pub enum Sign {
    Equal,
    GreaterOrEqual,
    SmallerOrEqual,
}

#[allow(dead_code)]
pub fn get_random_name(len: usize) -> String {
    (0..len)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
        .collect()
}


#[allow(dead_code)]
pub struct Consraint {
    pub left: Function,
    pub sign: Sign,
    pub right: InfNum,
}

#[allow(dead_code)]
impl Consraint {
    pub fn new() -> Consraint {
        Consraint {
            left: Function::new(),
            sign: Sign::Equal,
            right: InfNum::new(),
        }
    }

    pub fn check(&self, variables: &HashMap<String, InfNum>) -> bool {
        match self.sign {
            Sign::GreaterOrEqual => self.left.get_value(variables) >= self.right,
            Sign::SmallerOrEqual => self.left.get_value(variables) <= self.right,
            Sign::Equal => self.left.get_value(variables) == self.right,
        }
    }

    pub fn do_cannonical_form(&mut self) -> Vec<Consraint> {
        match self.sign {
            Sign::GreaterOrEqual => {
                while self.left.add_variable(
                    get_random_name(10),
                    InfNum::from(-1.0, 0.0),
                )
                {}
            }
            Sign::SmallerOrEqual => {
                while self.left.add_variable(
                    get_random_name(10),
                    InfNum::from(1.0, 0.0),
                )
                {}
            }
            _ => {}
        }
        self.sign = Sign::Equal;
        if self.right < InfNum::from(0.0, 0.0) {
            self.right *= InfNum::from(-1.0, 1.0);
            self.left *= InfNum::from(-1.0, 1.0);
        }
        let mut res: Vec<Consraint> = Vec::<Consraint>::new();
        for i in &self.left.variables {
            let mut curr = Function::new();
            curr.add_variable(i.clone(), InfNum::from(1.0, 0.0));
            res.push(Consraint {
                right: InfNum::from(0.0, 0.0),
                sign: Sign::Equal,
                left: curr,
            });
        }
        res
    }
}
