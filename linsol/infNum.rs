use std::cmp;
use std::cmp::Ordering;
use std::ops;

pub struct InfNum {
    real: f64,
    inf: f64,
}

impl InfNum {
    pub fn new() -> InfNum {
        InfNum {
            real: 0.0,
            inf: 0.0,
        }
    }
}

impl PartialEq for InfNum {
    fn eq(&self, other: &InfNum) -> bool {
        if self.inf != 0.0 && other.inf != 0.0 {
            true
        } else if self.inf == other.inf {
            self.real.eq(&other.real)
        } else {
            false
        }
    }
}
impl Eq for InfNum {}

impl Ord for InfNum {
    fn cmp(&self, other: &InfNum) -> Ordering {
        if self.inf > other.inf {
            Ordering::Less
        } else if self.inf < other.inf {
            Ordering::Greater
        } else if self.real > other.real {
            Ordering::Less
        } else if self.real < other.real {
            Ordering::Greater
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
        InfNum {
            real: self.real + other.real,
            inf: self.inf + other.inf,
        }
    }
}

impl ops::Sub<InfNum> for InfNum {
    type Output = InfNum;
    fn sub(self, other: InfNum) -> InfNum {
        InfNum {
            real: self.real - other.real,
            inf: self.inf - other.inf,
        }
    }
}

impl ops::Mul<InfNum> for InfNum {
    type Output = InfNum;
    fn mul(self, other: InfNum) -> InfNum {
        InfNum {
            real: self.real * other.real,
            inf: self.inf * other.inf,
        }
    }
}

impl ops::Div<InfNum> for InfNum {
    type Output = InfNum;
    fn div(self, other: InfNum) -> InfNum {
        InfNum {
            real: self.real / other.real,
            inf: self.inf / other.inf,
        }
    }
}
