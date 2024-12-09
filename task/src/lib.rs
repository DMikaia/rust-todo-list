pub mod tasks;

#[cfg(test)]
mod tests {
    use super::*;
    use tasks::Task;

    #[test]
    fn it_works() {
        let task: Task = Task::new(1, "Hello, world!".to_string());

        assert_eq!(Task::new(1, "Hello, world!".to_string()), task);
    }
}
