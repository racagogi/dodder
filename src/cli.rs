use clap::{Arg, ArgAction, Command};

pub fn cli() -> clap::ArgMatches {
    Command::new("dodder")
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
        .subcommand(
            Command::new("move")
                .arg(Arg::new("from").short('f'))
                .arg(Arg::new("to").short('t'))
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
                ),
        )
        .subcommand(
            Command::new("print")
                .arg(Arg::new("from").short('f'))
                .arg(Arg::new("verbose").short('v').num_args(0)),
        )
        .subcommand(Command::new("init"))
        .get_matches()
}
