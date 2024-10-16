
use std::io;

enum MenuOption {
    CreatePie = 1,
    ModifyPie = 2,
    AddFund = 3,
    Exit = 4,
    Error = -1
}


fn print_menu(){
    print!(
    r###"
        Please select the following option:

        1) Create a pie
        2) Modify a pie
        3) Add funds
        4) Exit

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

    loop{
        match user_option {
            user_option if user_option == MenuOption::AddFund as i32 => println!("To be implemented"),
            user_option if user_option == MenuOption::CreatePie as i32 => println!("To be implemented"),
            user_option if user_option == MenuOption::ModifyPie as i32 => println!("To be implemented"),
            user_option if user_option == MenuOption::Exit as i32 => break,
            _ => println!("Please select a valid option")
        } 
    }
    
    
}
