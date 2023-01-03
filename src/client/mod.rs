mod imp;
mod update;
mod installed_apps;

use gtk::glib;
pub use installed_apps::get_installed_apps;

const ADB_PROGRAM: &str = "adb";

glib::wrapper! {
    pub struct AdbClient(ObjectSubclass<imp::AdbClientImp>);
}

impl Default for AdbClient {
    fn default() -> Self {
        glib::Object::new(&[])
    }
}