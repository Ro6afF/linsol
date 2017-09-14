use linsol::inf_num::*;
use linsol::constraint::*;
use std::collections::HashMap;

#[test]
fn constraint_check() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 1.0),
    );
    inst.sign = Sign::GreaterOrEqual;
    inst.right = InfNum::from(-1.0, 0.0);
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(0.0, 0.0));
    assert!(inst.check(&vals));
}

#[test]
fn constraint_check1() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 1.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(1.0, 0.5),
    );
    inst.sign = Sign::GreaterOrEqual;
    inst.right = InfNum::from(-1.0, 0.0);
    let mut vals = HashMap::<String, InfNum>::new();
    vals.insert(String::from("x"), InfNum::from(0.0, 0.0));
    vals.insert(String::from("y"), InfNum::from(0.0, 1.0));
    assert!(inst.check(&vals));
}

#[test]
fn constraint_normalize() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::Equal;
    inst.right = InfNum::from(1.0, 0.0);
    match inst.normalize() {
        None => panic!("No artificial"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(-1.0, 0.0));
}

#[test]
fn constraint_normalize1() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::Equal;
    inst.right = InfNum::from(-1.0, 0.0);
    match inst.normalize() {
        None => panic!("No artificial"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(-1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(1.0, 0.0));
}

#[test]
fn constraint_normalize2() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::GreaterOrEqual;
    inst.right = InfNum::from(1.0, 0.0);
    match inst.normalize() {
        None => panic!("No artificial"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(-1.0, 0.0));
    assert!(inst.left.coefficients[2] == InfNum::from(-1.0, 0.0));
    assert!(inst.left.coefficients[3] == InfNum::from(1.0, 0.0));
}

#[test]
fn constraint_normalize3() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::LessOrEqual;
    inst.right = InfNum::from(-1.0, 0.0);
    match inst.normalize() {
        None => panic!("No artificial"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(-1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(1.0, 0.0));
    assert!(inst.left.coefficients[2] == InfNum::from(-1.0, 0.0));
    assert!(inst.left.coefficients[3] == InfNum::from(1.0, 0.0));
}

#[test]
fn constraint_normalize4() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::LessOrEqual;
    inst.right = InfNum::from(1.0, 0.0);
    match inst.normalize() {
        Some(_) => panic!("Artificial appeared"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(-1.0, 0.0));
    assert!(inst.left.coefficients[2] == InfNum::from(1.0, 0.0));
}

#[test]
fn constraint_normalize5() {
    let mut inst = Consraint::new();
    inst.left.add_variable(
        String::from("x"),
        InfNum::from(1.0, 0.0),
    );
    inst.left.add_variable(
        String::from("y"),
        InfNum::from(-1.0, 0.0),
    );
    inst.sign = Sign::GreaterOrEqual;
    inst.right = InfNum::from(-1.0, 0.0);
    match inst.normalize() {
        Some(_) => panic!("Artificial appeared"),
        _ => {}
    }
    assert!(inst.right == InfNum::from(1.0, 0.0));
    assert!(inst.sign == Sign::Equal);
    assert!(inst.left.get_coefficient(String::from("x")) == InfNum::from(-1.0, 0.0));
    assert!(inst.left.get_coefficient(String::from("y")) == InfNum::from(1.0, 0.0));
    assert!(inst.left.coefficients[2] == InfNum::from(1.0, 0.0));
}