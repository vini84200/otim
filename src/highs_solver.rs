use std::time::Instant;

use crate::scheduling::Size;
use good_lp::{default_solver, variable, variables, Solution, SolverModel, Variable};

pub struct HighsSolver {
    values: Vec<Size>,
    maxIt: usize,
    verbose: bool,
    maxTime: i32,
}

impl HighsSolver {
    pub fn new(values: Vec<Size>) -> Self {
        Self {
            values,
            maxIt: 100000,
            maxTime: 60 * 60,
            verbose: false,
        }
    }

    pub fn setMaxIt(&mut self, maxIt: usize) {
        self.maxIt = maxIt;
    }

    pub fn setVerbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn setMaxTime(&mut self, maxTime: i32) {
        self.maxTime = maxTime;
    }

    fn get_min_task_time(&self) -> Vec<Size> {
        let n = self.values.len();
        let mut m = vec![0; n * n];
        for i in 0..n {
            for j in i..n {
                let min = self.values.get(i).unwrap().min(self.values.get(j).unwrap());
                m[j * n + i] = *min;
                m[i * n + j] = *min;
            }
        }

        m
    }

    pub fn run(&mut self) {
        // Precomputa menor valor entre quaisquer duas tarefas
        println!("Iniciando Solver de PI...");
        let now = Instant::now();
        let min_task_time = self.get_min_task_time();
        let n = self.values.len();
        let big: i32 = self.values.iter().sum();

        variables! {
            vars:
                m >= 0;
        }

        let s: Vec<Variable> = self
            .values
            .iter()
            .map(|val| vars.add(variable().integer().min(0)))
            .collect();
        let mut y = Vec::new();
        for i in 0..n {
            let mut yi = Vec::new();
            for j in 0..n {
                yi.push(vars.add(variable().binary()));
            }
            y.push(yi);
        }

        let mut problem = vars.minimise(m).using(default_solver);

        for i in 0..n {
            let pi = self.values[i];
            let si = s[i];
            // m precisa ser maior que a terminação de todas as tarefas
            problem.add_constraint(m - si - pi >> 0);

            for j in 0..n {
                if i == j {
                    continue;
                }

                let mij = min_task_time[i * n + j];
                let sj = s[j];
                let yij = y[i][j];
                problem.add_constraint(si - sj + big * yij >> mij);
                problem.add_constraint(-si + sj + big * (1 - yij) >> mij);
            }
        }
        let formulate_time = now.clone().elapsed();
        println!("Formulated problem in {:?}", formulate_time);

        let start_solve = Instant::now();
        println!("Started solving...");

        problem.set_parameter("maxIt", self.maxIt.to_string().as_str());
        if self.verbose {
            problem.set_parameter("sLog", "1");
        } else {
            problem.set_parameter("sLog", "0");
        }

        problem.set_parameter("sec", self.maxTime.to_string().as_str());
        problem.set_parameter("ratio", "0.02");

        match problem.solve() {
            Ok(solution) => {
                println!("Finished Solver in {:?}", start_solve.elapsed());
                println!("Total time: {:?}", now.elapsed());

                println!("Solution found: {}", solution.value(m));
            }
            Err(e) => {
                println!("Error in {:?}", start_solve.elapsed());
                println!("ERROR {}", e.to_string());
            }
        }
    }
}
