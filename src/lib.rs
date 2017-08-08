pub mod linsol;

#[cfg(test)]
mod tests {
    use linsol::inf_num::InfNum;
    use linsol::function::Function;
    use linsol::constraint::Consraint;
    use linsol::constraint::Sign;
    use linsol::utilities::get_random_name;
    use linsol::solver::Solver;
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
        assert!(inst == InfNum::from(4.18, -6.28));
    }

    #[test]
    fn inf_num_div() {
        let inst1: InfNum = InfNum::from(1.9, 3.1);
        let inst2: InfNum = InfNum::from(0.1, -10.0);
        let inst = inst1 / inst2;
        assert!(inst == InfNum::from(19.0, -0.31));
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
        inst1 -= InfNum::from(1.0, -1.0);
        assert!(inst1 == InfNum::from(4.0, -2.0));
    }

    #[test]
    fn inf_num_mulass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 *= InfNum::from(-3.0, 2.0);
        inst1 *= InfNum::from(1.0, -1.0);
        assert!(inst1 == InfNum::from(-6.0, 2.0));
    }

    #[test]
    fn inf_num_divass() {
        let mut inst1 = InfNum::from(2.0, -1.0);
        inst1 /= InfNum::from(-2.0, 2.0);
        inst1 /= InfNum::from(1.0, -1.0);
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
        assert!(inst.get_value(&vals) == InfNum::from(-9.7, 1.0));
    }

    #[test]
    fn function_get_value1() {
        let mut inst = Function::new();
        inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
        inst.add_variable(String::from("y"), InfNum::from(2.0, -1.0));
        inst.add_variable(String::from("z"), InfNum::from(-12.0, 1.0));
        let mut vals = HashMap::<String, InfNum>::new();
        vals.insert(String::from("y"), InfNum::from(1.0, 0.0));
        assert!(inst.get_value(&vals) == InfNum::from(2.0, 0.0));
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
        assert!(inst.get_value(&vals) == InfNum::from(3.3, -1.0));
    }

    #[test]
    fn function_change_coeficient() {
        let mut inst = Function::new();
        inst.add_variable(String::from("x"), InfNum::from(1.0, 0.0));
        inst.add_variable(String::from("y"), InfNum::from(1.0, 0.0));
        inst.change_coeficient(String::from("y"), InfNum::from(2.3, -1.0));
        let mut vals = HashMap::<String, InfNum>::new();
        vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
        vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
        assert!(inst.get_value(&vals) == InfNum::from(3.3, -1.0));
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
        assert!(inst.get_value(&vals) == InfNum::from(3.3, -1.0));
    }

    #[test]
    fn function_divass() {
        let mut inst = Function::new();
        inst.add_variable(String::from("x"), InfNum::from(2.0, 0.0));
        inst.add_variable(String::from("y"), InfNum::from(4.6, -1.0));
        inst /= InfNum::from(2.0, 1.0);
        let mut vals = HashMap::<String, InfNum>::new();
        vals.insert(String::from("x"), InfNum::from(1.0, 1.0));
        vals.insert(String::from("y"), InfNum::from(1.0, 1.0));
        vals.insert(String::from("z"), InfNum::from(1.0, 1.0));
        assert!(inst.get_value(&vals) == InfNum::from(3.3, -1.0));
    }

    #[test]
    fn function_get_coeficient() {
        let mut inst = Function::new();
        inst.add_variable(String::from("x"), InfNum::from(-12.34, 56.78));
        assert!(inst.get_coeficient(String::from("x")) == InfNum::from(-12.34, 56.78));
    }

    #[test]
    fn function_get_coeficient1() {
        let mut inst = Function::new();
        inst.add_variable(String::from("x"), InfNum::from(-12.34, 56.78));
        assert!(inst.get_coeficient(String::from("y")) == InfNum::from(0.0, 0.0));
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
        assert!(
            res.target_function.coeficients == inst.target_function.coeficients &&
                res.target_function.variables == inst.target_function.variables
        );
        for i in 0..res.constraints.len() {
            for j in 0..res.constraints[i].left.variables.len() {
                assert!(
                    res.constraints[i].left.variables[j] == inst.constraints[i].left.variables[j]
                );
                assert!(
                    res.constraints[i].left.coeficients[j] == inst.constraints[i].left.coeficients[j]
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
        assert!(out.len() == 3);
        assert!(inst.target_function.coeficients[3] == InfNum::from(0.0, 1.0));
        assert!(inst.target_function.coeficients[4] == InfNum::from(0.0, 1.0));
        assert!(inst.target_function.coeficients[5] == InfNum::from(0.0, 1.0));
        assert!(inst.constraints[0].left.coeficients[3] == InfNum::from(1.0, 0.0));
        assert!(inst.constraints[1].left.coeficients[3] == InfNum::from(1.0, 0.0));
        assert!(inst.constraints[2].left.coeficients[2] == InfNum::from(1.0, 0.0));
    }
}