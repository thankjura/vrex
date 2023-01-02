mod imp;
mod bus;

use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct VRexApp(ObjectSubclass<imp::VRexAppImp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap, gio::ApplicationCommandLine;
}

impl VRexApp {
    pub fn new(app_id: &str) -> Self {
        glib::Object::new(&[("application-id", &app_id)])
    }

    pub fn has_window(&self) -> bool {
        let obj = self.imp();
        if obj.window.borrow_mut().is_some() {
            return true;
        };

        false
    }
}
