use linsol::inf_num::InfNum;
use linsol::function::Function;
use linsol::constraint::Sign;
use linsol::constraint::Consraint;
use linsol::utilities::get_random_name;
use std::collections::HashMap;
use std::f64;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TargetValue {
    Min,
    Max,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Solver {
    pub target_function: Function,
    pub target_value: TargetValue,
    pub constraints: Vec<Consraint>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TableCell {
    Name(String),
    Value(InfNum),
}

impl PartialEq for TableCell {
    fn eq(&self, other: &TableCell) -> bool {
        match self {
            &TableCell::Name(ref x) => {
                match other {
                    &TableCell::Name(ref y) => return x == y,
                    _ => {}
                }
            }
            &TableCell::Value(ref x) => {
                match other {
                    &TableCell::Value(ref y) => return x == y,
                    _ => {}
                }
            }
        }
        false
    }
}

impl Eq for TableCell {}

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

    pub fn canonical_form(&mut self) {
        for i in 0..self.constraints.len() {
            let curr = &mut self.constraints[i];
            let mut name = get_random_name(10);
            match curr.sign {
                Sign::GreaterOrEqual => {
                    while !curr.left.add_variable(
                        name.clone(),
                        InfNum::from(-1.0, 0.0),
                    )
                    {
                        name = get_random_name(10);
                    }
                    self.target_function.add_variable(
                        name,
                        InfNum::from(0.0, 0.0),
                    );
                }
                Sign::SmallerOrEqual => {
                    while !curr.left.add_variable(name.clone(), InfNum::from(1.0, 0.0)) {
                        name = get_random_name(10);
                    }
                    self.target_function.add_variable(
                        name,
                        InfNum::from(0.0, 0.0),
                    );
                }
                _ => {}
            }
            curr.sign = Sign::Equal;
            if curr.right < InfNum::from(0.0, 0.0) {
                curr.right *= InfNum::from(-1.0, 0.0);
                curr.left *= InfNum::from(-1.0, 0.0);
            }
        }
    }

    pub fn base_form(&mut self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        for i in &mut self.constraints {
            let mut curr = get_random_name(10);
            while !i.left.add_variable(curr.clone(), InfNum::from(1.0, 0.0)) {
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

    pub fn get_simplex_table(&mut self) -> SimplexTable {
        self.canonical_form();
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
            res[0][i + 3] = TableCell::Value(self.target_function.coefficients[i]);
            res[1][i + 3] = TableCell::Name(self.target_function.variables[i].clone());
        }
        for i in 0..bases.len() {
            res[i + 2][0] = TableCell::Value(InfNum::from(0.0, 1.0));
            res[i + 2][1] = TableCell::Name(bases[i].clone());
            res[i + 2][2] = TableCell::Value(self.constraints[i].right);
        }

        for i in 0..bases.len() {
            for j in 0..self.target_function.variables.len() {
                res[i + 2][j + 3] = TableCell::Value(self.constraints[i].left.get_coefficient(
                    self.target_function.variables[j].clone(),
                ));
            }
        }

        for i in 0..self.target_function.variables.len() + 1 {
            let mut resu = InfNum::from(0.0, 0.0);
            for j in 0..bases.len() {
                match res[j + 2][i + 2] {
                    TableCell::Value(x) => {
                        match res[j + 2][0] {
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
            res[bases.len() + 2][i + 2] = TableCell::Value(resu);
        }
        res
    }

    pub fn check_optimality(&self, table: &SimplexTable) -> bool {
        for i in 0..table[0].len() - 2 {
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

    pub fn check_limitlessness(&self, table: &SimplexTable) -> bool {
        for i in 0..table[0].len() - 2 {
            match table[table.len() - 1][i + 2] {
                TableCell::Value(x) => {
                    if x > InfNum::from(0.0, 0.0) {
                        let mut curr = false;
                        for j in 0..table.len() - 3 {
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
/*
    pub fn improve_table(&self, table: &mut SimplexTable) {
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
        for i in 0..table.len() - 3 {
            match table[i + 2][2] {
                TableCell::Value(x) => {
                    match table[i + 2][selectionv] {
                        TableCell::Value(y) => {
                            if y > InfNum::from(0.0, 0.0) {
                                if x / y < minbr {
                                    minbr = x / y;
                                    selectionb = i + 2;
                                }
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

        for j in 0..table[0].len() - 2 {
            match table[selectionb][j + 2].clone() {
                TableCell::Value(x) => {
                    match table[selectionb][selectionv] {
                        TableCell::Value(y) => table[selectionb][j + 2] = TableCell::Value(x / y),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        for i in 0..table.len() - 2 {
            if i + 2 != selectionb {
                let mut coefficient = InfNum::from(1.0, 1.0);
                match table[selectionb][selectionv] {
                    TableCell::Value(x) => {
                        match table[i + 2][selectionv] {
                            TableCell::Value(y) => coefficient = y / x,
                            _ => {}
                        }
                    }
                    _ => {}
                }
                for j in 0..table[i].len() - 2 {
                    match table[i + 2][j + 2] {
                        TableCell::Value(x) => {
                            match table[selectionb][j + 2] {
                                TableCell::Value(y) => {
                                    table[i + 2][j + 2] = TableCell::Value(x - coefficient * y)
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

*/
    pub fn improve_table(&self, table: &mut SimplexTable) {
        let mut selectionv: usize = 0;
        let mut maxv = InfNum::from(-1.0, -1.0);
        let mut selectionb: usize = 0;
        let mut minb = InfNum::from(1.0, 1.0);

        for i in 0..table[0].len() - 3 {
            match table[table.len() - 1][i + 3].clone() {
                TableCell::Value(x) => {
                    if x > maxv {
                        maxv = x;
                        selectionv = i + 3;
                    }
                }
                _ => {}
            }
        }

        for i in 0..table.len() - 3 {
            match table[i + 2][selectionv] {
                TableCell::Value(x) => {
                    match table[i + 2][2] {
                        TableCell::Value(y) => {
                            if x / y < minb {
                                minb = x / y;
                                selectionb = i + 2;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        let tmp = table[selectionb][0].clone();
        table[selectionb][0] = table[0][selectionv].clone();
        table[0][selectionv] = tmp;
        let tmp = table[selectionb][1].clone();
        table[selectionb][1] = table[1][selectionv].clone();
        table[1][selectionv] = tmp;
        let curr = match table[selectionb][selectionv] {
            TableCell::Value(x) => x,
            _ => InfNum::from(f64::NAN, f64::NAN),
        };

        for i in 0..table[0].len() - 2 {
            match table[selectionb][i + 2] {
                TableCell::Value(x) => table[selectionb][i + 2] = TableCell::Value(x / curr),
                _ => {}
            }
        }

        for i in 0..table.len() - 2 {
            if i + 2 != selectionb {
                let coef = match table[i + 2][selectionv] {
                    TableCell::Value(x) => x / curr,
                    _ => InfNum::from(f64::NAN, f64::NAN),
                };
                for j in 0..table[i].len() - 2 {
                    table[i + 2][j + 2] = TableCell::Value(match table[i + 2][j + 2] {
                        TableCell::Value(x) => {
                            x -
                                coef *
                                    match table[selectionb][j + 2] {
                                        TableCell::Value(y) => y,
                                        _ => InfNum::from(f64::NAN, f64::NAN),
                                    }
                        }
                        _ => InfNum::from(f64::NAN, f64::NAN),
                    })
                }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn solve(&mut self) -> Result<HashMap<String, InfNum>, String> {
        match self.target_value {
            TargetValue::Max => self.target_function *= InfNum::from(-1.0, 1.0),
            _ => {}
        }
        self.canonical_form();
        let mut table = self.get_simplex_table();
        let mut plok = String::new();
        while !self.check_optimality(&table) {
            for i in &table {
                for j in i {
                    plok.push_str(&format!("{: <60} | ", format!("{:?}", j)));
                }
                plok.push_str("\n")
            }
            plok.push_str(&format!("\n----------------------------\n"));
            if self.check_limitlessness(&table) {
                return Result::Err(String::from("Function is unlimited!"));
            }
            self.improve_table(&mut table);
        }
        for i in &table {
            for j in i {
                plok.push_str(&format!("{: <60} | ", format!("{:?}", j)));
            }
            plok.push_str("\n")
        }
        plok.push_str(&format!("\n----------------------------\n"));
        panic!("{}", plok);
        let mut res = HashMap::<String, InfNum>::new();
        for i in 0..table.len() - 3 {
            match table[i + 2][1].clone() {
                TableCell::Name(x) => {
                    let _ = match table[i + 2][2] {
                        TableCell::Value(y) => res.insert(x, y),
                        _ => panic!("Got name in cell with value expected!"),
                    };
                }
                _ => {}
            }
        }
        Result::Ok(res)
    }
}
