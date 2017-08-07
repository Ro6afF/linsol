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

    pub fn from(left: Function, sign: Sign, right: InfNum) -> Consraint {
        Consraint {
            left: left,
            sign: sign,
            right: right,
        }
    }

    pub fn check(&self, variables: &HashMap<String, InfNum>) -> bool {
        match self.sign {
            Sign::GreaterOrEqual => self.left.get_value(variables) >= self.right,
            Sign::SmallerOrEqual => self.left.get_value(variables) <= self.right,
            Sign::Equal => self.left.get_value(variables) == self.right,
        }
    }
}
