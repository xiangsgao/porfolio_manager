
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

fn print_add_fund(){
    print!(
    r###"
        Please add your funds below, use negative number to subtract, if you wish to cancel, simply enter zero
    "###
    )
}

fn print_current_fund(){
    print!(
    r###"
        You have ${}
    "###, 123);
}

fn add_fund(amount: f32){

}

fn interactive_add_fund(){
    print_current_fund();
    print_add_fund();
    
    loop {
       
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("failed to read from stdin");

        let amount: Result<f32, _> = text.trim().parse();
        if let Err(_) = amount{
            print!(
            r###"
                Please enter a valid amount
            "###
            );
            continue;
        }

        add_fund(amount.unwrap());
        println!(
            r###"
                Fund added
            "###
        );
        break;
    }
}

pub fn ask_menu_option() -> i32{
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let user_option: i32 = match input_text.trim().parse() {
        Ok(e) => e,
        Err(_) => -1
    };
    user_option
}

pub fn interactive_menu(){
    print_menu();
    let mut user_option = ask_menu_option();

    loop{
        match user_option {
            user_option if user_option == MenuOption::AddFund as i32 => interactive_add_fund(),
            user_option if user_option == MenuOption::CreatePie as i32 => println!("To be implemented"),
            user_option if user_option == MenuOption::ModifyPie as i32 => println!("To be implemented"),
            user_option if user_option == MenuOption::Exit as i32 => break,
            _ => println!("Please select a valid option")
        } 
        print_menu();
        user_option = ask_menu_option();
    }
    
    
}
