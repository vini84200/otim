use std::time::Instant;

use crate::scheduling::Size;
use crate::Scheduling;
use rand::Rng;

const ITERSTOCOOL: usize = 100;
const ITERMAXMETROPOLES: usize = 100;

#[derive(Debug)]
pub struct SimulatedAnnealing {
    values: Vec<Size>,
    temperatura: f64,
    iterstocool: usize,
    itermaxmetropoles: usize,
    resfriamento: f64,
    result: Option<Vec<Size>>,
}
fn gen_vec_n_shuffle(v_in: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut v = v_in.clone();
    let mut rng = rand::thread_rng();
    let last_indice = v.len() - 1;
    for offset in 0..n {
        let indice = rng.gen_range(0..=last_indice);
        v.swap(indice, last_indice - offset);
    }
    v
}

impl SimulatedAnnealing {
    pub fn new(values: Vec<Size>, temperatura: f64) -> Self {
        Self {
            values,
            temperatura,
            iterstocool: ITERSTOCOOL,
            itermaxmetropoles: ITERMAXMETROPOLES,
            resfriamento: 0.85f64,
            result: None,
        }
    }

    pub fn set_iterstocool(&mut self, val: usize) {
        self.iterstocool = val;
    }

    pub fn set_itermaxmetropoles(&mut self, val: usize) {
        self.itermaxmetropoles = val;
    }

    pub fn set_resfriamento(&mut self, val: f64) {
        self.resfriamento = val;
    }

    fn metropoles(&self, mut solution: Vec<Size>) -> (usize, Vec<Size>) {
        let mut rng = rand::thread_rng();
        let mut best_one = Scheduling::from(solution.clone()).get_end_time();
        for _ in 0..self.itermaxmetropoles {
            let testing_solution = gen_vec_n_shuffle(&solution, 5);
            let tes_val = Scheduling::from(testing_solution.clone()).get_end_time();
            let delta = tes_val as i32 - best_one as i32;
            if delta <= 0 {
                best_one = tes_val;
                solution = testing_solution;
            } else {
                let boltzman = std::f64::consts::E.powf(-delta as f64 / self.temperatura);
                if rng.gen::<f64>() < boltzman {
                    best_one = tes_val;
                    solution = testing_solution;
                }
            }
        }
        (best_one, solution)
    }

    pub fn run(&mut self) {
        let now = Instant::now();

        let mut best_solution_in = self.values.clone();
        best_solution_in.sort_by(|a, b| b.cmp(a));

        let initial_solution = Scheduling::from(best_solution_in.clone()).get_end_time();

        let temperatura_final: f64 = 1.0f64;

        let cooling_iterations = self.iterstocool;

        let mut best_global = initial_solution;
        let mut best_val;

        while self.temperatura >= temperatura_final {
            for _ in 0..cooling_iterations {
                (best_val, best_solution_in) = self.metropoles(best_solution_in);
                if best_val < best_global {
                    best_global = std::cmp::min(best_val, best_global);
                    println!("temperatura: {}", self.temperatura);
                    println!("valor: {best_global}");
                }
            }
            self.temperatura = self.temperatura * self.resfriamento;
        }
        let duration = now.elapsed();
        println!("Tempo {:?}", duration);

        println!("Final one: {}", best_global);
        println!("Final one: {:?}", best_solution_in);
        self.result = Some(best_solution_in);
    }
}
