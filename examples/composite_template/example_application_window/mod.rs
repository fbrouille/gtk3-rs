mod imp;

use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl ExampleApplicationWindow {
    pub fn new<P: glib::object::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let imp = self.imp();
        imp.subtitle
            .set_text("This is an example window made using composite templates");
    }
}
