use std::fmt::Display;

pub type Size = i32;

#[derive(Debug)]
pub struct Scheduling {
    collisions: Vec<Size>,
    end_time: usize,
    max_size: usize,
    solution_vec: Vec<(usize, Size)>,
}

impl Scheduling {
    pub fn new(max_size: usize, entries: usize) -> Scheduling {
        Scheduling {
            collisions: vec![Size::MAX; max_size],
            solution_vec: Vec::with_capacity(entries),
            end_time: 0,
            max_size,
        }
    }

    pub fn verify(&self) -> bool {
        for (start_block, size_first) in &self.solution_vec {
            let mut found_eq = false;
            for (start_other, size_other) in &self.solution_vec {
                if start_other == start_block && size_other == size_first && !found_eq {
                    found_eq = true;
                    continue;
                }
                let distance = start_other.abs_diff(*start_block) as i32;
                let min_size = size_first.min(size_other);

                let is_valid = distance >= *min_size;

                if !is_valid {
                    return false;
                }
            }
        }

        true
    }

    fn insert_at(&mut self, pos: usize, size: Size) {
        for i in 0..size as usize {
            self.collisions[i + pos] = (i + 1) as i32
        }
        self.solution_vec.push((pos, size));
        let end_time = pos + size as usize;
        if end_time > self.end_time {
            self.end_time = end_time;
        }
    }

    fn collision_in_vec(&self, block: Size, i: usize) -> Option<usize> {
        //se o tam bloco é menor que o elemento no vetor de indice i
        if block < self.collisions[i] {
            //calcula se pode colocar os demais valores
            for i_seg in (i + 1)..(i + block as usize) {
                if block >= self.collisions[i_seg] {
                    return Some(i_seg + 1);
                }
            }
        } else {
            return Some(i + 1);
        }
        None
    }

    pub fn add(&mut self, block: &Size) -> Option<usize> {
        let mut pos = 0;
        while pos < self.max_size {
            if let Some(next_pos) = self.collision_in_vec(*block, pos) {
                pos = next_pos;
            } else {
                self.insert_at(pos, *block);
                return Some(pos);
            }
        }
        None
    }

    pub fn get_end_time(&self) -> usize {
        self.end_time
    }

    pub fn get_trivial_time(&self) -> usize {
        self.max_size
    }
	pub fn get_solution_vec(&self) -> Vec<(usize, Size)>{
		self.solution_vec.to_owned()
	}

    pub fn get_sol(&self) -> String {
        let mut s = String::new();
        s.push_str("\n Solução ");
        s.push_str(&self.get_end_time().to_string());
        s.push_str(" = [\n");
        s.push_str("pi \t si\n");
        s.push_str("==========\n");
        for (start, duration) in &self.solution_vec {
            s.push_str(&duration.to_string());
            s.push_str(" \t ");
            s.push_str(&start.to_string());
            s.push_str("\n");
        }
        s.push_str("]");
        s
    }
}

impl From<Vec<Size>> for Scheduling {
    fn from(value: Vec<Size>) -> Self {
        let max_size = value.iter().sum::<i32>() as usize;
		let entries = value.len();
        let mut s = Scheduling::new(max_size, entries);
        value.iter().for_each(|v| _ = s.add(v));
        s
    }
}

impl From<Vec<&Size>> for Scheduling {
    fn from(value: Vec<&Size>) -> Self {
        let max_size = value.iter().fold(0, |acc, v| acc + *v) as usize;
		let entries = value.len();
        let mut s = Scheduling::new(max_size, entries);
        value.iter().for_each(|v| _ = s.add(v));
        s
    }
}

impl Display for Scheduling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_size = self.get_end_time();
        write!(f, "\n Solução {} = [\n", line_size)?;
        for _ in 0..=line_size {
            write!(f, "=")?;
        }
        writeln!(f)?;
        for (start, duration) in &self.solution_vec {
            // let end = start.to_owned() as i32 + duration;
            for _ in 0..start.to_owned() {
                write!(f, " ")?;
            }
            for _ in 0..duration.to_owned() {
                write!(f, "#")?;
            }
            writeln!(f)?;
        }

        writeln!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let s = Scheduling::new(10,0);
        assert!(s.verify());
        assert_eq!(s.end_time, 0);
    }

    #[test]
    fn finds_not_valid() {
        let mut s = Scheduling::new(10, 2);
        s.insert_at(3, 2);
        s.insert_at(4, 5);
        assert!(!s.verify());
        assert_eq!(s.end_time, 9);
    }

    #[test]
    fn trivial() {
        let mut s = Scheduling::new(10, 2);
        s.insert_at(0, 3);
        s.insert_at(3, 5);
        assert!(s.verify());
    }

    #[test]
    fn finds_simple() {
        let mut values = vec![1, 1, 1, 2, 3, 5, 8];
        values.sort_by(|a, b| b.cmp(a));
        let sched = Scheduling::from(values);
        assert_eq!(sched.end_time, 11);
        assert!(sched.verify());
		let mut solution_vec = sched.get_solution_vec();
		solution_vec.sort_by(|(a, _), (b, _)| a.cmp(b));
        let mut allocation_truth = Vec::new();
        allocation_truth.push((0, 8));
        allocation_truth.push((1, 1));
        allocation_truth.push((2, 2));
        allocation_truth.push((3, 1));
        allocation_truth.push((4, 1));
        allocation_truth.push((5, 5));
        allocation_truth.push((8, 3));

        assert_eq!(solution_vec, allocation_truth)
    }

    #[test]
    fn test_print_sched() {
        let mut s = Scheduling::new(10, 2);
        s.insert_at(3, 2);
        s.insert_at(4, 5);

        let expected = "\n Solução 9 = [\n\
                        pi \t si\n\
                        ==========\n\
                        2 \t 3\n\
                        5 \t 4\n\
                        ]";
        assert_eq!(s.get_sol(), expected.to_string());

    }

    #[test]
    fn test_print_sched2() {
        let mut s = Scheduling::new(10, 2);
        s.insert_at(1, 2);
        s.insert_at(3, 2);

        let expected = "\n Solução 5 = [\n\
                        pi \t si\n\
                        ==========\n\
                        2 \t 1\n\
                        2 \t 3\n\
                        ]";
        assert_eq!(s.get_sol(), expected.to_string());

    }
}
