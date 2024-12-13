use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    id: usize,
    description: String,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Self { id, description }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} - {}", self.id, self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let task: Task = Task::new(1, "Hello, world!".to_string());

        assert_eq!(Task::new(1, "Hello, world!".to_string()), task);
    }
}
