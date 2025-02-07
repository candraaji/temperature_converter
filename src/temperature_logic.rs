
use std::io;

pub fn celsius_to_fahrenheit() {
    println!("Enter the temperature in Celsius: ");

    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return celsius_to_fahrenheit();
        }
    };

    let fahrenheit = celsius * 1.8 + 32.0;
    println!(" ");
    println!("The temperature in Fahrenheit is: {}", fahrenheit);
    println!(" ");
}

pub fn fahrenheit_to_celsius(){
    println!("Enter the temperature in Fahrenheit: ");

    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return fahrenheit_to_celsius();
        }
    };

    let celsius = (fahrenheit - 32.0) / 1.8;
    println!(" ");
    println!("The temperature in Celsius is: {}", celsius);
    println!(" ");

}

pub fn kelvin_to_celsius(){
    println!("Enter the temperature in Kelvin: ");

    let mut kelvin = String::new();
    io::stdin().read_line(&mut kelvin).expect("Failed to read line");

    let kelvin: f64 = match kelvin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return kelvin_to_celsius();
        }
    };

    let celsius = kelvin - 273.15;
    println!(" ");
    println!("The temperature in Celsius is: {}", celsius);
    println!(" ");
}

pub fn celsius_to_kelvin(){
    println!("Enter the temperature in Celsius: ");

    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read line");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return celsius_to_kelvin();
        }
    };

    let kelvin = celsius + 273.15;
    println!(" ");
    println!("The temperature in Kelvin is: {}", kelvin);
    println!(" ");
}

pub fn kelvin_to_fahrenheit(){
    println!("Enter the temperature in Kelvin: ");

    let mut kelvin = String::new();
    io::stdin().read_line(&mut kelvin).expect("Failed to read line");

    let kelvin: f64 = match kelvin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return kelvin_to_fahrenheit();
        }
    };

    let fahrenheit = kelvin * 1.8 - 459.67;
    println!(" ");
    println!("The temperature in Fahrenheit is: {}", fahrenheit);
    println!(" ");

}

pub fn fahrenheit_to_kelvin(){
    println!("Enter the temperature in Fahrenheit: ");

    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            return fahrenheit_to_kelvin();
        }
    };

    let kelvin = (fahrenheit + 459.67) / 1.8;
    println!(" ");
    println!("The temperature in Kelvin is: {}", kelvin);
    println!(" ");

}