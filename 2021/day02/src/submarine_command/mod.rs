use recap::Recap;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?P<direction>(up|down|forward))\s*(?P<distance>\d+)")]
pub struct Command {
    pub direction: String,
    pub distance: i8,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.direction, self.distance)
    }
}
