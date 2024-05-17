use clap::{Parser, Subcommand};
use player::{Player, show_welcome, show_prompt };

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "View a value. Choose from: 'profile' or profile components.")]
    Show{key: String},
}

fn main() {

    // create a new player
    let player = Player::new();
    show_welcome(&player);

    // loop player input
    loop {

        show_prompt(&player);

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let input = format!("{} {}", "car ", input);

        // split the input into args
        let args = shlex::split(&input).ok_or("error: Invalid quoting").unwrap();

        // parse the input
        let cli = Cli::try_parse_from(args.iter());

        // check if parsing was successful
        match cli {
            Ok(cli) => {
                // handle the input
                match cli.commands {
                    Commands::Show { key } => player.handle_show(key.as_str()),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}