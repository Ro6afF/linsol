mod linsol;
use linsol::inf_num::InfNum;
//use linsol::function::Function;

#[test]
fn inf_num_creation() {
    let inst: InfNum = InfNum::new();
    assert!((inst.inf == 0.0) && (inst.real == 0.0));
}

#[test]
fn inf_num_eq1() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 1.0;
    inst1.real = 2.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 1.0;
    inst2.real = 2.0;
    assert!(inst1 == inst2);
}

#[test]
fn inf_num_eq2() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 112.0;
    inst1.real = 2.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 1.0;
    inst2.real = 2.0;
    assert!(inst1 == inst2);
}

#[test]
fn inf_num_eq3() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 1.0;
    inst1.real = 2.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 1212.0;
    inst2.real = 2.0;
    assert!(inst1 == inst2);
}

#[test]
fn inf_num_eq4() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 0.0;
    inst1.real = 2.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 0.0;
    inst2.real = 2.0;
    assert!(inst1 == inst2);
}

#[test]
fn inf_num_ieq1() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 1.0;
    inst1.real = 3.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 0.0;
    inst2.real = 2.0;
    assert!(inst1 != inst2);
}

#[test]
fn inf_num_ieq2() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 0.0;
    inst1.real = 3.0;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 1.0;
    inst2.real = 2.0;
    assert!(inst1 != inst2);
}

#[test]
fn inf_num_add() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 3.1;
    inst1.real = 1.9;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 2.2;
    inst2.real = 2.2;
    let inst = inst1 + inst2;
    assert!(
        inst ==
            InfNum {
                inf: 5.3,
                real: 4.1,
            }
    );
}

#[test]
fn inf_num_sub() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 3.1;
    inst1.real = 1.9;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = 2.2;
    inst2.real = 2.2;
    let inst = inst1 - inst2;
    assert!(
        inst ==
            InfNum {
                inf: 0.9,
                real: -0.3,
            }
    );
}

#[test]
fn inf_num_mul() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 3.1;
    inst1.real = 1.9;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = -2.2;
    inst2.real = 2.2;
    let inst = inst1 * inst2;
    assert!(
        inst ==
            InfNum {
                inf: -6.28,
                real: 4.18,
            }
    );
}

#[test]
fn inf_num_div() {
    let mut inst1: InfNum = InfNum::new();
    inst1.inf = 3.1;
    inst1.real = 1.9;
    let mut inst2: InfNum = InfNum::new();
    inst2.inf = -10.0;
    inst2.real = 0.1;
    let inst = inst1 * inst2;
    assert!(
        inst ==
            InfNum {
                inf: -0.31,
                real: 19.0,
            }
    );
}