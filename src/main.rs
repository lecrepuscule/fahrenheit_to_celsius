use std::io;

fn main() {

	loop{

		println!("Enter the temperature.");

    	let mut f_temp = String::new();
    	io::stdin().read_line(&mut f_temp)
    		.expect("Failed to read line");

    	let f_temp: f64 = match f_temp.trim().parse() {
    		Ok(num) => num,
    		Err(_) => continue,
    	};

    	println!("Enter the unit, 'F' or 'C'.");

		let mut unit = String::new();
		io::stdin().read_line(&mut unit)
			.expect("Failed to read line");

		let unit = unit.trim();

		if unit == "F" {
			println!("The temperature in {} is: {}", unit, f_to_c(f_temp));
		} else if unit == "C" {
			println!("The temperature in {} is: {}", unit, c_to_f(f_temp));
		} else {
			continue
		};

    	break;
	};

   	
}

fn f_to_c(temperature: f64) -> f64 {
	(temperature - 32.0) * 5.0 / 9.0
}

fn c_to_f(temperature: f64) -> f64 {
	temperature * 9.0 / 5.0 + 32.0
}
