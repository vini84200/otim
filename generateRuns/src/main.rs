
// Runs are generated from a set of parameters
// Example of a generated run:
// pueue add -g otim -- './target/release/otim -i trsp/trsp_1000_4.dat sa  --itermaxmetropoles 50'

use std::fmt::Display;

fn main() {
    let ns = vec![50, 100, 1000];
    let ms = vec![1, 2, 3, 4];

    let itermaxmetropoles = vec![100];
    let resfriamento = vec![0.85, 0.95];
    let iterstocool = vec![100];
    let temperature = vec![500., 500., 500., 750., 1000.];

    for n in ns {
        for m in &ms {
            for itermaxmetropoles in &itermaxmetropoles {
                for resfriamento in &resfriamento {
                    for iterstocool in &iterstocool {
                        for t in &temperature {
                            let run = Run::new(n, *m, *itermaxmetropoles, *resfriamento, *iterstocool, *t);
                            println!("{}", run);
                        }
                    }
                }
            }
        }
    }

}

struct Run {
    n: usize,
    m: usize,
    itermaxmetropoles: usize,
    resfriamento: f64,
    iterstocool: usize,
    temperature: f64,
}

impl Run {
    fn new(n: usize, m: usize, itermaxmetropoles: usize, resfriamento: f64, iterstocool: usize, temperature: f64) -> Self {
        Self {
            n,
            m,
            itermaxmetropoles,
            resfriamento,
            iterstocool,
            temperature,
        }
    }
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pueue add -g otim -- './target/release/otim -i trsp/trsp_{}_{}.dat sa --itermaxmetropoles {} --resfriamento {} --iterstocool {} -t {}'", self.n, self.m, self.itermaxmetropoles, self.resfriamento, self.iterstocool, self.temperature)
    }
}
