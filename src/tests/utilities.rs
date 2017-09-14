use linsol::utilities::get_random_name;

#[test]
fn random_name() {
    let inst = get_random_name(12);
    println!("{}", inst);
    assert!(inst.len() == 12);
}

#[test]
fn random_name1() {
    let inst = get_random_name(123);
    println!("{}", inst);
    assert!(inst.len() == 123);
}