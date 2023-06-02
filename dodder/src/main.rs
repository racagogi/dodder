use dodder::{config::Config, dodder::Dodder};


fn main() {
    Config::new();
    let config = Config::read();
    println!("{:?}",config);
    Dodder::read(true);
}
