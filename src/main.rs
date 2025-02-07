mod temperature_logic;

use std::io;

use temperature_logic::{celsius_to_fahrenheit, fahrenheit_to_celsius, kelvin_to_celsius, celsius_to_kelvin, kelvin_to_fahrenheit, fahrenheit_to_kelvin};

fn main() {
    println!("====== Welcome to the temperature converter! ======");

    loop {
        println!("which conversion would you like to do?");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Kelvin to Celsius");
        println!("4. Celsius to Kelvin");
        println!("5. Kelvin to Fahrenheit");
        println!("6. Fahrenheit to Kelvin");
        println!("7. Exit");
        println!("=====================================");
        
        let mut choice = String::new();
        match io::stdin().read_line(&mut choice){
            Ok(num) => num,
            Err(_) => {
                println!("=====================================");
                println!("Please type a number");
                println!("=====================================");
                continue;
            }
        };

    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("=====================================");
            println!("Please type a number");
            println!("=====================================");
            continue;
        }
    };

    match choice {
        1 => {
            println!("=====================================");
            println!("You have chosen to convert Celsius to Fahrenheit");
            celsius_to_fahrenheit();
            println!("=====================================");
        },
        2 => {
            println!("=====================================");
            println!("You have chosen to convert Fahrenheit to Celsius");
            fahrenheit_to_celsius();
            println!("=====================================");
        },
        3 => {
            println!("=====================================");
            println!("You have chosen to convert Kelvin to Celsius");
            kelvin_to_celsius();
            println!("=====================================");
        },
        4 => {
            println!("=====================================");
            println!("You have chosen to convert Celsius to Kelvin");
            celsius_to_kelvin();
            println!("=====================================");
        },
        5 => {
            println!("=====================================");
            println!("You have chosen to convert Kelvin to Fahrenheit");
            kelvin_to_fahrenheit();
            println!("=====================================");
        },
        6 => {
            println!("=====================================");
            println!("You have chosen to convert Fahrenheit to Kelvin");
            fahrenheit_to_kelvin();
            println!("=====================================");
        },
        7 => {
            println!("=====================================");
            println!("Exiting the program");
            println!("=====================================");
            break;
        },
        _ => {
            println!("=====================================");
            println!("Invalid choice");
            println!("=====================================");
        }
    }

    
}



}
