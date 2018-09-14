mod calc;
use calc::time_dilation_module;
use std::io;
#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    print_main_menu();
}

fn print_main_menu() {
    println!("********** Rust - Time Dilation Calculator *************");
    println!("~ © Riedl Kevin, 2018 ~");
    println!("\nWelcome and thank you for testing/using my time dilation calculator.\n\
        As far as I know, it's the first one which has been written in Rust!");

    println!("Input proper time and relative velocity separated by a space: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if is_input_valid(&input) {
                let input_vec: Vec<&str> = input.split(" ").collect();
                println!("{}",input_vec.get(0).unwrap_or(&"0"));
                println!("Result: {}",time_dilation_module::calc_time_dilation(1.0,299792.457));
            } else {
                println!("Error: Input not valid.\nPlease provide 2 numbers separated by space!");
                print_main_menu();
                return;
            }
        }
        Err(error) => println!("ERROR: Could not read userInput -> {}", error)
    }
}

//just borrow value to keep it memory
fn is_input_valid(input: &String) -> bool {
    //Lazy static to compile regex just once.
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]+ [0-9]+$").unwrap();
    }
    //return result
    RE.is_match(input)
}

