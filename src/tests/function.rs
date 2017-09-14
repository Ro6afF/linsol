use linsol::inf_num::InfNum;
use linsol::function::Function;
use std::collections::HashMap;

#[test]
fn function_add_variable() {
    let mut inst = Function::new();
    assert!(inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0)) == true);
    assert!(inst.add_variable(String::from("x"), InfNum::from(2.3, -1.0)) == false);
}

#[test]
fn function_get_value0() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(2.3, -1.0));
    inst.add_variable(String::from("z"), InfNum::from(-12.0, 1.0));
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::new());
    vals.insert(String::from("y"), InfNum::from(1.0, 0.0));
    vals.insert(String::from("z"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(-9.7, -1.0));
}

#[test]
fn function_get_value1() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(2.0, -1.0));
    inst.add_variable(String::from("z"), InfNum::from(-12.0, 1.0));
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("y"), InfNum::from(1.0, 0.0));
    assert!(inst.get_value(&vals) == InfNum::from(2.0, -1.0));
}

#[test]
fn function_get_value2() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(2.3, -1.0));
    inst.add_variable(String::from("z"), InfNum::from(-12.0, 1.0));
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("z"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(-8.7, 0.0));
}

#[test]
fn function_get_value3() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(2.3, -1.0));
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("z"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(3.3, 1.0));
}

#[test]
fn function_change_coefficient() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(1.0, 0.0));
    inst.change_coefficient(String::from("y"), InfNum::from(2.3, -1.0));
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(3.3, 1.0));
}

#[test]
fn function_mulass() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(0.5, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(1.15, -1.0));
    inst *= InfNum::from(2.0, 1.0);
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("z"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(3.3, 1.0));
}

#[test]
fn function_divass() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(2.0, 0.0));
    inst.add_variable(String::from("y"), InfNum::from(4.6, -1.0));
    inst /= InfNum::from(2.0, 0.0);
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
    vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
    assert!(inst.get_value(&vals) == InfNum::from(3.3, 0.5));
}

#[test]
fn function_get_coefficient() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(-12.34, 56.78));
    assert!(inst.get_coefficient(String::from("x")) == InfNum::from(-12.34, 56.78));
}

#[test]
fn function_get_coefficient1() {
    let mut inst = Function::new();
    inst.add_variable(String::from("x"), InfNum::from(-12.34, 56.78));
    assert!(inst.get_coefficient(String::from("y")) == InfNum::from(0.0, 0.0));
}