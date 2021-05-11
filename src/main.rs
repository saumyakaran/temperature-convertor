use std::io;

fn main() {
    let unit = get_unit();
    let temperature = get_temp();
    let (temperature, unit) = convertor(temperature, unit);

    println!("\n👉🏻 Result: {} °{}", temperature, unit);
}

fn get_unit() -> char {
    println!("Enter the temperature unit you want to convert from ('C' for celsius, 'F' for fahrenheit): ");

    loop {
        let mut unit_input = String::new();

        io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read line");
        
        if unit_input.trim().len() == 1 {
            match unit_input.chars().nth(0) {
                Some(value) => {
                    println!("value is {}", value);

                    if value == 'C' || value == 'F' {
                        return value;
                    } else {
                        println!("Please enter 'C' or 'F'!");
                    }
                },
                None => {
                    println!("Please enter 'C' or 'F'!");
                    continue;
                }
            };
        } else {
            println!("Please enter 'C' or 'F'!");
        }
    }  
}

fn get_temp() -> f64 {
    let mut temperature_input = String::new();

    println!("Enter the temperature: ");

    loop {
        io::stdin()
        .read_line(&mut temperature_input)
        .expect("Failed to read line");

        let temperature_input: f64 = match temperature_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        return temperature_input;
    }
}

fn convertor(temperature: f64, unit: char) -> (f64, char) {
    if unit == 'F' {
        let result = (temperature - 32.0) * 5.0 / 9.0;
        (result, 'C')
    } else if unit == 'C' {
        let result = (temperature * 9.0 / 5.0) + 32.0;
        (result, 'F')
    } else {
        panic!("Invalid Input: Expected 'C' for celsius or 'F' for farhenheit!")
    }
}