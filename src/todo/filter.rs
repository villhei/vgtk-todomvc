
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, Callback, UpdateAction, VNode};
use super::model::{TaskFilter};

#[derive(Clone, Debug, Default)]
pub struct Filter {
     pub filters: &'static [(&'static str, TaskFilter)],
     pub active: TaskFilter,
     pub on_changed: Callback<TaskFilter>,
}

impl PartialEq for Filter {
    fn eq(&self, props: &Self) -> bool {
        self.filters.eq(props.filters) && self.active == props.active
    }
}

#[derive(Clone, Debug)]
pub enum FilterMessage {
    Changed(TaskFilter)
}

impl Component for Filter {
    type Message = FilterMessage;
    type Properties = Self;

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            FilterMessage::Changed(filter) => {
                self.on_changed.send(filter);
                UpdateAction::Render
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
            <Box spacing=10>
            {
                self.filters.iter().map(|(label, filter)| gtk! {
                    <ToggleButton label={ * label}
                        active={filter == &self.active }
                        on toggled=|_| FilterMessage::Changed(filter.clone())/>
                })
            }
            </Box>
        }
    }
}