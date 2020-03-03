
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, Callback, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct Radio {
     pub labels: &'static [&'static str],
     pub active: usize,
     pub on_changed: Callback<usize>,
}

impl PartialEq for Radio {
    fn eq(&self, props: &Self) -> bool {
        self.labels.eq(props.labels) && self.active == props.active
    }
}

#[derive(Clone, Debug)]
pub enum RadioMessage {
    Changed(usize)
}

impl Component for Radio {
    type Message = RadioMessage;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            RadioMessage::Changed(index) => {
                self.on_changed.send(index);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
        <Box spacing=10>
        {
            self.labels.iter().enumerate().map(|(index, label)| gtk! {
                <ToggleButton label={ * label}
                    active={index == self.active }
                    on toggled=|_| RadioMessage::Changed(index)/>
            })
        }
        </Box>
    }
    }
}