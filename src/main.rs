use itertools::Itertools;
use std::{io, time::Instant};

use crate::{input_parser::InputParser, scheduling::Scheduling};

mod input_parser;
mod scheduling;

fn main() -> io::Result<()> {
    let parser = InputParser::from_stdin();
    let values = parser.get_values();
    let mut ordered = values.clone();
    let now = Instant::now();
    ordered.sort_by(|a, b| b.cmp(a));
    let sched = Scheduling::from(ordered);

    println!("Valor Guloso: {}", sched.get_end_time());
    println!("Valor Máximo: {}", sched.get_trivial_time());
    println!("Verificacao: {}", sched.verify());
    let mut best_one = sched;
    let mut count: usize = 0;

    println!("Testando permutações:");
    for perm in values.iter().permutations(values.len()).unique() {
        // println!("{:?}", perm);
        let sched = Scheduling::from(perm.to_owned());
        if sched.verify() {
            if sched.get_end_time() < best_one.get_end_time() {
                println!("NOVA Solução {} na iter {count}", sched.get_end_time());
                best_one = sched;
            }
        } else {
            panic!("Should be valid!! {:?}", sched);
        }
        if count % 100000 == 0 {
            println!("Iter. {}", count);
            if count % 1000000 == 0 {
                println!("Best: {}", best_one);
            }
        }

        count += 1;
    }
    let duration = now.elapsed();

    println!("Final one: {}", best_one);
    println!("Total de iter. {count}");
    println!("Tempo {:?}", duration);
    println!("Value: {}", best_one.get_end_time());
    Ok(())
}
