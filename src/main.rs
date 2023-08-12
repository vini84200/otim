use std::time::Instant;
use rand::Rng;

use crate::{input_parser::InputParser, scheduling::Scheduling};

//use crate::scheduling_lean::SchedLean;

mod input_parser;
mod scheduling;
//mod scheduling_lean;

const ITERSTOCOOL: usize = 100; 
const ITERMAXMETROPOLES: usize = 100; 

fn gen_vec_n_shuffle(v_in: &Vec<i32>, n: usize) -> Vec<i32>{
	let mut v = v_in.clone();
	let mut rng = rand::thread_rng();
	let last_indice = v.len() - 1;
	for offset in 0..n{
		let indice = rng.gen_range(0..=last_indice);
		v.swap(indice, last_indice - offset);
	}
	v
}

fn metropoles(temperatura: f64, mut solution: Vec<i32>) -> (usize, Vec<i32>){
	let mut rng = rand::thread_rng();
    let mut best_one = Scheduling::from(solution.clone()).get_end_time();
	for _ in 0..ITERMAXMETROPOLES{
		let testing_solution = gen_vec_n_shuffle(&solution, 5);
        let tes_val = Scheduling::from(testing_solution.clone()).get_end_time();
		let delta = tes_val as i32 - best_one as i32;
		if delta <= 0 {
			best_one = tes_val;
			solution = testing_solution;
		}else{
			let boltzman = std::f64::consts::E.powf(-delta as f64/temperatura);
			if rng.gen::<f64>() < boltzman {
				best_one = tes_val;
				solution = testing_solution;
			}
		}
	}
	(best_one, solution)
}

fn main(){
    let parser = InputParser::from_stdin();
    let values = parser.get_values();

	//algoritmo de metrópolis de metrópolis em ordem aleatória - inerente a vizinhança
	//processa os s' vizinhos de s
	//se f(s') <= f(s) passa para s'
	//se não usa a probabilidade de boltman que cresce com a temperatura e decresce com f(s') - f(s)

	let now = Instant::now();

    let mut best_solution_in = values.clone();
    best_solution_in.sort_by(|a, b| b.cmp(a));

	let max_solution: i32 = values.iter().sum();
	let initial_solution = Scheduling::from(best_solution_in.clone()).get_end_time();

	let temperatura_inicial: f64 = max_solution as f64 - initial_solution as f64;
	let temperatura_final: f64 = 1.0f64;

	let mut temperatura: f64 = temperatura_inicial;
	let resfriamento: f64 = 0.85f64;
	let cooling_iterations = ITERSTOCOOL;

	let mut best_global = initial_solution;
	let mut best_val;

	while temperatura >= temperatura_final {
		for _ in 0..cooling_iterations{
			(best_val, best_solution_in) = metropoles(temperatura, best_solution_in);
			if best_val < best_global {
				best_global = std::cmp::min(best_val, best_global);
				println!("temperatura: {}", temperatura);
				println!("valor: {best_global}");
			}
		}
		temperatura = temperatura * resfriamento;
	}
    let duration = now.elapsed();
    println!("Tempo {:?}", duration);

    println!("Final one: {}", best_global);
    println!("Final one: {:?}", best_solution_in);
}
