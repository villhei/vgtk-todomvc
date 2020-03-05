
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, Callback, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct Filter {
     pub labels: &'static [&'static str],
     pub active: usize,
     pub on_changed: Callback<usize>,
}

impl PartialEq for Filter {
    fn eq(&self, props: &Self) -> bool {
        self.labels.eq(props.labels) && self.active == props.active
    }
}

#[derive(Clone, Debug)]
pub enum FilterMessage {
    Changed(usize)
}

impl Component for Filter {
    type Message = FilterMessage;
    type Properties = Self;

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            FilterMessage::Changed(index) => {
                self.on_changed.send(index);
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
                self.labels.iter().enumerate().map(|(index, label)| gtk! {
                    <ToggleButton label={ * label}
                        active={index == self.active }
                        on toggled=|_| FilterMessage::Changed(index)/>
                })
            }
            </Box>
        }
    }
}