pub mod task_structure {

    pub struct Task {
        pub title: String,
        pub description: String,
        done: bool,
    }

    impl Task {
        pub fn new(title: &str, description: &str) -> Self {
            Task {
                title: title.to_string(),
                description: description.to_string(),
                done: false,
            }
        }

        pub fn mark_done(&mut self) {
            self.done = true;
        }
    }
}