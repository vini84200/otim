use std::io;


fn main() -> io::Result<()> {
    println!("Hello, world!");
	let mut file_lines = io::stdin().lines();
	let n: i32 = file_lines.next().expect("value of file").unwrap().parse().unwrap();
	let values: Vec<i32> = file_lines.into_iter()
		.map(|v| v.unwrap().parse().unwrap())
		.collect::<Vec<i32>>();

	
	println!("n: {}\nvalues:{:?}",n, values);

	Ok(())
}
