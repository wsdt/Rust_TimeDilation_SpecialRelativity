mod calc;
use calc::time_dilation_module;

fn main() {
    print_main_menu();
}

fn print_main_menu() {
    println!("********** Rust - Time Dilation Calculator *************");
    println!("~ © Riedl Kevin, 2018 ~");
    println!("\nWelcome and thank you for testing/using my time dilation calculator.\n\
        As far as I know, it's the first one which has been written in Rust!");

    //Example execution (works)
    println!("Result: {}",time_dilation_module::calc_time_dilation(1.0,299792.457));
}
