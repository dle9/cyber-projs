use clap::{Parser, Subcommand};
use player::{Player, show_welcome, show_prompt, get_username};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "Play a game. Choose from: 'dice', 'nines', 'tetris'")]
    Play{game: String},

    #[clap(about = "Enter a chatroom. Choose from: 'room1', 'room2', 'room3'")]
    Chat{room: String},

    #[clap(about = "Show a value. Choose from: 'profile', 'name', 'level'")]
    Show{value: String},

    #[clap(about = "Change a value. Choose from: 'name'")]
    Change{option: String, value: String},
}

fn main() {
    let name = get_username();

    // create a new player  
    let mut player = Player::new(name);
    show_welcome(&player);

    // loop player input
    loop {
        show_prompt(&player);

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // trim the automatic new line
        let input = input.trim();

        // automatically include initial arg
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
                    Commands::Play{game} => println!("Play"),
                    Commands::Chat{room} => println!("Chat"),
                    Commands::Show{value} => player.handle_show(value.as_str()),
                    Commands::Change{option, value} => player.handle_change(option.as_str(), value.as_str()),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}