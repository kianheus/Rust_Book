use std::io;

fn main() {
    println!("-------------------------------------------");
    println!("Welcome to the great temperature converter!");
    println!("-------------------------------------------\n");

    println!("Would you like to convert from Celsius to Fahrenheit?");
    println!("Or from Fahrenheit to Celsius?");

    let conv_choice: u32 = loop {
        println!("Press 1 for C -> F, press 2 for F -> C.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => break 1,
            "2" => break 2,
            _ => println!("Invalid input, please enter 1 or 2.")
        };
    };

    let (input_type, output_type): (&str, &str) = if conv_choice == 1 {
        ("Celsius", "Fahrenheit")
    } else {
        ("Fahrenheit", "Celsius")
    };
        
    println!("Converting {input_type} to {output_type}");

    

    let input_temp: f64 = loop{
        let mut input_temp_string = String::new();
        println!("What temperature value would you like to convert?");

        io::stdin()
            .read_line(&mut input_temp_string)
            .expect("Failed to read line");

            match input_temp_string.trim().parse::<f64>(){
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input");
                continue}
        };
    };

    let output_temp: f64;

    if conv_choice == 2 {
        output_temp = input_temp * 9. / 5. + 32.;
    } else {
        output_temp = (input_temp - 32.) * 5. / 9.;
    }

    println!("{input_temp} degrees {input_type} equals {output_temp} degrees {output_type}.");
    
}
