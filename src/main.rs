mod interactive;

use clap::Parser;
use interactive::interactive_menu;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(long, short, action, default_value_t = true)]
    interactive: bool
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if(args).interactive{
        interactive_menu();
    }

    Ok(())
}
