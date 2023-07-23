use std::io;

/*
Task: Palindrome Checker

Write a program that checks if a given word is a palindrome.
A palindrome is a word that reads the same backward as forward.
For example, "radar" and "level" are palindromes.
*/

fn validate_input(input: &str) -> String {
    return input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .collect();
}

fn is_palindrome(input: &str) -> bool {
    let reverse_input: String = input.chars().rev().collect();

    return reverse_input == input;
}

fn show_result(input: String) {
    if is_palindrome(input.as_str()) {
        println!("{} is a palindrome.", input);
    } else {
        println!("{} is not a palindrome.", input);
    }
}

fn main() {
    println!("Enter a word: ");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // remove new line character
    input = input.trim().to_string();

    let valid_input: String = validate_input(input.as_str());

    show_result(valid_input);
}
