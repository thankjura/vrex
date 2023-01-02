mod imp;
mod update;

use gtk::glib;

glib::wrapper! {
    pub struct AdbClient(ObjectSubclass<imp::AdbClientImp>);
}

impl Default for AdbClient {
    fn default() -> Self {
        glib::Object::new(&[])
    }
}
