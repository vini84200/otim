pub type Size = i32;

pub struct SchedLean {
    collisions: Vec<Size>,
	max_size: usize,
	end_time: usize
}

impl SchedLean{
	pub fn new(max_size: usize) -> Self{
		SchedLean {
			collisions: vec![Size::MAX; max_size],
			max_size,
			end_time: max_size
		}
	}
	pub fn solve(&mut self, input: &Vec<i32>) -> usize {
		self.reset();
		input.iter().for_each(|v| self.add(*v as Size));
        self.end_time
	}
	fn reset(&mut self){
		self.end_time = 0;
		self.collisions.fill(Size::MAX);
	}

    fn insert_at(&mut self, pos: usize, size: Size) {
        for i in 0..size as usize {
            self.collisions[i + pos] = (i + 1) as Size 
        }
        let end_time = pos + size as usize;
        if end_time > self.end_time {
            self.end_time = end_time;
        }
    }

    fn collision_in_vec(&self, block: Size, i: usize) -> Option<usize> {
        if block < self.collisions[i] {
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

    pub fn add(&mut self, block: Size){
        let mut pos = 0;
        while pos < self.max_size {
            if let Some(next_pos) = self.collision_in_vec(block, pos) {
                pos = next_pos;
            } else {
                return self.insert_at(pos, block);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
	use crate::scheduling::Scheduling;

    #[test]
    fn trivial() {
		let input = vec![3, 5];
		let max_size = input.iter().sum::<i32>() as usize;
		let mut solver = SchedLean::new(max_size);
		let res = solver.solve(&input);
        let solution = Scheduling::from(input);

		assert_eq!(max_size, solution.get_trivial_time());
        assert_eq!(res, solution.get_end_time());
    }

	/*
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
    } */
}
