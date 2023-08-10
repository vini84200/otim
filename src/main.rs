use std::io;

use crate::scheduling::Scheduling;

mod scheduling;

fn main() -> io::Result<()> {
    let mut file_lines = io::stdin().lines();
    let n: usize = file_lines
        .next()
        .expect("value of file")
        .unwrap()
        .parse()
        .unwrap();
    let values: Vec<i32> = file_lines
        .into_iter()
        .map(|v| v.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    let mut ordered = values.clone();
    ordered.sort_by(|a, b| b.cmp(a));
    let sched = Scheduling::from(ordered);

    println!("Valor Guloso: {}", sched.get_end_time());
    println!("Valor Máximo: {}", sched.get_trivial_time());
    println!("Verificacao: {}", sched.verify());
    Ok(())
}
