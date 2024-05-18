use clap::{Parser, Subcommand};

mod ui;
mod player;
mod games;

// use chat::{Chat};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "dice | impulse | tetris")]
    Play{game: String},

    #[clap(about = "room1 | room2 | room3")]
    Chat{room: String},

    #[clap(about = "profile | name | level | coin")]
    Show{value: String},

    #[clap(about = "name")]
    Change{option: String, value: String},
}

fn main() {
    let name = ui::prompt_username();

    // create a new player  
    let mut player = player::Player::new(name);

    // create Games instance for the player
    let games = games::Games::new(&player);

    // loop player input
    loop {
        ui::show_prompt(&player);

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // trim the automatic new line
        let input = input.trim();

        // automatically include initial arg
        let input = format!("{} {}", "> ", input);

        // split the input into args
        let args = shlex::split(&input).ok_or("error: Invalid quoting").unwrap();

        // parse the input
        let cli = Cli::try_parse_from(args.iter());

        // check if parsing was successful
        match cli {
            Ok(cli) => {
                // handle the input
                match cli.commands {
                    Commands::Play{game} => games.handle_play(game.as_str()),
                    // Commands::Chat { room } => {
                    //     // Execute the command here
                    //     std::process::Command::new("cargo")
                    //        .arg("run")
                    //        .arg("--bin")
                    //        .arg("server")
                    //        .output()
                    //        .expect("Failed to execute command");
                    // },
                    Commands::Chat { room } => (),
                    Commands::Show{value} => player.handle_show(value.as_str()),
                    Commands::Change{option, value} => player.handle_change(option.as_str(), value.as_str()),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}