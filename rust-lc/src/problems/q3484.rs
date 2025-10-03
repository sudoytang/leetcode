#[allow(unused)]
struct Spreadsheet {
    cells: Vec<[i32; 26]>,
}

#[allow(unused)]
enum Expr {
    Num(i32),
    CellRef(usize, usize),
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl Spreadsheet {

    fn parse_idx(expr: &str) -> Expr {
        match expr.parse::<i32>() {
            Ok(val) => Expr::Num(val),
            Err(_) => {
                // at here the format should be [A-Z]\d+
                let col = (expr.as_bytes()[0] - b'A') as usize;
                let row = expr[1..].parse::<usize>().expect("Invalid expr!!") - 1;
                Expr::CellRef(row, col)
            }
        }
    }

    fn new(rows: i32) -> Self {
        Self {
            cells: vec![[0; 26]; rows as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        if let Expr::CellRef(r, c) = Self::parse_idx(&cell) {
            self.cells[r][c] = value;
        } else {
            panic!("Invalid Cell Expr: {}", cell);
        }
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn get_value(&self, formula: String) -> i32 {
        formula[1..]
            .split('+')
            .map(|s| {
                match Self::parse_idx(s) {
                    Expr::Num(v) => v,
                    Expr::CellRef(i, j) => self.cells[i][j],
                }
            }).sum()
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */

#[allow(unused)]
struct PH;