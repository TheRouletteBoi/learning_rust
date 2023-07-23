use std::{char, io};

/**
 * formula for temperature conversion
 * Celsius to Fahrenheit: F = (C * 9/5) + 32
 * Fahrenheit to Celsius: C = (F - 32) * 5/9
 *
 *
 * Known bugs:
 * There is a bug if you input "32 COFFEE" the program will continue flow without validation.
 */

fn celsius_to_fahrenheit(c: f32) -> f32 {
    if c == 0.0 {
        return 0.0;
    }

    let fahrenheit: f32 = (c * 9.0 / 5.0) + 32.0;

    return fahrenheit;
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    if f == 0.0 {
        return 0.0;
    }

    let celsius: f32 = (f - 32.0) * 5.0 / 9.0;

    return celsius;
}

fn is_input_fahrenheit(temperature: char) -> bool {
    if temperature == 'F' {
        return true;
    }

    return false;
}

fn is_input_celsius(temperature: char) -> bool {
    if temperature == 'C' {
        return true;
    }

    return false;
}

fn is_valid_format(uint_char: char) -> bool {
    match uint_char {
        'C' | 'F' => true,
        _ => false,
    }
}

fn show_result(input_temperature_value: f32, input_uint_char: char) {
    let is_fahrenheit: bool = is_input_fahrenheit(input_uint_char);

    let input_uint_name: &str;
    let conversion_uint: &str;
    let conversion_value: f32;

    if is_fahrenheit {
        input_uint_name = "Fahrenheit";
        conversion_uint = "Celsius";
        conversion_value = fahrenheit_to_celsius(input_temperature_value);
    } else {
        input_uint_name = "Celsius";
        conversion_uint = "Fahrenheit";
        conversion_value = celsius_to_fahrenheit(input_temperature_value);
    }

    println!(
        "{} degrees {} is equal to {} degrees {}",
        input_temperature_value, input_uint_name, conversion_value, conversion_uint
    );
}

fn main() {
    println!("Enter a temperature and uint (e.g., 23 C or 75 F): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim the new line character from the input
    input = input.trim().to_string();

    let input_split: Vec<&str> = input.split_whitespace().collect();

    if input_split.len() != 2 {
        println!("Invalid input format.");
        return;
    }

    let input_temperature_value: f32 = match input_split[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature value.");
            return;
        }
    };

    let input_uint_char: char = match input_split[1].chars().next() {
        Some(uint_char) => uint_char,
        None => {
            println!("Invalid uint.");
            return;
        }
    };

    if !is_valid_format(input_uint_char) {
        println!("Input uint format");
        return;
    }

    show_result(input_temperature_value, input_uint_char);
}
