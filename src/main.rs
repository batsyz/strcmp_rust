use std::env;

fn main () {
	println!("Compare 2 strings on the fly!");
	
	let args: Vec<String> = env::args().collect();

	let first_arg = &args[1];
	let second_arg = &args[2];

	println!("{}", first_arg.to_lowercase() == second_arg.to_lowercase());
}
