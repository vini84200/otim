use std::{io, time::Instant};
use rand::Rng;

use crate::{input_parser::InputParser, scheduling::Scheduling};

use crate::scheduling_lean::SchedLean;

mod input_parser;
mod scheduling;
mod scheduling_lean;

const ITERMAX: usize = 1000000; 

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

fn solve_with_schedlean(mut best_solution_in: Vec<i32>){
	let max_val: usize = best_solution_in.iter().sum::<i32>() as usize;
	let mut solver = SchedLean::new(max_val);
	
	let mut best_one = solver.solve(&best_solution_in);
	
    let now = Instant::now();
	for _ in 0..ITERMAX{
		let testing_solution = gen_vec_n_shuffle(&best_solution_in, 5);
        let tes_val = solver.solve(&testing_solution);
		if tes_val < best_one {
			best_one = tes_val;
			best_solution_in = testing_solution;
		}
	}

	let verifier = Scheduling::from(best_solution_in.clone());
	if !verifier.verify(){
		panic!("Valor encontrado não é válido")
	}

    let duration = now.elapsed();
    println!("Tempo {:?}", duration);

    //println!("Final one: {}", best_one);

    let duration = now.elapsed();
    println!("Tempo {:?}", duration);

    println!("Value: {}", best_one);
    println!("Final one: {:?}", best_solution_in);
}

fn solve_with_scheduler(mut best_solution_in: Vec<i32>){
    let mut best_one = Scheduling::from(best_solution_in.clone());
	
    let now = Instant::now();
	for _ in 0..ITERMAX{
		let testing_solution = gen_vec_n_shuffle(&best_solution_in, 5);
        let tes_val = Scheduling::from(testing_solution.clone());
		if tes_val.get_end_time() < best_one.get_end_time() {
			best_one = tes_val;
			best_solution_in = testing_solution;
		}
	}

	if !best_one.verify(){
		panic!("Valor encontrado não é válido")
	}

    let duration = now.elapsed();
    println!("Tempo {:?}", duration);

    println!("Final one: {}", best_one.get_end_time());
    println!("Final one: {:?}", best_solution_in);
}

fn main() -> io::Result<()> {
    let parser = InputParser::from_stdin();
    let values = parser.get_values();

    let mut best_solution_in = values.clone();
    best_solution_in.sort_by(|a, b| b.cmp(a));
	solve_with_schedlean(best_solution_in.clone());
	solve_with_scheduler(best_solution_in);
    Ok(())
}
