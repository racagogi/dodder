use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("dodder")
        .about("file memo tree")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .long_flag("add")
                .about("add file to tree")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .help("added file")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("position")
                        .long("position")
                        .short('p')
                        .help("position to add node")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("sibber")
                        .short('b')
                        .help("set position as sibber")
                        .num_args(0),
                )
                .arg(
                    Arg::new("last")
                        .short('l')
                        .help("insert last position")
                        .num_args(0),
                )
                .arg(
                    Arg::new("status")
                        .short('s')
                        .help("define file state")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .help("set node name")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("time")
                        .short('t')
                        .help("set status time")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("link")
                .short_flag('l')
                .about("link to nodes")
                .arg(Arg::new("nodes").num_args(2).required(true)),
        )
        .subcommand(
            Command::new("remove")
                .arg(
                    Arg::new("node")
                        .short('n')
                        .conflicts_with("link")
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("link")
                        .short('l')
                        .conflicts_with("node")
                        .action(ArgAction::Set)
                        .num_args(2),
                ),
        )
        .subcommand(
            Command::new("set")
                .short_flag('s')
                .arg(Arg::new("time").short('t'))
                .arg(Arg::new("name").short('n'))
                .arg(Arg::new("status").short('s')),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", add_matches)) => {
            println!("file: {:?}", add_matches.get_one::<String>("file"));
            println!("position: {:?}", add_matches.get_one::<String>("position"));
            println!("sibber: {:?}", add_matches.get_one::<bool>("sibber"));
            println!("last: {:?}", add_matches.get_one::<bool>("last"));
            println!("status: {:?}", add_matches.get_one::<String>("status"));
            println!("name: {:?}", add_matches.get_one::<String>("name"));
            println!("time: {:?}", add_matches.get_one::<String>("time"));
        }
        Some(("remove", remove_matches)) => {
            println!("node: {:?}", remove_matches.get_one::<String>("node"));
            println!("link: {:?}", remove_matches.get_raw("link"));
        }
        Some(("link", link_mathces)) => {
            println!("links {:?}", link_mathces.get_raw("nodes"));
        }
        Some(("set", set_mathces)) => {
            println!("time {:?}", set_mathces.get_one::<String>("time"));
            println!("name {:?}", set_mathces.get_one::<String>("naem"));
            println!("status {:?}", set_mathces.get_one::<String>("status"));
        }
        _ => unreachable!(),
    }
}
