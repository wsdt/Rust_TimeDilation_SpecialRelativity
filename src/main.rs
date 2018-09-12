mod calc;
use calc::time_dilation_module;
use std::io;

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
            println!("{} - {} bytes read",n, input);
        }
        Err(error) => println!("ERROR: Could not read userInput -> {}", error)
    }


    //Example execution (works)
    println!("Result: {}",time_dilation_module::calc_time_dilation(1.0,299792.457));
}

