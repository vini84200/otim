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
	for value_in_iter in ordered.iter() {
		let size = *value_in_iter;

		let mut i = 0;
		'nextv:
		while i < max_size {
			if let Some(offset) = collision_in_vec(size, i, &mut min_vec){
				i = offset;
			}else{
				for s in 1..=size{
					min_vec[i] = s;
					i += 1;
				}
				break 'nextv;
			};
		}
	}
	/* 
	print!("[");
	min_vec.iter()
		.filter(|x| **x != std::i32::MAX)
		.for_each(|v| print!("{}, ", v));
	println!("]");
	*/

	println!("Valor Guloso: {}", min_vec.iter().filter(|x| **x != std::i32::MAX).count());
	println!("Valor Máximo: {}", max_size);
	Ok(())
}


fn collision_in_vec(block: i32, i: usize, min_vec: &mut Vec<i32>) -> Option<usize> {
	//se o tam bloco é menor que o elemento no vetor de indice i
	if block < min_vec[i] {
		//calcula se pode colocar os demais valores
		for i_seg in (i + 1)..(i + block as usize){
			if block >= min_vec[i_seg]{
				return Some(i_seg + 1)
			}
		}
	}else{
		return Some(i + 1)
	}
	None
}