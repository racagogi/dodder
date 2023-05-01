use dodder::{cli, command};

fn main() {
    let matches = cli::cli();

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
        Some(("move", move_mathces)) => {
            println!("time {:?}", move_mathces.get_one::<String>("time"));
            println!("name {:?}", move_mathces.get_one::<String>("naem"));
        }
        Some(("print", print_mathces)) => {
            let from = print_mathces.get_one::<String>("from");
            let verbose = print_mathces.get_one::<bool>("verbose").unwrap();
            let print = command::print(&from, verbose.to_owned());
            println!("{print}");
        }
        _ => unreachable!(),
    }
}
