use vgtk::ext::*;
use vgtk::lib::gdk_pixbuf::Pixbuf;
use vgtk::lib::gio::{Cancellable, MemoryInputStream};
use vgtk::lib::glib::Bytes;
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, VNode};

static DOG: &[u8] = include_bytes!("../dog.png");

pub struct AboutDialog {
    dog: Pixbuf,
}

impl Default for AboutDialog {
    fn default() -> Self {
        let data_stream = MemoryInputStream::new_from_bytes(&Bytes::from_static(DOG));
        let dog = Pixbuf::new_from_stream(&data_stream, None as Option<&Cancellable>).unwrap();
        AboutDialog { dog }
    }
}

impl Component for AboutDialog {
    type Message = ();
    type Properties = ();

    fn view(&self) -> VNode<Self> {
        gtk! {
    <Dialog::new_with_buttons(
        Some("About the application"),
        None as Option<&Window>,
        DialogFlags::MODAL,
        &[("Ok", ResponseType::Ok)]
    )>
        <Box spacing=10 orientation=Orientation::Vertical>
            <Image pixbuf=Some(self.dog.clone())/>
            <Label markup="<big><b>Cool app</b></big>"/>
        </Box>
     </Dialog>
    }
    }
}

impl AboutDialog {
    #[allow(unused_must_use)]
    pub fn run() {
        vgtk::run_dialog::<AboutDialog>(vgtk::current_window().as_ref());
    }
}