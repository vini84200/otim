use std::io;


fn main() -> io::Result<()> {
	let mut file_lines = io::stdin().lines();
	let n: usize = file_lines.next().expect("value of file").unwrap().parse().unwrap();
	let values: Vec<i32> = file_lines.into_iter()
		.map(|v| v.unwrap().parse().unwrap())
		.collect::<Vec<i32>>();

	//println!("n: {}\nvalues:{:?}",n, values);	

	let mut ordered = values.clone();
	ordered.sort_by(|a, b| b.cmp(a));
	let max_size = values.iter().sum::<i32>() as usize;
	let mut min_vec = vec![std::i32::MAX; max_size];

	//println!("max size: {}", max_size);

	'found:
	for value_in_iter in ordered.iter() {
		let size = *value_in_iter;

		let mut i = 0;
		'willy:
		while i < max_size {
			//println!("size: {} < min_vec[i]: {}", size, min_vec[i]);
			if size < min_vec[i] {
				//println!("Tentanto encaixar valor: {} no indice {} ", size, i);
				for i_seg in i..(i + size as usize){
					if size >= min_vec[i_seg]{
						i += i_seg;
						continue 'willy; 
					}
				}
				for s in 1..=size{
					min_vec[i] = s;
					i += 1;
				}
				//println!("{:?}", min_vec);
				continue 'found;
			}
			i += 1;
		}
	}
	println!("{:?}", min_vec);
	println!("O valor guloso mínimo é: {}", min_vec.iter().rev().take_while(|x| **x == std::i32::MAX).count());
	println!("o max size é: {}",max_size);
	Ok(())
}
