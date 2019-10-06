
mod langtons;

use langtons::Langtons;

fn main() {
    env_logger::init();

    let mut langtons = Langtons::new();

    langtons.start_simulation();
}
