pub mod load;
pub mod tasks;

#[allow(unused_imports)]
use load::load_task;

#[allow(unused_imports)]
use tasks::Task;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_task() {
        let task: Task = Task::new(1, "Hello, world!".to_string());

        assert_eq!(Task::new(1, "Hello, world!".to_string()), task);
    }

    #[test]
    fn loading_task() {
        let expected_result = vec![
            Task::new(1, "Buy some milk.".to_string()),
            Task::new(2, "Eat more healthy".to_string()),
            Task::new(3, "Drink more water".to_string()),
        ];

        assert_eq!(expected_result, load_task("../tasks.txt".to_string()));
    }
}
