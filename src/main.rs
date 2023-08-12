use std::path::PathBuf;

use clap::{Parser, Subcommand};
use simulated_annealing::SimulatedAnnealing;

use crate::{input_parser::InputParser, scheduling::Scheduling};

//use crate::scheduling_lean::SchedLean;

mod input_parser;
mod scheduling;
mod simulated_annealing;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "Input File")]
    input_file: Option<PathBuf>,

    #[arg(short, long, value_name = "Outpuf FILE")]
    output_file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    SimulatedAnnealing {
        #[arg(short, long, value_name = "Temperatura", default_value_t = 200f64)]
        temperatura_inicial: f64,

        #[arg(short, long, value_name = "Resfriamente", default_value_t = 0.85f64)]
        resfriamento: f64,

        #[arg(long, default_value_t = 100)]
        iterstocool: usize,
        #[arg(long, default_value_t = 100)]
        itermaxmetropoles: usize,
    },
    Highs,
}

fn main() {
    let cli = Cli::parse();
    let parser = if let Some(file_path) = cli.input_file {
        InputParser::from_file(file_path)
    } else {
        InputParser::from_stdin()
    };
    // let parser = InputParser::from_stdin();
    let values = parser.get_values();
    match &cli.command {
        Commands::SimulatedAnnealing {
            temperatura_inicial,
            resfriamento,
            itermaxmetropoles,
            iterstocool,
        } => {
            let mut sim_anealing = SimulatedAnnealing::new(values, *temperatura_inicial);
            sim_anealing.set_resfriamento(*resfriamento);
            sim_anealing.set_iterstocool(*iterstocool);
            sim_anealing.set_itermaxmetropoles(*itermaxmetropoles);
            sim_anealing.run();
        }
        Commands::Highs => todo!(),
    }

    //algoritmo de metrópolis de metrópolis em ordem aleatória - inerente a vizinhança
    //processa os s' vizinhos de s
    //se f(s') <= f(s) passa para s'
    //se não usa a probabilidade de boltman que cresce com a temperatura e decresce com f(s') - f(s)
}
