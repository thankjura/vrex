mod imp;
mod device_selector;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct VRexWindow(ObjectSubclass<imp::VRexWindowImp>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl VRexWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }
}
