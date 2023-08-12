use std::{fs::read_to_string, io, path::PathBuf};

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

    pub(crate) fn from_file(file_path: PathBuf) -> InputParser {
        let file_lines = read_to_string(file_path).expect("Couldnt read the file");
        let mut file_lines = file_lines.lines();
        let _n: usize = file_lines.next().expect("value of file").parse().unwrap();
        let values: Vec<i32> = file_lines.map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
        Self { sizes: values }
        // todo!()
    }
}
