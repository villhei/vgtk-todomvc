#![recursion_limit = "512"]

mod todo;

use vgtk::ext::*;
use vgtk::lib::gio::{ActionExt, SimpleAction, ApplicationFlags};
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, run, Component, UpdateAction, VNode};

use crate::todo::filter::Filter;
use crate::todo::menu::AppMenu;
use crate::todo::about::AboutDialog;
use crate::todo::model::{Model, Task, TaskFilter};
use crate::todo::task_row::TaskRow;

#[derive(Clone, Debug)]
pub enum Message {
    Exit,
    About,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
    Filter { filter: TaskFilter },
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
            Message::About => {
                AboutDialog::run();
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        gtk! {
            <Application::new_unwrap(Some("org.ville.vgtk-todomvc"), ApplicationFlags::empty())>
                <SimpleAction::new("quit", None)
                    Application::accels=["<Meta>q"].as_ref()
                    enabled=true
                    on activate=|_,_| Message::Exit />
                <SimpleAction::new("about", None)
                    enabled=true
                    on activate=|_,_| Message::About />
                <Window
                    default_width=800
                    default_height=600
                    border_width=20
                    on destroy=|_| Message::Exit title="Hello rust">
                <@AppMenu />
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
                                    .map(|(index, task)| gtk! {
                                        <@TaskRow task=task
                                            on changed=|_| Message::Toggle { index }
                                            on deleted=|_| Message::Delete { index }
                                        />
                                    })
                            }
                        </ListBox>
                    </ScrolledWindow>
                    <Box>
                        <Label label=self.items_left() />
                        <@Filter
                            Box::center_widget=true
                            active=&self.filter
                            filters=[("All", TaskFilter::All), ("Active", TaskFilter::Undone), ("Completed", TaskFilter::Done)].as_ref()
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