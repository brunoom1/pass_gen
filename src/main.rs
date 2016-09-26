extern crate rand;

use rand::Rng;
use std::env;
use std::str::FromStr;

fn gen_password(size:u8){

	if size > 100 || size < 5{
		panic!("min size for password is 5 and max is 100");
	}

	let mut v:Vec<u8> = vec![0; 100];

	for i in &mut v{
		*i = rand::thread_rng().gen_range(33, 125);
	}

	let mut c:u8 = 0;

	for i in &v{

		if c == size{
			break;
		}

		print!("{}", *i as char);

		c += 1;
	}
}

fn main() {

	let mut c:env::Args = env::args();

	if c.len() == 2{

		let mut size:u8 = 6;


		match c.nth(1) {
		    Some(param) => { 
		    	println!("{:?}", param);
		    	match u8::from_str(&param) {
		    		Ok(s) => {
		    			size = s;
		    		},
		    		Err(_) => {
		    			println!("parameter is not int");
		    		}
		    	}
		    },
		    None => { println!("pass password size for params ex: cargo run 10") }
		}		


		if size > 100 || size < 5{
			println!("min size for password is 5 and max is 100");			
			std::process::exit(0);
		}

		print!("\n Your new password:  ");
		gen_password(size);
		print!("");
		println!("\n");
	}
}
