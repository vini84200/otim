use simulated_annealing::SimulatedAnnealing;

use crate::{input_parser::InputParser, scheduling::Scheduling};

//use crate::scheduling_lean::SchedLean;

mod input_parser;
mod scheduling;
mod simulated_annealing;

fn main() {
    let parser = InputParser::from_stdin();
    let values = parser.get_values();
    let mut sim_anealing = SimulatedAnnealing::new(values, 200f64);
    sim_anealing.run();

    //algoritmo de metrópolis de metrópolis em ordem aleatória - inerente a vizinhança
    //processa os s' vizinhos de s
    //se f(s') <= f(s) passa para s'
    //se não usa a probabilidade de boltman que cresce com a temperatura e decresce com f(s') - f(s)
}
