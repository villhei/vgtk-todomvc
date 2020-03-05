

#[derive(Clone, Debug)]
pub struct Task {
    pub text: String,
    pub done: bool,
}

impl Task {
    pub fn new<S: ToString>(text: S, done: bool) -> Self {
        Self {
            text: text.to_string(),
            done,
        }
    }

    pub fn label(&self) -> String {
        if self.done {
            format!(
                "<span strikethrough=\"true\" alpha=\"50%\">{}</span>",
                self.text
            )
        } else {
            self.text.clone()
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            done: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskFilter {
    All,
    Done,
    Undone,
}

impl Default for TaskFilter {
    fn default() -> TaskFilter { TaskFilter::All }
}

#[derive(Clone, Debug)]
pub struct Model {
    pub tasks: Vec<Task>,
    pub filter: TaskFilter,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("While my guitar", false),
                Task::new("Lorem lipsun", false),
                Task::new("Kukkeliskuu", false),
                Task::new("HopHop", true),
                Task::new("Kukkeliskuu", false),
            ],
            filter: TaskFilter::All,
        }
    }
}

impl Model {
    pub fn items_left(&self) -> String {
        let tasks_left_count = self.tasks.iter().filter(|task| !task.done).count();
        let plural = if tasks_left_count == 1 { "item " } else { "items" };
        format!("{} {} tasks left", tasks_left_count, plural)
    }
    pub fn filter_task(&self, task: &Task) -> bool {
        match self.filter {
            TaskFilter::All => true,
            TaskFilter::Undone => !task.done,
            TaskFilter::Done => task.done,
        }
    }
    pub fn count_completed(&self) -> usize {
        self.tasks.iter().filter(|task| task.done).count()
    }
}
