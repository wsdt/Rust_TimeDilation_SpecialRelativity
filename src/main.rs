mod calc;
use calc::time_dilation_module;
use std::io;
#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    loop { print_main_menu()};
}

fn print_main_menu() {
    println!("********** Rust - Time Dilation Calculator *************");
    println!("~ © Riedl Kevin, 2018 ~");
    println!("\nWelcome and thank you for testing/using my time dilation calculator.\n\
        As far as I know, it's the first one which has been written in Rust!");

    println!("Input proper time and relative velocity separated by a space: ");

    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read_error");
    if is_input_valid(&a_str) {
        let user_input: Vec<f64> = a_str.split_whitespace()
            .map(|x| x.parse::<f64>().expect("parse error"))
            .collect::<Vec<f64>>();

        println!("Time-Dilation: {}",time_dilation_module::calc_time_dilation(
            *user_input.get(0).unwrap(),
            *user_input.get(1).unwrap()));
    } else {
        println!("Error: Input not valid.\nPlease provide 2 numbers separated by space!");
        print_main_menu();
        return;
    }
}

//just borrow value to keep it memory
fn is_input_valid(input: &String) -> bool {
    //Lazy static to compile regex just once.
    lazy_static! {
        static ref RE: Regex = Regex::new("[0-9]+ [0-9]+").unwrap();
    }
    //return result
    RE.is_match(input)
}

