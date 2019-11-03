
mod langtons;

use langtons::Langtons;
use langtons::LangtonsAnt;

fn main() {
    env_logger::init();
    
    // create a langtons ant system
    let mut langtons = Langtons::new();

    // loop over cycles
    loop {
        // cycle should look like:
        // tile: issue command to ant
        let position = langtons.issue_command();
        let active_face = langtons.grab_or_create();
        langtons.append_or_overwrite(active_face, position);
        langtons.step_forward();

        // antn: execute command
        // antn: take one step
        // plane: switch_active
    }
}
