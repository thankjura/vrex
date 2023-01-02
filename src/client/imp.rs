use crate::client::update::update_devices_list;
use crate::structs::Device;
use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt};
use std::sync::{Arc, Mutex};
use gtk::prelude::ObjectExt;

pub struct AdbClientImp {
    devices: Arc<Mutex<Vec<Device>>>,
    selected_index: usize,
}

impl Default for AdbClientImp {
    fn default() -> Self {
        Self::new()
    }
}

impl AdbClientImp {
    pub fn new() -> Self {
        let devices = Arc::new(Mutex::new(vec![]));
        Self {
            devices,
            selected_index: 0,
        }
    }

    pub fn watch(&self) {
        let obj_ref = self.obj().downgrade();
        let devices = Arc::clone(&self.devices);

        glib::timeout_add_seconds(1, move || {
            let obj = obj_ref.upgrade().unwrap();
            if update_devices_list(&devices) {
                obj.emit_by_name::<()>("devices-changed", &[]);
            }
            glib::Continue(true)
        });
    }

    pub fn get_active_device(&self) -> Option<Device> {
        if let Some(dev) = self.devices.lock().unwrap().get(self.selected_index) {
            return Some(dev.clone());
        }

        None
    }

    pub fn set_active_device(&mut self, index: usize) {
        self.selected_index = index;
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AdbClientImp {
    const NAME: &'static str = "AdbClient";
    type Type = super::AdbClient;
}

impl ObjectImpl for AdbClientImp {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![
            Signal::builder("devices-changed").build()
        ]);

        SIGNALS.as_ref()
    }
}
