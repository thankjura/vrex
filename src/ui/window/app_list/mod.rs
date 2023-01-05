mod imp;
mod app_row;
mod store;

use gtk::glib;

glib::wrapper! {
    pub struct AppList(ObjectSubclass<imp::AppListImp>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for AppList {
    fn default() -> Self {
        Self::new()
    }
}

impl AppList {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}