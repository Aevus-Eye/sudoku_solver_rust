use std::time::Instant;

type RuleFn = fn(usize, i8, &SudBoard) -> bool;
type OptimizeFn = fn(&mut SudBoard) -> RuleResult;

struct SudokuSolver {
    rules: Vec<RuleFn>,
    optis: Vec<OptimizationFn>,
    sudoku_stack: Vec<SudBoard>,
    solutions: Vec<SudBoard>,
    wrong_solutions: Vec<SudBoard>,
    start_time: Instant,
    first_sol_time: Option<Instant>,
}

impl SudokuSolver {
    pub fn new(initial_board: [i8; 9 * 9], rules: Vec<RuleFn>, optis: Vec<OptimizationFn>) -> SudokuSolver {
        let pos: Vec<i8> = (1..=9).collect();
        let ss = SudokuSolver {
            sudoku_stack: vec![SudBoard {
                state: initial_board
                    .iter()
                    .map(|&x| {
                        if x == 0 {
                            Field::Possible(pos.clone())
                        } else {
                            Field::Value(x)
                        }
                    })
                    .collect(),
            }],
            rules, optis,
            solutions: vec![],
            wrong_solutions: vec![],
            start_time: Instant::now(),
            first_sol_time: None,
        };
        if ss.sudoku_stack[0].state.len() != 9 * 9 {
            panic!("sudoku is not of the correct length");
        }
        ss
    }

    pub fn top(&self) -> &SudBoard {
        &self.sudoku_stack[self.sudoku_stack.len() - 1]
    }

    pub fn top_mut(&mut self) -> &mut SudBoard {
        let index = self.sudoku_stack.len() - 1;
        &mut self.sudoku_stack[index]
    }

    //pub fn get(&self, index: usize) -> &Field {
    //    &self.top().state[index]
    //}

    pub fn set(&mut self, index: usize, value: Field) {
        /*if let Field::Possible(v) = &value {
            if v.len() == 0 {
                // println!("{}", index);
                return false;
            }
            if v.len() == 1 {
                value = Field::Value(v[0]);
            }
        }

        if self.top().state[index] != value {
            self.changed = true;
            self.top_mut().state[index] = value;
        }
        return true;*/
        self.top_mut().state[index] = value;
    }

    pub fn push(&mut self) {
        //println!("push");
        let mut x: Vec<(usize, Vec<i8>)> = self
            .top()
            .state
            .iter()
            .enumerate()
            .filter_map(|(i, x)| match x {
                Field::Possible(y) => Some((i, y.clone())),
                _ => None,
            })
            .collect();

        x.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        if x.len() == 0 {
            //win check
            if self.solutions.contains(self.top()) {
                println!("found duplicate");
            } else {
                //println!("solution found!");
                //print_sudoku(self.top());
                self.solutions.push(self.top().clone());
                if self.solutions.len() == 1 {
                    self.first_sol_time = Some(Instant::now());
                }
            }
            //panic!("sudoku solved!");
        }

        // brute force
        for e in x {
            for v in e.1 {
                //println!("push {}", self.sudoku_stack.len());
                //let lsi = self.sudoku_stack.len() - 1;
                self.sudoku_stack.push(self.top().clone());
                self.set(e.0, Field::Value(v));
                if self.solutions.iter().any(|sol| self.top().is_inside(sol)) {
                    self.pop(true);
                    //println!("quick pop");
                    continue;
                }
                if self
                    .wrong_solutions
                    .iter()
                    .any(|wsol| wsol.is_inside(self.top()))
                {
                    self.pop(false);
                    //println!("quick pop wrong");
                    continue;
                }
                self.solve_loop();
            }
        }
    }

    pub fn pop(&mut self, push_on_wrong_stack: bool) {
        let s = self.sudoku_stack.pop();
        if push_on_wrong_stack {
            if let Some(s) = s {
                self.wrong_solutions.push(s);
            }
        }
        //println!("pop {}", self.sudoku_stack.len());

        //if self.sudoku_stack.len()==1{
        //   println!("test!");
        //}
        //if self.sudoku_stack.len() == 0 {
        // panic!("search ended");
        //}
    }

    pub fn solve(&mut self) -> bool {
        //println!("iterate");
        let mut res = RuleResult::Unchanged;
        for i in 0..(self.rules.len()) {
            //for r in self.rules.iter(){
            //match r(self.top_mut()){
            match self.rules[i](self.top_mut()) {
                RuleResult::Impossible => {
                    self.pop(true);
                    return false;
                }
                RuleResult::Changed => res = RuleResult::Changed,
                _ => {}
            }
            //println!("{:?}",res);
        }

        /*for i in 0..(9 * 9) {
            match self.get(i) {
                Field::Value(_) => continue,
                Field::Possible(p) => {
                    let mut p = p.to_vec();
                    for r in self.rules.iter() {
                        let pa = r(i, &self);
                        p.retain(|x| pa.contains(x));
                    } //for
                    if !self.set(i, Field::Possible(p)) {
                        self.pop(true);
                        return false;
                    }
                } //match branch
            } //match
        } //for
        */
        if res == RuleResult::Unchanged {
            self.push();
            self.pop(true);
            return false;
        }
        return true;
    } //fn

    pub fn solve_loop(&mut self) {
        while self.solve() {}
    }
} //impl

#[derive(PartialEq, Clone)]
pub enum Field {
    Value(i8),
    Possible(Vec<i8>),
}
#[derive(PartialEq, Clone)]
pub struct SudBoard {
    pub state: Vec<Field>,
}
#[derive(PartialEq, Debug)]
enum RuleResult {
    Unchanged,
    Changed,
    Impossible,
}

impl SudBoard {
    fn is_inside(&self, other: &SudBoard) -> bool {
        self.state
            .iter()
            .zip(other.state.iter())
            .skip_while(|(a, b)| {
                if let Field::Value(b2) = b {
                    if let Field::Value(a2) = a {
                        return a2 == b2;
                    }
                    return true;
                }
                return true;
            })
            .next()
            .is_none()
    }
}

/*
fn rule_helper_invert(inv: &[i8], all_values: &[i8]) -> Vec<i8> {
    let mut res: Vec<i8> = all_values.to_vec();
    res.retain(|&x| !inv.contains(&x));
    res
}
*/

fn rule_line_h(index: usize, cur_val: i8, sb: &SudBoard) -> bool {
    for i in (index / 9 * 9)..(index / 9 * 9 + 9) {
        if let Field::Value(n) = sb.state[i] {
            if n == cur_val && i != index {
                return false;
            }
        }
    }
    true
}

fn rule_line_v(index: usize, cur_val: i8, sb: &SudBoard) -> bool {
    for i in ((index % 9)..(9 * 9)).step_by(9) {
        if let Field::Value(n) = sb.state[i] {
            if n == cur_val && i != index {
                return false;
            }
        }
    }
    true
}

fn rule_block(index: usize, cur_val: i8, sb: &SudBoard) -> bool {
    let x = index % 9 / 3 * 3;
    let y = index / 9 / 3 * 3;
    let start = x + y * 9;

    for h in 0..3 {
        for i in (start + h * 9)..(start + 3 + h * 9) {
            if let Field::Value(n) = sb.state[i] {
                if n == cur_val && i != index {
                    return false;
                }
            }
        }
    }
    return true;
}

fn rule_normal_sudoku(index: usize, cur_val: i8, sb: &SudBoard) -> bool {
    let x = index % 9 / 3 * 3;
    let y = index / 9 / 3 * 3;
    let start = x + y * 9;

    let mut it = ((index % 9)..(9 * 9))
        .step_by(9)
        .chain((index / 9 * 9)..(index / 9 * 9 + 9))
        .chain((start + h * 9)..(start + 3 + h * 9))
        .chain((start + h * 9 + 9)..(start + 3 + h * 9 + 9))
        .chain((start + h * 9 + 9 + 9)..(start + 3 + h * 9 + 9 + 9));

    for i in it {
        if let Field::Value(n) = sb.state[i] {
            if n == cur_val && i != index {
                return false;
            }
        }
    }

    return true;
}

//todo: this is an optimization function, NOT a rule function. split this into optimization and
//rule. rule fn take and index and the board and tell you wheter the numver at the index is
//possible, and optimization fn will change the underlying field::possible and remocve digits from
//there. they do not change possibilitys into values! the class will do a trip around the board and
//search for changeable fields or empty fields.
fn optimize_normal_sudoku(sb: &mut SudBoard) -> RuleResult {
    let mut lines = vec![vec![]; 9];
    let mut cols = vec![vec![]; 9];
    let mut blocks = vec![vec![]; 9];
    let mut changed = false;

    for (i, v) in sb.state.iter().enumerate() {
        if let Field::Value(v) = v {
            lines[i / 9].push(*v);
            cols[i % 9].push(*v);
            blocks[i % 9 / 3 + i / 9 / 3 * 3].push(*v);
        }
    }

    //println!("lines {:?}\ncols{:?}\nblocks{:?}",lines,cols,blocks); "\033[0;31m RED \033[0m"

    for (i, f) in sb.state.iter_mut().enumerate() {
        if let Field::Possible(p) = f {
            //p;
            //p.retain(|x| !(lines[i/9/3].contains(&x)));
            //p.swap_remove(0);
            for pi in (0..(p.len())).rev() {
                if lines[i / 9].contains(&&p[pi])
                    || cols[i % 9].contains(&&p[pi])
                    || blocks[i % 9 / 3 + i / 9 / 3 * 3].contains(&&p[pi])
                {
                    p.swap_remove(pi);
                    changed = true;
                }
            }
            if p.len() == 0 {
                return RuleResult::Impossible;
            }

            /*let len = p.len();
            let v = p[0];
            drop(p);
            if len == 1 {
                *f = Field::Value(v);
                lines[i / 9].push(v);
                cols[i % 9].push(v);
                blocks[i % 9 / 3 + i / 9 / 3 * 3].push(v);
            }*/
        }
    }

    if changed {
        RuleResult::Changed
    } else {
        RuleResult::Unchanged
    }
}

fn print_sudoku(sb: &SudBoard) {
    for i in 0..(9 * 9) {
        match sb.state[i] {
            Field::Value(n) => print!("{}", n),
            _ => print!("?"),
        }
        if i % 3 == 2 {
            print!(" ")
        }
        if i % 9 == 8 {
            print!("\n")
        }
        if i % (9 * 3) == 26 {
            print!("\n")
        }
    }
}

fn main() {
    /*for i in 0..(9*9){
        print!("{},",i/9);
    }

    return ;*/

    //let mut _x = sudoku_solver::SsData::new();
    #[rustfmt::skip]
    let mut _y=SudokuSolver::new([
       /* //empty
        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,
        
        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,

        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,
        0,0,0, 0,0,0, 0,0,0,
        */

        //easy 1 sol
        /*0,3,0, 0,1,0, 0,6,0,
        7,5,0, 0,3,0, 0,4,8,
        0,0,6, 9,8,4, 3,0,0,
        
        0,0,3, 0,0,0, 8,0,0,
        9,1,2, 0,0,0, 6,7,4,
        0,0,4, 0,0,0, 5,0,0,

        0,0,1, 6,7,5, 2,0,0,
        6,8,0, 0,9,0, 0,1,5,
        0,9,0, 0,4,0, 0,3,0,*/


        //same as above but 108 solutions
        0,3,0, 0,1,0, 0,6,0,
        0,5,0, 0,0,0, 0,0,8,
        0,0,6, 9,0,4, 3,0,0,
        
        0,0,3, 0,0,0, 8,0,0,
        9,0,2, 0,0,0, 6,7,4,
        0,0,4, 0,0,0, 5,0,0,

        0,0,1, 6,0,5, 2,0,0,
        6,0,0, 0,0,0, 0,0,5,
        0,9,0, 0,4,0, 0,3,0,

        //hard 1 solution
        /*0,0,0, 1,0,0, 0,6,8, 
        0,1,3, 8,0,0, 0,0,0, 
        0,0,0, 0,4,7, 0,0,1, 
        
        0,0,9, 5,0,0, 0,0,4, 
        0,7,2, 0,0,0, 8,3,0, 
        3,0,0, 0,0,8, 6,0,0, 
                
        1,0,0, 3,2,0, 0,0,0, 
        0,0,0, 0,0,4, 1,9,0, 
        4,8,0, 0,0,1, 0,0,0,*/


    ],vec![
       //rule_line_v,
       //rule_line_h,
       //rule_block,
       rule_normal_sudoku,
    ]);

    //println!("{:?}", rule_line_v(26, &_y));
    //println!("{:?}", rule_line_h(26, &_y));
    //println!("{:?}", rule_block(26, &_y));

    _y.solve_loop();
    let end_time = Instant::now();

    println!("stack size {}", _y.sudoku_stack.len());
    for (i, s) in _y.solutions.iter().enumerate() {
        println!("solution {}", i);
        print_sudoku(s);
    }
    println!("solutions {}\n wrong solutions {}\n first solution found after {:.2?}\n program execution time {:.2?}",_y.solutions.len(),_y.wrong_solutions.len(),_y.first_sol_time.unwrap_or(_y.start_time).saturating_duration_since(_y.start_time),end_time.saturating_duration_since(_y.start_time));

    println!("");
}
