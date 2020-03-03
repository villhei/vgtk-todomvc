use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
struct Radio {
    pub labels: &'static [&'static str],
    pub active: usize,
}

#[derive(Clone, Debug)]
enum RadioMessage {}

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

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box spacing=10>
            {
                self.labels.iter().enumerate().map(|(index, label)| gtk! {
                    <ToggleButton label={ * label}
                    active={index == self.active } />
                })
            }
            </Box>
        }
    }
}
