#![recursion_limit = "256"]

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};
mod radio;

#[derive(Clone, Debug)]
struct Task {
    text: String,
    done: bool,
}

impl Task {
    pub fn new<S: ToString>(text: S, done: bool) -> Self {
        Self {
            text: text.to_string(),
            done,
        }
    }

    fn label(&self) -> String {
        if self.done {
            format!(
                "<span strikethrough=\"true\" alpha=\"50%\">{}</span>",
                self.text
            )
        } else {
            self.text.clone()
        }
    }

    fn render(&self, index: usize) -> VNode<Model> {
        gtk! {
            <ListBoxRow>
                <Box spacing=9>
                    <CheckButton active=self.done on toggled=|_| Message::Toggle { index } />
                    <Label label=self.label() use_markup=true />
                    <Button
                        Box::pack_type=PackType::End
                        relief=ReliefStyle::None image="edit-delete"
                        on clicked=|_| Message::Delete {
                        index
                    } />
                </Box>
            </ListBoxRow>
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<Task>,
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
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
}



impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::Toggle { index } => {
                self.tasks[index].done = !self.tasks[index].done;
                UpdateAction::Render
            }
            Message::Add { task } => {
                self.tasks.push(Task::new(task, false));
                UpdateAction::Render
            }
            Message::Delete { index } => {
                self.tasks.remove(index);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("org.ville.vgtk-todomvc"), ApplicationFlags::empty())>
                <Window
                    default_width=800
                    default_height=600
                    border_width=20
                    on destroy=|_| Message::Exit title="Hello rust">
                <Box orientation=Orientation::Vertical spacing=18>
                    <Entry placeholder_text="What needs to be done?"
                        on activate=|entry| {
                        entry.select_region(0, -1);
                        Message::Add {
                            task: entry.get_text().unwrap().to_string()
                        }
                    }/>
                    <ScrolledWindow Box::fill=true Box::expand=true>
                        <ListBox selection_mode=SelectionMode::None>
                            {
                                self.tasks.iter().enumerate().map(|(index, task)| task.render(index))
                            }
                        </ListBox>
                    </ScrolledWindow>
                </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
