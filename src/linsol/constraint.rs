use linsol::function::Function;
use linsol::inf_num::InfNum;
use std::collections::HashMap;
use linsol::utilities::get_random_name;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Sign {
    Equal,
    GreaterOrEqual,
    LessOrEqual,
}

impl PartialEq for Sign {
    fn eq(&self, other: &Sign) -> bool {
        let typ: i8;
        match self {
            &Sign::Equal => typ = 0,
            &Sign::GreaterOrEqual => typ = 1,
            &Sign::LessOrEqual => typ = 2,
        }
        let res: bool;
        match other {
            &Sign::Equal => res = typ == 0,
            &Sign::GreaterOrEqual => res = typ == 1,
            &Sign::LessOrEqual => res = typ == 2,
        }
        res
    }
}

impl Eq for Sign {}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
            Sign::LessOrEqual => self.left.get_value(variables) <= self.right,
            Sign::Equal => self.left.get_value(variables) == self.right,
        }
    }

    pub fn normalize(&mut self) -> Option<String> {
        if self.right < InfNum::from(0.0, 0.0) {
            self.right *= InfNum::from(-1.0, 0.0);
            self.left *= InfNum::from(-1.0, 0.0);
            match self.sign {
                Sign::GreaterOrEqual => self.sign = Sign::LessOrEqual,
                Sign::LessOrEqual => self.sign = Sign::GreaterOrEqual,
                _ => {}
            }
        }


        match self.sign {
            Sign::LessOrEqual => {
                self.left.add_variable(
                    get_random_name(10),
                    InfNum::from(1.0, 0.0),
                );
                self.sign = Sign::Equal;
                None
            }
            Sign::Equal => {
                let name = get_random_name(10);
                self.left.add_variable(name.clone(), InfNum::from(1.0, 0.0));
                self.sign = Sign::Equal;
                Some(name)
            }
            Sign::GreaterOrEqual => {
                let n1 = get_random_name(10);
                let mut n2 = get_random_name(10);
                while n1 == n2 {
                    n2 = get_random_name(10);
                }
                self.left.add_variable(n1, InfNum::from(-1.0, 0.0));
                self.left.add_variable(n2.clone(), InfNum::from(1.0, 0.0));
                self.sign = Sign::Equal;
                Some(n2)
            }
        }
    }
}
