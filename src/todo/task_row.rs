use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, Callback, UpdateAction, VNode};
use super::model::{Task};

#[derive(Clone, Debug, Default)]
pub struct TaskRow {
    pub task: Task,
    pub on_changed: Callback<()>,
    pub on_deleted: Callback<()>,
}

#[derive(Clone, Debug)]
pub enum TaskMessage {
    Changed,
    Delete,
}

impl Component for TaskRow {
    type Message = TaskMessage;
    type Properties = Self;

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            TaskMessage::Changed => {
                self.on_changed.send(());
                UpdateAction::Render
            }
            TaskMessage::Delete => {
                self.on_deleted.send(());
                UpdateAction::None
            }
        }
    }

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <ListBoxRow>
                <Box spacing=9>
                    <CheckButton active=self.task.done on toggled=|_| TaskMessage::Changed />
                    <Label label=self.task.label() use_markup=true />
                    <Button
                        Box::pack_type=PackType::End
                        relief=ReliefStyle::None image="edit-delete"
                        on clicked=|_| TaskMessage::Delete />
                </Box>
            </ListBoxRow>
        }
    }
}