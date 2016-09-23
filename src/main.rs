extern crate rand;

use rand::Rng;

fn main() {

	let mut v:[u8;10] = [0;10];

	for i in &mut v{
		*i = rand::thread_rng().gen_range(33, 125);
	}

	println!("\n\nYour new password");

	for i in &v{
		let c:char = *i as char;
		print!("{}", c);
	}

	println!("\n\n");

}
