/*
Task: Grade Classifier

Write a program that takes a student's score as input and classifies it into a grade according to the following scale:

90 to 100: A
80 to 89: B
70 to 79: C
60 to 69: D
0 to 59: F
Requirements:

Ask the user to enter their score as an integer.
Use a match expression to classify the score into a grade.
Display the grade to the user.
Hints:

You can use the read_line() function to read the user input as a String and then convert it to an integer using parse().
Example Output:

Enter your score: 85
Your grade is B.

Enter your score: 62
Your grade is D.

Enter your score: 95
Your grade is A.
Try to implement the Grade Classifier using the match syntax, and handle all the grade ranges correctly. You can also add error handling in case the user enters an invalid score. Happy coding! If you have any questions or need further guidance, feel free to ask.
*/

use std::io::{self, Write};

fn find_banna_show_result() {
    let fruit = "banana";
    match fruit {
        "apple" => println!("It's an apple!"),
        "banana" => println!("It's a banana!"),
        "orange" => println!("It's an orange!"),
        _ => println!("I don't know what fruit it is."),
    }
}

fn find_banna_return_string() {
    let fruit = "bannana";

    let result: &str = match fruit {
        "bannana" => "It's a bannana",
        "apple" => "It's an apple",
        "orange" => "It's an orange",
        _ => "You found a fruit not on the list",
    };

    println!("you found a {}", result);
}

fn main() {
    print!("Enter your score: ");
    io::stdout().flush();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    let grade_int = input.parse::<i32>();

    // unwrap() will remove ParseIntError from parse()
    let grade: &str = match grade_int.unwrap() {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid grade number",
    };

    println!("Your grade is a {}", grade);
}
