mod imp;
mod device_selector;
mod app_list;

use gtk::{gio, glib};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::ui::window::app_list::AppList;

glib::wrapper! {
    pub struct VRexWindow(ObjectSubclass<imp::VRexWindowImp>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::ConstraintTarget, gtk::Native, gtk::Root;
}

impl VRexWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
    }

    pub fn app_list(&self) -> AppList {
        self.imp().app_list_box.get()
    }
}
