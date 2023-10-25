mod about;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use crate::about::AboutDialog;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (800, 600), center: true, title: "Gosub PoC UserAgent", flags: "MAIN_WINDOW|RESIZABLE|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "&File")]
    file_menu: nwg::Menu,

    #[nwg_control(text: "&Open\tCtrl+O", parent: file_menu)]
    #[nwg_events( OnMenuItemSelected: [BasicApp::do_open] )]
    file_open_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    file_separator_1: nwg::MenuSeparator,

    #[nwg_control(text: "E&xit\tAlt+F4", parent: file_menu)]
    #[nwg_events( OnMenuItemSelected: [BasicApp::do_exit] )]
    file_exit_item: nwg::MenuItem,


    #[nwg_control(text: "&Help", )]
    help_menu: nwg::Menu,

    #[nwg_control(text: "&Get Help", parent: help_menu)]
    #[nwg_events( OnMenuItemSelected: [BasicApp::do_open] )]
    help_get_help_item: nwg::MenuItem,

    #[nwg_control(parent: help_menu)]
    help_separator_1: nwg::MenuSeparator,

    #[nwg_control(text: "A&bout Gosub", parent: help_menu)]
    #[nwg_events( OnMenuItemSelected: [BasicApp::do_help_about] )]
    help_about_item: nwg::MenuItem,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,

    #[nwg_control]
    #[nwg_events( OnNotice: [BasicApp::read_dialog_output] )]
    dialog_notice: nwg::Notice,
}

impl BasicApp {

    fn do_help_about(&self) {
        AboutDialog::popup(self.dialog_notice.sender());
    }

    fn read_dialog_output(&self) {
        println!("dialog output has been received");
    }


    fn do_open(&self) {
    }

    fn do_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
