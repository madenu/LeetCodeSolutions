pub mod remove_duplicates_from_sorted_array;
pub mod remove_element;

use std::io;

// Print a menu of choices (include the quit option)
// Loop back around if input not recognized
// Allow user to select a choice from the menu
// Make a call to the backing function
// Continue or return to main menu

static CHOICES: &[&str;2] = &["Remove duplicates from sorted array (LeetCode 26)",
                              "Remove element (LeetCode 27)"];

fn main() {
    
    let mut choice;
    loop {
        print_menu();
        choice = get_input_choice(); 

        match choice {
            0 => {
                let mut input_vector = get_input_vector();
                let ans = remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut input_vector);
            },
            1 => {
                let mut input_vector = get_input_vector();
                // TODO Get element to remove
                let ans = remove_element::Solution::remove_element(&mut input_vector, 0); 
            },
            _ => {
                continue;
            }
        }
    }
}

fn print_menu() {
    println!("Please choose a LeetCode problem:");
    for (i, choice) in CHOICES.iter().enumerate() {
        println!("{i}. {choice}")
    }
}

fn get_input_choice() -> usize {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // TODO Check for index out of bounds
    let choice: usize = input.trim().parse().expect("Please type a number!");
    println!("You chose: {choice}. {}", CHOICES[choice]);
    return choice;
}

fn get_input_vector() -> Vec<i32> {
    return Vec::new();
}