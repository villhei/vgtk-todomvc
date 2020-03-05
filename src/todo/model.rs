

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

#[derive(Clone, Debug)]
pub  struct Model {
    pub tasks: Vec<Task>,
    pub filter: usize,
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
            filter: 0,
        }
    }
}
