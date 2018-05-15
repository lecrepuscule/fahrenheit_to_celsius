use std::io;

fn main() {

	loop{

    	println!("Enter the temperature in celcius.");

    	let mut f_temp = String::new();
    	io::stdin().read_line(&mut f_temp)
    		.expect("Failed to read line");

    	let f_temp: f64 = match f_temp.trim().parse() {
    		Ok(num) => num,
    		Err(_) => continue,
    	};

    	println!("The temperature in celsius is: {}", f_to_c(f_temp));
    	break;
	};

   	
}

fn f_to_c(temperature: f64) -> f64 {
	(temperature - 32.0) * 5.0 / 9.0
}
