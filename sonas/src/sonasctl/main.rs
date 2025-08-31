use sonas::server;
use std::env;

fn main() {
	let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
	println!("{:?}", args);
	let result = server::send_line(&args);

	match result {
		Ok(x) => println!("{:?}", x),
		Err(e) => println!("{:?}", e),
	}
}
