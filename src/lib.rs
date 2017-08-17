pub mod linsol;

#[cfg(test)]
mod tests {
    use linsol::inf_num::InfNum;
    use linsol::function::Function;
    use linsol::constraint::Consraint;
    use linsol::constraint::Sign;
    use linsol::utilities::get_random_name;
    use linsol::solver::Solver;
    use linsol::solver::TableCell;
    use linsol::solver::TargetValue;
    use std::collections::HashMap;

    #[test]
    fn inf_num_comp() {
        let inst1 = InfNum::from(0.0, 2.0);
        let inst2 = InfNum::from(-1.0, 7.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_comp1() {
        let inst1 = InfNum::from(0.0, 0.0);
        let inst2 = InfNum::from(-1.0, 0.0);
        assert!(inst1 > inst2);
    }

    #[test]
    fn inf_num_comp2() {
        let inst1 = InfNum::from(0.0, 0.0);
        let inst2 = InfNum::from(-1.0, 1.0);
        assert!(inst1 < inst2);
    }

    #[test]
    fn inf_num_comp3() {
        let inst1 = InfNum::from(-1.0, 1.0);
        let inst2 = InfNum::from(-1.0, 1.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_comp4() {
        let inst1 = InfNum::from(0.0, -2.0);
        let inst2 = InfNum::from(-1.0, -7.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_comp5() {
        let inst1 = InfNum::from(0.0, 2.0);
        let inst2 = InfNum::from(-1.0, -7.0);
        assert!(inst1 > inst2);
    }

    #[test]
    fn inf_num_creation_new() {
        let inst: InfNum = InfNum::new();
        assert!((inst.inf == 0.0) && (inst.real == 0.0));
    }

    #[test]
    fn inf_num_creation_from() {
        let inst: InfNum = InfNum::from(5.0, -4.3);
        assert!((inst.inf == -4.3) && (inst.real == 5.0));
    }

    #[test]
    fn inf_num_eq1() {
        let inst1: InfNum = InfNum::from(2.0, 1.0);
        let inst2: InfNum = InfNum::from(2.0, 1.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_eq2() {
        let inst1: InfNum = InfNum::from(2.0, 112.0);
        let inst2: InfNum = InfNum::from(2.0, 1.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_eq3() {
        let inst1: InfNum = InfNum::from(2.0, 1.0);
        let inst2: InfNum = InfNum::from(2.0, 112.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_eq4() {
        let inst1: InfNum = InfNum::from(2.0, 0.0);
        let inst2: InfNum = InfNum::from(2.0, 0.0);
        assert!(inst1 == inst2);
    }

    #[test]
    fn inf_num_ieq1() {
        let inst1: InfNum = InfNum::from(3.0, 1.0);
        let inst2: InfNum = InfNum::from(2.0, 0.0);
        assert!(inst1 != inst2);
    }

    #[test]
    fn inf_num_ieq2() {
        let inst1: InfNum = InfNum::from(3.0, 0.0);
        let inst2: InfNum = InfNum::from(2.0, 1.0);
        assert!(inst1 != inst2);
    }

    #[test]
    fn inf_num_add() {
        let inst1: InfNum = InfNum::from(1.9, 3.1);
        let inst2: InfNum = InfNum::from(2.2, 2.2);
        let inst = inst1 + inst2;
        assert!(inst == InfNum::from(4.1, 5.3));
    }

    #[test]
    fn inf_num_sub() {
        let inst1: InfNum = InfNum::from(1.9, 3.1);
        let inst2: InfNum = InfNum::from(2.2, 2.2);
        let inst = inst1 - inst2;
        assert!(inst == InfNum::from(-0.3, 0.9));
    }

    #[test]
    fn inf_num_mul() {
        let inst1: InfNum = InfNum::from(1.9, 3.1);
        let inst2: InfNum = InfNum::from(2.2, 2.2);
        let inst = inst1 * inst2;
        assert!(inst == InfNum::from(4.18, 6.28));
    }

    #[test]
    fn inf_num_div() {
        let inst1: InfNum = InfNum::from(1.9, 3.1);
        let inst2: InfNum = InfNum::from(0.1, -10.0);
        let inst = inst1 / inst2;
        println!("{:?}", inst);
        assert!(inst == InfNum::from(-0.31, 0.0));
    }

    #[test]
    fn inf_num_addass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 += InfNum::from(-3.0, 2.0);
        inst1 += InfNum::from(1.0, -1.0);
        assert!(inst1 == InfNum::from(0.0, 0.0));
    }

    #[test]
    fn inf_num_subass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 -= InfNum::from(-3.0, 2.0);
        assert!(inst1 == InfNum::from(4.0, -2.0));
    }

    #[test]
    fn inf_num_mulass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 *= InfNum::from(-3.0, 2.0);
        assert!(inst1 == InfNum::from(-6.0, 2.0));
    }

    #[test]
    fn inf_num_divass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 /= InfNum::from(-2.0, 0.0);
        println!("{:?}", inst1);
        assert!(inst1 == InfNum::from(-1.0, 0.5));
    }

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
    fn solver_cannonical_form() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(5.5, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-1.5, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-5.0, 0.0);
        inst.constraints[len].sign = Sign::Equal;
        inst.canonical_form();
        let mut res = Solver::new();
        res.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        res.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        res.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        res.target_value = TargetValue::Min;
        res.constraints.push(Consraint::new());
        let mut len = res.constraints.len() - 1;
        res.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        res.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        res.constraints[len].left.add_variable(
            inst.constraints[len].left.variables[inst.constraints[len].left.variables.len() -
                                                     1]
                .clone(),
            InfNum::from(1.0, 0.0),
        );
        res.constraints[len].right = InfNum::from(5.5, 0.0);
        res.constraints[len].sign = Sign::Equal;
        res.constraints.push(Consraint::new());
        len = res.constraints.len() - 1;
        res.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(-1.0, 0.0),
        );
        res.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(-1.0, 0.0),
        );
        res.constraints[len].left.add_variable(
            inst.constraints[len].left.variables[inst.constraints[len].left.variables.len() -
                                                     1]
                .clone(),
            InfNum::from(1.0, 0.0),
        );
        res.constraints[len].right = InfNum::from(1.5, 0.0);
        res.constraints[len].sign = Sign::Equal;
        res.constraints.push(Consraint::new());
        len = res.constraints.len() - 1;
        res.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(-1.0, 0.0),
        );
        res.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(-1.0, 0.0),
        );
        res.constraints[len].right = InfNum::from(5.0, 0.0);
        res.constraints[len].sign = Sign::Equal;
        for i in 0..res.constraints.len() {
            for j in 0..res.constraints[i].left.variables.len() {
                assert!(
                    res.constraints[i].left.variables[j] == inst.constraints[i].left.variables[j]
                );
                assert!(
                    res.constraints[i].left.coefficients[j] == inst.constraints[i].left.coefficients[j]
                );
            }
        }
    }

    #[test]
    fn solver_base_form() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(5.5, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-1.5, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-5.0, 0.0);
        inst.constraints[len].sign = Sign::Equal;
        inst.canonical_form();
        let out = inst.base_form();
        println!("{:?}", inst);
        assert!(out.len() == 3);
        assert!(inst.constraints[0].left.coefficients[3] == InfNum::from(1.0, 0.0));
        assert!(inst.constraints[1].left.coefficients[3] == InfNum::from(1.0, 0.0));
        assert!(inst.constraints[2].left.coefficients[2] == InfNum::from(1.0, 0.0));
        assert!(inst.target_function.coefficients[5] == InfNum::from(0.0, 1.0));
        assert!(inst.target_function.coefficients[6] == InfNum::from(0.0, 1.0));
        assert!(inst.target_function.coefficients[7] == InfNum::from(0.0, 1.0));
    }

    #[test]
    fn solver_simplex_table() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(5.5, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-1.5, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-5.0, 0.0);
        inst.constraints[len].sign = Sign::Equal;
        let tableinst = inst.get_simplex_table();
        let exp = vec![
            InfNum {
                real: 0.0,
                inf: 12.0,
            },
            InfNum {
                real: -1.0,
                inf: 0.0,
            },
            InfNum {
                real: -2.0,
                inf: 0.0,
            },
            InfNum {
                real: 3.0,
                inf: -2.0,
            },
            InfNum {
                real: 0.0,
                inf: 1.0,
            },
            InfNum {
                real: 0.0,
                inf: 1.0,
            },
            InfNum {
                real: 0.0,
                inf: 0.0,
            },
            InfNum {
                real: 0.0,
                inf: 0.0,
            },
            InfNum {
                real: 0.0,
                inf: 0.0,
            },
        ];
        for i in 2..tableinst[0].len() {
            match tableinst[tableinst.len() - 1][i] {
                TableCell::Value(expr) => assert!(expr == exp[i - 2]),
                _ => {}
            }
        }
    }

    #[test]
    fn solver_check_optimality() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(5.5, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-1.5, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-5.0, 0.0);
        inst.constraints[len].sign = Sign::Equal;
        let tableinst = inst.get_simplex_table();
        assert!(!inst.check_optimality(&tableinst));
    }

    #[test]
    fn solver_check_optimality1() {
        let table = vec![
            vec![
                TableCell::Value(InfNum::from(-1.0, 0.0)),
                TableCell::Value(InfNum::from(-2.0, 0.0)),
                TableCell::Value(InfNum::from(0.0, 0.0)),
                TableCell::Value(InfNum::from(0.0, -1.0)),
            ],
        ];
        let inst = Solver::new();
        assert!(inst.check_optimality(&table));
    }

    #[test]
    fn solver_check_limitlessness() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("y"),
            InfNum::from(2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("z"),
            InfNum::from(-3.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(5.5, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-1.5, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("y"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("z"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(-5.0, 0.0);
        inst.constraints[len].sign = Sign::Equal;
        let tableinst = inst.get_simplex_table();
        assert!(!inst.check_limitlessness(&tableinst));
    }

    #[test]
    fn solver_check_limitlessness1() {
        let inst = Solver::new();
        let table = vec![
            vec![
                TableCell::Name(String::from("MIN")),
                TableCell::Name(String::from("MIN")),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 2.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 3.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.0,
                }),
            ],
            vec![
                TableCell::Name(String::from("Cb")),
                TableCell::Name(String::from("Vb")),
                TableCell::Name(String::from("b")),
                TableCell::Name(String::from("x")),
                TableCell::Name(String::from("y")),
                TableCell::Name(String::from("z")),
                TableCell::Name(String::from("}=j*4zv\u{7f}4V")),
                TableCell::Name(String::from("u0$(L8-8Nh")),
                TableCell::Name(String::from("P=},. 09y0")),
            ],
            vec![
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.0,
                }),
                TableCell::Name(String::from("}=j*4zv\u{7f}4V")),
                TableCell::Value(InfNum {
                    real: -0.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: -1.0,
                    inf: -1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
            ],
            vec![
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.0,
                }),
                TableCell::Name(String::from("u0$(L8-8Nh")),
                TableCell::Value(InfNum {
                    real: -0.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: -1.0,
                }),
                TableCell::Value(InfNum {
                    real: -1.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
            ],
            vec![
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 1.10,
                }),
                TableCell::Name(String::from("P=},. 09y0")),
                TableCell::Value(InfNum {
                    real: -0.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: -1.0,
                    inf: 1.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 1.0,
                    inf: 0.0,
                }),
            ],
            vec![
                TableCell::Name(String::from("UNDEFINED")),
                TableCell::Name(String::from("UNDEFINED")),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 3.0,
                }),
                TableCell::Value(InfNum {
                    real: -1.0,
                    inf: 4.0,
                }),
                TableCell::Value(InfNum {
                    real: -2.0,
                    inf: 2.0,
                }),
                TableCell::Value(InfNum {
                    real: -3.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
                TableCell::Value(InfNum {
                    real: 0.0,
                    inf: 0.0,
                }),
            ],
        ];
        assert!(inst.check_limitlessness(&table));
    }

    #[test]
    fn solver_imorive_table() {
        let mut inst = Solver::new();
        inst.target_function = Function::new();
        inst.target_function.add_variable(
            String::from("x1"),
            InfNum::from(-2.0, 0.0),
        );
        inst.target_function.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        inst.target_value = TargetValue::Min;
        inst.constraints.push(Consraint::new());
        let mut len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x1"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(1.0, 0.0);
        inst.constraints[len].sign = Sign::GreaterOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x1"),
            InfNum::from(3.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("x2"),
            InfNum::from(2.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(3.0, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        inst.constraints.push(Consraint::new());
        len = inst.constraints.len() - 1;
        inst.constraints[len].left.add_variable(
            String::from("x1"),
            InfNum::from(-2.0, 0.0),
        );
        inst.constraints[len].left.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        inst.constraints[len].right = InfNum::from(1.0, 0.0);
        inst.constraints[len].sign = Sign::SmallerOrEqual;
        let mut table = inst.get_simplex_table();
        inst.improve_table(&mut table);
    }

    #[test]
    fn solver_solve() {
        let mut solver = Solver::new();
        solver.target_function.add_variable(String::from("x"), InfNum::from(0.2, 0.0));
        solver.target_function.add_variable(String::from("y"), InfNum::from(0.08, 0.0));
        solver.target_value = TargetValue::Min;
        solver.constraints.push(Consraint::new());
        solver.constraints[0].left.add_variable(String::from("x"), InfNum::from(0.1, 0.0));
        solver.constraints[0].sign = Sign::GreaterOrEqual;
        solver.constraints[0].right = InfNum::from(0.4, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[1].left.add_variable(String::from("y"), InfNum::from(0.1, 0.0));
        solver.constraints[1].sign = Sign::GreaterOrEqual;
        solver.constraints[1].right = InfNum::from(0.6, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[2].left.add_variable(String::from("x"), InfNum::from(0.1, 0.0));
        solver.constraints[2].left.add_variable(String::from("y"), InfNum::from(0.2, 0.0));
        solver.constraints[2].sign = Sign::GreaterOrEqual;
        solver.constraints[2].right = InfNum::from(2.0, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[3].left.add_variable(String::from("x"), InfNum::from(0.2, 0.0));
        solver.constraints[3].left.add_variable(String::from("y"), InfNum::from(0.1, 0.0));
        solver.constraints[3].sign = Sign::GreaterOrEqual;
        solver.constraints[3].right = InfNum::from(1.7, 0.0);
        println!("{:?}", solver.solve());
        assert!(1 == 0);
    }

    #[test]
    fn solver_solve1() {
        let mut solver = Solver::new();
        solver.target_function.add_variable(
            String::from("x1"),
            InfNum::from(-2.0, 0.0),
        );
        solver.target_function.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        solver.target_value = TargetValue::Min;
        solver.constraints.push(Consraint::new());
        solver.constraints[0].left.add_variable(
            String::from("x1"),
            InfNum::from(1.0, 0.0),
        );
        solver.constraints[0].left.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        solver.constraints[0].sign = Sign::GreaterOrEqual;
        solver.constraints[0].right = InfNum::from(1.0, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[1].left.add_variable(
            String::from("x1"),
            InfNum::from(-3.0, 0.0),
        );
        solver.constraints[1].left.add_variable(
            String::from("x2"),
            InfNum::from(2.0, 0.0),
        );
        solver.constraints[1].sign = Sign::SmallerOrEqual;
        solver.constraints[1].right = InfNum::from(3.0, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[2].left.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        solver.constraints[2].sign = Sign::SmallerOrEqual;
        solver.constraints[2].right = InfNum::from(3.0, 0.0);
        solver.constraints.push(Consraint::new());
        solver.constraints[3].left.add_variable(
            String::from("x1"),
            InfNum::from(-2.0, 0.0),
        );
        solver.constraints[3].left.add_variable(
            String::from("x2"),
            InfNum::from(1.0, 0.0),
        );
        solver.constraints[3].sign = Sign::SmallerOrEqual;
        solver.constraints[3].right = InfNum::from(1.0, 0.0);
        assert_eq!(
            solver.solve(),
            Result::Err(String::from("Function is unlimited!"))
        );
    }
}
