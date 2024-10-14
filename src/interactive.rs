
use clap::Parser;
use std::io;

enum MenuOption {
    CREATE_PIE = 1,
    MODIFY_PIE = 2,
    ADD_FUND = 3,
    ERROR = -1
}


fn print_menu(){
    print!(
    r###"
        Please select the following option:

        1) Create a pie
        2) Modify a pie
        3) Add funds

    "###
    );
}


pub fn interactive_menu(){
    print_menu();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let user_option: i32 = match input_text.trim().parse() {
        Ok(e) => e,
        Err(_) => -1
    };

    match user_option {
        1 => println!("To be implemented"),
        2 => println!("To be implemented"),
        3 => println!("To be implemented"),
        _ => println!("Please select a valid option")
    } 
    
}
