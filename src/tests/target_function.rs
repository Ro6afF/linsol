use linsol::inf_num::InfNum;
use linsol::function::Function;
use linsol::target_function::TargetFunction;
use linsol::target_function::TargetValue;

#[test]
fn target_function_change_type() {
    let mut func = Function::new();
    func.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    func.add_variable(String::from("y"), InfNum::from(-2.0, 0.0));
    func.add_variable(String::from("z"), InfNum::from(0.0, 0.0));
    let mut inst = TargetFunction::from(&func, TargetValue::Min);
    inst.change_type(TargetValue::Min);
    assert!(inst.function.get_coefficient(String::from("x")) == InfNum::from(1.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("y")) == InfNum::from(-2.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("z")) == InfNum::from(0.0, 0.0));
    assert!(inst.target == TargetValue::Min);
}

#[test]
fn target_function_change_type1() {
    let mut func = Function::new();
    func.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    func.add_variable(String::from("y"), InfNum::from(-2.0, 0.0));
    func.add_variable(String::from("z"), InfNum::from(0.0, 0.0));
    let mut inst = TargetFunction::from(&func, TargetValue::Min);
    inst.change_type(TargetValue::Max);
    assert!(inst.function.get_coefficient(String::from("x")) == InfNum::from(-1.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("y")) == InfNum::from(2.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("z")) == InfNum::from(0.0, 0.0));
    assert!(inst.target == TargetValue::Max);
}

#[test]
fn target_function_change_type2() {
    let mut func = Function::new();
    func.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    func.add_variable(String::from("y"), InfNum::from(-2.0, 0.0));
    func.add_variable(String::from("z"), InfNum::from(0.0, 0.0));
    let mut inst = TargetFunction::from(&func, TargetValue::Max);
    inst.change_type(TargetValue::Min);
    assert!(inst.function.get_coefficient(String::from("x")) == InfNum::from(-1.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("y")) == InfNum::from(2.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("z")) == InfNum::from(0.0, 0.0));
    assert!(inst.target == TargetValue::Min);
}

#[test]
fn target_function_change_type3() {
    let mut func = Function::new();
    func.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    func.add_variable(String::from("y"), InfNum::from(-2.0, 0.0));
    func.add_variable(String::from("z"), InfNum::from(0.0, 0.0));
    let mut inst = TargetFunction::from(&func, TargetValue::Max);
    inst.change_type(TargetValue::Max);
    assert!(inst.function.get_coefficient(String::from("x")) == InfNum::from(1.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("y")) == InfNum::from(-2.0, 0.0));
    assert!(inst.function.get_coefficient(String::from("z")) == InfNum::from(0.0, 0.0));
    assert!(inst.target == TargetValue::Max);
}