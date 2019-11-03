use log::info;
use std::collections::HashMap;
use std::{thread, time};

mod ant;
mod command_queue;
mod direction;
mod field;
mod position;

use ant::Ant;
use field::Field;
use position::Position;

pub struct Langtons<'a> {
    fields: HashMap<Position, Field<'a>>,
    ant: Ant,
    current_position: Position,
}

impl Langtons<'_> {
    pub fn new() -> Self {
        let mut fields = HashMap::new();
        let ant = Ant::default();
        let current_position = Position::default();

        fields.insert(Position::default(), Field::default());

        Self {
            fields,
            ant,
            current_position,
        }
    }

    pub fn start_simulation(&mut self) {
        loop {
            if let Some(field) = self.fields.get_mut(&self.current_position) {
                // issue command to ant for field on current_position
                self.ant.execute_command(field.advance_queue());

                // let ant take a step forward
                self.current_position = self.ant.step();

                if let None = self.fields.get(&self.current_position) {
                    info!("no new field");
                    self.fields
                        .insert(self.current_position, Field::new());
                }

                info!("current pos now: {:?}", self.current_position);
                // wait a bit
                let quarter_second = time::Duration::from_millis(250);

                thread::sleep(quarter_second);
                println!("")
            }
        }
    }
}
