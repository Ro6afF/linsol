use linsol::inf_num::InfNum;
use linsol::function::Function;
use linsol::constraint::Sign;
use linsol::constraint::Consraint;
use linsol::utilities::get_random_name;
use std::collections::HashMap;

#[allow(dead_code)]
pub enum TargetValue {
    Min,
    Max,
}

#[allow(dead_code)]
pub struct Solver {
    pub target_function: Function,
    pub target_value: TargetValue,
    pub constraints: Vec<Consraint>,
}

#[derive(Clone)]
#[allow(dead_code)]
enum TableCell {
    Name(String),
    Value(InfNum),
}

#[allow(dead_code)]
type SimplexTable = Vec<Vec<TableCell>>;

#[allow(dead_code)]
impl Solver {
    pub fn new() -> Solver {
        Solver {
            target_function: Function::new(),
            target_value: TargetValue::Min,
            constraints: Vec::<Consraint>::new(),
        }
    }

    pub fn from(function: Function, value: TargetValue, constraints: Vec<Consraint>) -> Solver {
        Solver {
            target_function: function,
            target_value: value,
            constraints: constraints,
        }
    }

    fn canonical_form(&mut self) {
        for i in 0..self.constraints.len() {
            let curr = &mut self.constraints[i];
            match curr.sign {
                Sign::GreaterOrEqual => {
                    while curr.left.add_variable(
                        get_random_name(10),
                        InfNum::from(-1.0, 0.0),
                    )
                    {}
                }
                Sign::SmallerOrEqual => {
                    while curr.left.add_variable(
                        get_random_name(10),
                        InfNum::from(1.0, 0.0),
                    )
                    {}
                }
                _ => {}
            }
            curr.sign = Sign::Equal;
            if curr.right < InfNum::from(0.0, 0.0) {
                curr.right *= InfNum::from(-1.0, 1.0);
                curr.left *= InfNum::from(-1.0, 1.0);
            }
        }
    }

    fn base_form(&mut self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        for i in &mut self.constraints {
            let mut curr = get_random_name(10);
            while i.left.add_variable(curr.clone(), InfNum::from(1.0, 0.0)) {
                curr = get_random_name(10);
            }
            res.push(curr.clone());
            self.target_function.add_variable(
                curr,
                InfNum::from(0.0, 1.0),
            );
        }
        res
    }

    fn get_simplex_table(&mut self) -> SimplexTable {
        let bases = self.base_form();
        let mut res = SimplexTable::new();
        for i in 0..bases.len() + 3 {
            res.push(Vec::<TableCell>::new());
            for _ in 0..self.target_function.variables.len() + 3 {
                res[i].push(TableCell::Name(String::from("UNDEFINED")));
            }
        }
        res[0][0] = TableCell::Name(String::from("MIN"));
        res[0][1] = TableCell::Name(String::from("MIN"));
        res[0][2] = TableCell::Value(InfNum::from(0.0, 0.0));
        res[1][0] = TableCell::Name(String::from("Cb"));
        res[1][1] = TableCell::Name(String::from("Vb"));
        res[1][2] = TableCell::Name(String::from("b"));
        for i in 0..self.target_function.variables.len() {
            res[0][i + 2] = TableCell::Value(self.target_function.coeficients[i]);
            res[1][i + 2] = TableCell::Name(self.target_function.variables[i].clone());
        }
        for i in 0..bases.len() {
            res[i + 2][0] = TableCell::Value(InfNum::from(0.0, 1.0));
            res[i + 2][1] = TableCell::Name(bases[i].clone());
            res[i + 2][2] = TableCell::Value(self.constraints[i].right);
        }

        for i in 0..bases.len() {
            for j in 0..self.target_function.variables.len() {
                res[i + 2][j + 2] = TableCell::Value(self.constraints[i].left.get_coeficient(
                    self.target_function.variables[i].clone(),
                ));
            }
        }

        for i in 0..self.target_function.variables.len() + 1 {
            let mut resu = InfNum::from(0.0, 0.0);
            for j in 0..bases.len() {
                match res[j + 2][i + 1] {
                    TableCell::Value(x) => {
                        match res[j + 3][0] {
                            TableCell::Value(y) => resu += x * y,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            match res[0][i + 2] {
                TableCell::Value(x) => resu -= x,
                _ => {}
            }
            res[bases.len() + 3][i + 2] = TableCell::Value(resu);
        }
        res
    }

    fn check_optimality(&self, table: &SimplexTable) -> bool {
        for i in 0..table[0].len() {
            match table[table.len() - 1][i + 2] {
                TableCell::Value(x) => {
                    if x > InfNum::from(0.0, 0.0) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }

    fn check_limitlessness(&self, table: &SimplexTable) -> bool {
        for i in 0..table[0].len() {
            match table[table.len() - 1][i + 2] {
                TableCell::Value(x) => {
                    if x > InfNum::from(0.0, 0.0) {
                        let mut curr = false;
                        for j in 0..table.len() {
                            match table[j + 2][i + 2] {
                                TableCell::Value(y) => {
                                    if y > InfNum::from(0.0, 0.0) {
                                        curr = true;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if !curr {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn improve_table(&self, table: &mut SimplexTable) {
        let mut selectionv: usize = 0;
        let mut maxdv = InfNum::from(0.0, -1.0);
        for i in 0..table[table.len() - 1].len() - 3 {
            match table[table.len() - 1][i + 3] {
                TableCell::Value(x) => {
                    if x > maxdv {
                        maxdv = x;
                        selectionv = i + 3;
                    }
                }
                _ => {}
            }
        }
        let mut selectionb: usize = 0;
        let mut minbr = InfNum::from(0.0, 1.0);
        for i in 0..table.len() {
            match table[i + 3][2] {
                TableCell::Value(x) => {
                    match table[i + 3][selectionv] {
                        TableCell::Value(y) => {
                            if x / y < minbr {
                                minbr = x / y;
                                selectionb = i;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let tmp = table[0][selectionv].clone();
        table[0][selectionv] = table[selectionb][0].clone();
        table[selectionb][0] = tmp;
        let tmp = table[1][selectionv].clone();
        table[1][selectionv] = table[selectionb][1].clone();
        table[selectionb][1] = tmp;

        for i in 0..table.len() - 3 {
            if i != selectionb {
                let coeficient;
                match table[selectionb][selectionv] {
                    TableCell::Value(x) => {
                        match table[i + 3][selectionv] {
                            TableCell::Value(y) => coeficient = y / x,
                            _ => {coeficient = InfNum::new()}
                        }
                    }
                    _ => {coeficient = InfNum::new()}
                }
                for j in 0..table[i].len() - 3 {
                    match table[i][j] {
                        TableCell::Value(x) => {
                            match table[selectionb][j] {
                                TableCell::Value(y) => table[i][j] = TableCell::Value(x - coeficient * y),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn solve(&mut self) -> Result<HashMap<String, InfNum>, String> {
        let /*mut*/ res = HashMap::<String, InfNum>::new();
        self.canonical_form();
        let mut table = self.get_simplex_table();
        while !self.check_optimality(&table) {
            if self.check_limitlessness(&table) {
                return Result::Err(String::from("Function is unlimited!"));
            }
            self.improve_table(&mut table);
        }
        Result::Ok(res)
    }
}
