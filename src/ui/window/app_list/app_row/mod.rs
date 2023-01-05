mod imp;

use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassIsExt;


glib::wrapper! {
    pub struct AppRow(ObjectSubclass<imp::AppRowImp>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for AppRow {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRow {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn set_app_info(&self, name: &str, version: &str) {
        let imp = self.imp();
        imp.name.set_label(name);
        imp.version.set_label(version);
    }

    pub fn name(&self) -> String {
        self.imp().name.label().to_string()
    }

    pub fn version(&self) -> String {
        self.imp().version.label().to_string()
    }
}