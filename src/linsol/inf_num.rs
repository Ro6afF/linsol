use std::cmp::Ordering;
use std::ops;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct InfNum {
    pub real: f64,
    pub inf: f64,
}

#[allow(dead_code)]
impl InfNum {
    pub fn new() -> InfNum {
        InfNum {
            real: 0.0,
            inf: 0.0,
        }
    }
    pub fn from(real: f64, inf: f64) -> InfNum {
        InfNum {
            real: real,
            inf: inf,
        }
    }
}

impl PartialEq for InfNum {
    fn eq(&self, other: &InfNum) -> bool {
        if (self.inf > 0.0 && other.inf > 0.0) || (self.inf < 0.0 && other.inf < 0.0) {
            true
        } else if self.inf == 0.0 && other.inf == 0.0 {
            self.real == other.real
        } else {
            false
        }
    }
}

impl Eq for InfNum {}

impl Ord for InfNum {
    fn cmp(&self, other: &InfNum) -> Ordering {
        if (self.inf > 0.0 && other.inf > 0.0) || (self.inf < 0.0 && other.inf < 0.0) {
            Ordering::Equal
        } else if self.inf < other.inf {
            Ordering::Less
        } else if self.inf > other.inf {
            Ordering::Greater
        } else if self.real > other.real {
            Ordering::Greater
        } else if self.real < other.real {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for InfNum {
    fn partial_cmp(&self, other: &InfNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Add<InfNum> for InfNum {
    type Output = InfNum;
    fn add(self, other: InfNum) -> InfNum {
        let mut res = InfNum {
            real: self.real + other.real,
            inf: self.inf + other.inf,
        };
        if res.inf > 0.0 {
            res.inf = 1.0;
        } else if res.inf < 0.0 {
            res.inf = -1.0;
        }
        res
    }
}

impl ops::AddAssign<InfNum> for InfNum {
    fn add_assign(&mut self, other: InfNum) {
        *self = InfNum {
            inf: self.inf + other.inf,
            real: self.real + other.real,
        };
        if self.inf > 0.0 {
            self.inf = 1.0;
        } else if self.inf < 0.0 {
            self.inf = -1.0;
        }
    }
}


impl ops::Sub<InfNum> for InfNum {
    type Output = InfNum;
    fn sub(self, other: InfNum) -> InfNum {
        let mut res = InfNum {
            real: self.real - other.real,
            inf: self.inf - other.inf,
        };
        if res.inf > 0.0 {
            res.inf = 1.0;
        } else if res.inf < 0.0 {
            res.inf = -1.0;
        }
        res
    }
}

impl ops::SubAssign<InfNum> for InfNum {
    fn sub_assign(&mut self, other: InfNum) {
        *self = InfNum {
            inf: self.inf - other.inf,
            real: self.real - other.real,
        };
        if self.inf > 0.0 {
            self.inf = 1.0;
        } else if self.inf < 0.0 {
            self.inf = -1.0;
        }
    }
}

impl ops::Mul<InfNum> for InfNum {
    type Output = InfNum;
    fn mul(self, other: InfNum) -> InfNum {
        let mut res = InfNum {
            real: self.real * other.real,
            inf: self.inf * other.inf + self.real * other.inf + self.inf * other.real,
        };
        if res.inf > 0.0 {
            res.inf = 1.0;
        } else if res.inf < 0.0 {
            res.inf = -1.0;
        }
        res
    }
}

impl ops::MulAssign<InfNum> for InfNum {
    fn mul_assign(&mut self, other: InfNum) {
        *self = InfNum {
            real: self.real * other.real,
            inf: self.inf * other.inf + self.real * other.inf + self.inf * other.real,
        };
        if self.inf > 0.0 {
            self.inf = 1.0;
        } else if self.inf < 0.0 {
            self.inf = -1.0;
        }
    }
}

impl ops::Div<InfNum> for InfNum {
    type Output = InfNum;
    fn div(self, other: InfNum) -> InfNum {
        if other.inf.abs() < 0.0001 {
            let mut res = InfNum {
                real: self.real / other.real,
                inf: self.inf / other.real,
            };
            if res.inf > 0.0 {
                res.inf = 1.0;
            } else if res.inf < 0.0 {
                res.inf = -1.0;
            }
            res
        } else {
            InfNum {
                real: self.inf / other.inf,
                inf: 0.0,
            }
        }
    }
}

impl ops::DivAssign<InfNum> for InfNum {
    fn div_assign(&mut self, other: InfNum) {
        *self = if other.inf.abs() < 0.0001 {
            InfNum {
                real: self.real / other.real,
                inf: self.inf / other.real,
            }
        } else {
            InfNum {
                real: self.inf / other.inf,
                inf: 0.0,
            }
        };
        if self.inf > 0.0 {
            self.inf = 1.0;
        } else if self.inf < 0.0 {
            self.inf = -1.0;
        }
    }
}
