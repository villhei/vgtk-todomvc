use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, VNode};

fn build_todo_menu() -> vgtk::lib::gio::Menu {
    vgtk::menu()
        .section(vgtk::menu().item("About", "app.about"))
        .section(vgtk::menu().item("Quit", "app.quit"))
        .build()
}

#[derive(Clone, Debug, Default)]
pub struct AppMenu {
}


impl Component for AppMenu {
    type Message = ();
    type Properties = Self;

    fn view(&self) -> VNode<Self> {
        let main_menu = build_todo_menu();
        gtk! {
            <HeaderBar title="The Todo List" show_close_button=true>
                    <MenuButton HeaderBar::pack_type=PackType::Start
                        @MenuButtonExt::direction=ArrowType::Down
                        relief=ReliefStyle::None
                        image="open-menu-symbolic">
                        <Menu::new_from_model(&main_menu)/>
                    </MenuButton>
                </HeaderBar>
        }
    }
}
