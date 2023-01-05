use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassExt;
use crate::structs::Device;
use crate::structs::device::DevType;
use crate::ui::window::imp::VRexWindowImp;

impl VRexWindowImp {
    pub fn set_device(&self, device: Option<&Device>) {
        if let Some(device) = device {
            match device.dev_type() {
                DevType::Pico4 => {
                    if device.is_online() {
                        self.cover.set_resource(Some("/ru/slie/vrex/icons/pico4.png"));
                        self.device_summary.set_label(&format!("Pico 4: {}", device.id()));
                    } else {
                        self.cover.set_resource(Some("/ru/slie/vrex/icons/pico4_offline.png"));
                        self.device_summary.set_label(&format!("Pico 4: {} (offline)", device.id()));
                    }

                },
                DevType::Quest2 => {
                    if device.is_online() {
                        self.cover.set_resource(Some("/ru/slie/vrex/icons/quest2.png"));
                        self.device_summary.set_label(&format!("Quest 2: {}", device.id()));
                    } else {
                        self.cover.set_resource(Some("/ru/slie/vrex/icons/quest2_offline.png"));
                        self.device_summary.set_label(&format!("Quest 2: {} (offline)", device.id()));
                    }
                }
            }
            self.stack.set_visible_child_name("main");
            self.obj().emit_by_name::<()>("device-changed", &[&device.id()]);
        } else {
            self.cover.set_resource(None);
            self.device_summary.set_label("");
            self.stack.set_visible_child_name("placeholder");
        }
    }
}