use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct Nick {
    pub nick: String,
    colour: String,
}

impl fmt::Display for Nick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::*;

        let bracket_colour = "bright black";
        write!(
            f,
            "{}{}{}",
            "<".color(bracket_colour),
            self.nick.color(self.colour.as_str()).bold(),
            ">".color(bracket_colour)
        )
    }
}

impl Nick {
    pub fn new() -> Self {
        Nick {
            nick: String::new(),
            colour: String::new(),
        }
    }

    pub fn recalculate_colour(&mut self) {
        self.colour = crate::colour_on_hash(&self.nick).into();
    }
}
