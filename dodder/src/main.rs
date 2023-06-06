use chrono::Utc;
use dodder::{config::Config, dodder::Dodder, leaf::LeafData};

fn main() {
    Config::new();
    let config = Config::read();
    let mut dodder = Dodder::read(true);
    let temp1 = LeafData::new(
        "temp1",
        None,
        dodder::leaf::GTD::Hold,
        Utc::now(),
        true,
        &config,
    );
    let temp2 = LeafData::new(
        "temp2",
        None,
        dodder::leaf::GTD::Hold,
        Utc::now(),
        true,
        &config,
    );
    let temp3 = LeafData::new(
        "temp3",
        None,
        dodder::leaf::GTD::Hold,
        Utc::now(),
        true,
        &config,
    );
    dodder.add_child_last(temp1, 0);
    dodder.add_child_first(temp2, 1);
    dodder.add_child_first(temp3, 0);
    dodder.add_link(0, 3);
    dodder.add_link(1, 2);
    dodder.remove_link(0, 1);
    dodder.print(&config);
    dodder.write(true);
}
