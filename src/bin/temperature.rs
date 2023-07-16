use clap::{Arg, App};

fn main() {
    
    let matches = App::new("temperature")
            .version("0.1.0")
            .author("Sai Marn Pha")
            .about("This program receive temperature in Celsius or Fahrenheit from user
                and convert to their respective opposite scale")
            .arg(
                Arg::with_name("fah")
                    .value_name("Fahrenheit")
                    .help("Enter temperature in Fahrenheit")
                    .required(false)
                    .short("f")
                    .validator(validate_number)
            ).arg(
                Arg::with_name("cel")
                    .value_name("Celsius")
                    .help("Enter temperature in Celsius")
                    .required(false)
                    .short("c")
                    .validator(validate_number)
            ).get_matches();
    
    let fah: f32 = matches.value_of("fah")
                    .unwrap_or("0.0").to_string().parse().unwrap();
    let cel : f32 = matches.value_of("cel")
                    .unwrap_or("0.0").to_string().parse().unwrap();
    
    if matches.is_present("fah"){
        let tem_in_celsius = (5.0/9.0)*(fah-32.0);
        println!("Temperature of {fah} Fahrenheit  is equal to {tem_in_celsius} in Degree Celsius");
    }else if matches.is_present("cel") {
        let tem_in_fahrenheit = ((9.0 * cel)/5.0) + 32.0;
        println!("Temperature of {cel} Degree Celsius is equal to {tem_in_fahrenheit} in Fahrenheit");
    }
    // To convert Fah to Celsius, run => cargo run --bin temperature -- -f < a number >
    // To convert Celsisu to Fah, run => cargo run --bin temperature -- -c < a number >
    // For negative number inputs,
    // use = instead; Example : cargo run --bin temperature -- -c=-10
    
}

// Function for validating the arguments passed in the command line
fn validate_number(value: String) -> Result<(), String> {
    match value.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(" Must be only numbers!")),
    }
}
