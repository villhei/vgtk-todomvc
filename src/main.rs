#![recursion_limit = "512"]

mod radio;

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, run, Component, UpdateAction, VNode};

use crate::radio::Radio;


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
    filter: usize,
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

impl Model {
    fn items_left(&self) -> String {
        let tasks_left_count = self.tasks.iter().filter(|task| !task.done).count();
        let plural = if tasks_left_count == 1 { "item "} else { "items" };
        format!("{} {} tasks left", tasks_left_count, plural)
    }
    fn filter_task(&self, task: &Task) -> bool {
        match self.filter {
            0 => true,
            1 => !task.done,
            2 => task.done,
            _ => unreachable!(),
        }
    }
    fn count_completed(&self) -> usize {
        self.tasks.iter().filter(|task| task.done).count()
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
    Filter { filter: usize },
    Cleanup
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
            Message::Filter { filter } => {
                self.filter = filter;
                UpdateAction::Render
            }
            Message::Cleanup => {
                self.tasks.retain(|task| !task.done);
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
                                self.tasks.iter()
                                    .filter(|task| self.filter_task(task))
                                    .enumerate()
                                    .map(|(index, task)| task.render(index))
                            }
                        </ListBox>
                    </ScrolledWindow>
                    <Box>
                        <Label label=self.items_left() />
                        <@Radio
                            Box::center_widget=true
                            active=self.filter
                            labels=["All", "Active", "Completed"].as_ref()
                            on changed=|filter| Message::Filter { filter } />
                        {
                            gtk_if!(self.count_completed() > 0 => {
                                <Button
                                    label="Clear completed"
                                    Box::pack_type=PackType::End
                                    on clicked=|_| Message::Cleanup />
                            })
                        }
                    </Box>
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
