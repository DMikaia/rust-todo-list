use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Task {
    id: usize,
    description: String,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self { id, description }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} - {}", self.id, self.description)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.description == other.description
    }
}
