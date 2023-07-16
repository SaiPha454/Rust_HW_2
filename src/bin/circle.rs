use clap::{Arg, App};

fn main() {
    
    let matches = App::new("circle")
            .version("0.1.0")
            .author("Sai Marn Pha")
            .about("This program receive radius input from user
                and calculate the area of a circle")
            .arg(
                Arg::with_name("radius")
                    .value_name("RADIUS")
                    .help("Enter a number for radius")
                    .required(true)
                    .short("r")
                    .index(1)
            ).get_matches();
    
    let radius = matches.value_of("radius").unwrap().to_string();
    let radius = radius.parse().unwrap_or(0.0);
    let pi = 3.14159;
    let circle_area = pi * radius * radius;

    println!("The area of the circle with radius = {radius} is : {circle_area}");
    
    //To run the programm, use :  cargo run 5  #5 is the value of radius
    //For using -r short form -r : cargo run -r 5 
}
