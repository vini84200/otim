use std::{collections::BTreeMap, fmt::Display};

pub type Size = i32;

#[derive(Debug)]
pub struct Scheduling {
    collisions: Vec<Size>,
    end_time: usize,
    max_size: usize,
    allocations: BTreeMap<usize, Size>,
}

impl Scheduling {
    pub fn new(max_size: usize) -> Scheduling {
        Scheduling {
            collisions: vec![Size::MAX; max_size],
            allocations: BTreeMap::new(),
            end_time: 0,
            max_size,
        }
    }

    pub fn verify(&self) -> bool {
        for (start_block, size_first) in &self.allocations {
            let mut found_eq = false;
            for (start_other, size_other) in &self.allocations {
                if start_other == start_block && size_other == size_first && !found_eq {
                    found_eq = true;
                    continue;
                }
                let distance = start_other.abs_diff(start_block.to_owned()) as i32;
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
        if self.allocations.contains_key(&pos) {
            panic!("Tried to add to a position that already contains a task!");
        }
        for i in 0..size as usize {
            self.collisions[i + pos] = (i + 1) as i32
        }
        self.allocations.insert(pos, size);
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
            if let Some(next_pos) = self.collision_in_vec(block.to_owned(), pos) {
                pos = next_pos;
            } else {
                self.insert_at(pos, block.to_owned());
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
}

impl From<Vec<Size>> for Scheduling {
    fn from(value: Vec<Size>) -> Self {
        let max_size = value.iter().sum::<i32>() as usize;
        let mut s = Scheduling::new(max_size);
        value.iter().for_each(|v| {
            s.add(v);
        });

        s
    }
}

impl From<Vec<&Size>> for Scheduling {
    fn from(value: Vec<&Size>) -> Self {
        let max_size: i32 = value.iter().map(|a| a.to_owned()).sum();
        let mut s = Scheduling::new(max_size as usize);
        value.iter().for_each(|v| {
            s.add(v);
        });

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
        for (start, duration) in &self.allocations {
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
        let s = Scheduling::new(10);
        assert!(s.verify());
        assert_eq!(s.end_time, 0);
    }

    #[test]
    fn finds_not_valid() {
        let mut s = Scheduling::new(10);
        s.insert_at(3, 2);
        s.insert_at(4, 5);
        assert!(!s.verify());
        assert_eq!(s.end_time, 9);
    }

    #[test]
    fn trivial() {
        let mut s = Scheduling::new(10);
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
        let mut allocation_truth = BTreeMap::new();
        allocation_truth.insert(0, 8);
        allocation_truth.insert(1, 1);
        allocation_truth.insert(2, 2);
        allocation_truth.insert(3, 1);
        allocation_truth.insert(4, 1);
        allocation_truth.insert(5, 5);
        allocation_truth.insert(8, 3);
        assert_eq!(sched.allocations, allocation_truth)
    }
}
