extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::cell::RefCell;
use std::thread;
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct AboutDialog {
    data: RefCell<Option<String>>,

    #[nwg_control(size: (300, 300), center: true, title: "About Gosub", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [AboutDialog::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 3)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Welcome to Gosub\r\n\r\nCurrent version: 0.0.1\r\nClick here to look for updates\r\n\r\nGosub is (c) 2023 - Gosub")]
    #[nwg_layout_item(layout: grid, rowspan: 6, col: 0, row: 0)]
    text: nwg::Label,

    #[nwg_control(text: "Close", focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 7)]
    #[nwg_events( OnButtonClick: [AboutDialog::close] )]
    add_message_btn: nwg::Button,
}


impl AboutDialog {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn popup(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            // Create the UI just like in the main function
            let app = AboutDialog::build_ui(Default::default()).expect("Failed to build UI");
            nwg::dispatch_thread_events();

            // Notice the main thread that the dialog completed
            sender.notice();

            // Return the dialog data
            app.data.take().unwrap_or("Cancelled!".to_owned())
        })
    }
}