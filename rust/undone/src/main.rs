use clap::{Parser, Subcommand};

mod ui;
mod util;
mod player;
mod games;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "dice | reaction | impulse | tetris        ::", visible_alias = "p")]
    Play{game: String},
    
    #[clap(about = "replay last game               ::", visible_alias = "r")]
    Replay{},
 
    #[clap(about = "room1 | room2 | room3          ::", visible_alias = "c")]
    Chat{room: String},

    #[clap(about = "profile | name | level | coin  ::", visible_alias = "sh")]
    Show{value: String},

    #[clap(about = "name                           ::", visible_alias = "ch")]
    Change{option: String},

    #[clap(about = "clear the terminal             ::", visible_alias = "cl")]
    Clear{},

    #[clap(about = "exit the program", visible_aliases = &["q", "e"])]
    Exit{},
}

fn main() {
    let name = ui::prompt_username();
    
    // create a new player  
    let mut player = player::Player::new(name);
    
    // main loop
    loop {
        ui::print_main_prompt(&player);

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
                    Commands::Play{game} => player.games.handle_play(game.as_str()),
                    Commands::Replay{} => player.games.handle_replay(),
                    Commands::Chat {room} => println!("{room}"),
                    Commands::Show{value} => player.handle_show(value.as_str()),
                    Commands::Change{option} => player.handle_change(option.as_str()),
                    Commands::Clear{} => ui::clear_terminal(),
                    Commands::Exit{} => { println!("\nGoodbye"); break; },
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}


 // Commands::Chat { room } => {
                    //     // Execute the command here
                    //     std::process::Command::new("cargo")
                    //        .arg("run")
                    //        .arg("--bin")
                    //        .arg("server")
                    //        .output()
                    //        .expect("Failed to execute command");
                    // },