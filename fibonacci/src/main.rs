use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		println!("{:?}",args);
		process::exit(1);
	}

	let value = &args[1];

	let n: i32 = value.parse().unwrap();

	println!("{:}",fibonacci(n));
}

fn fibonacci(n :i32) -> i32 {
	if n < 2 {
		return n
	}

	fibonacci(n - 1) + fibonacci(n - 2)
}
