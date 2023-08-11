use std::io;

use crate::scheduling::Size;

#[derive(Debug)]
pub struct InputParser {
    sizes: Vec<Size>,
}

impl InputParser {
    pub fn from_stdin() -> Self {
        let mut file_lines = io::stdin().lines();
        let _n: usize = file_lines
            .next()
            .expect("value of file")
            .unwrap()
            .parse()
            .unwrap();
        let values: Vec<i32> = file_lines
            .map(|v| v.unwrap().parse().unwrap())
            .collect::<Vec<i32>>();
        Self { sizes: values }
    }

    pub fn get_values(self) -> Vec<Size> {
        self.sizes
    }
}
