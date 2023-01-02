use crate::structs::Device;
use crate::structs::device::DevType;
use crate::ui::window::imp::VRexWindowImp;

impl VRexWindowImp {
    pub fn set_device(&self, device: Option<&Device>) {
        if let Some(device) = device {
            match device.dev_type() {
                DevType::Pico4 => {
                    self.cover.set_resource(Some("/ru/slie/vrex/icons/pico4.png"));
                    self.device_summary.set_label(&format!("Pico 4: {}", device.id()));
                },
                DevType::Quest2 => {
                    self.cover.set_resource(Some("/ru/slie/vrex/icons/quest2.png"));
                    self.device_summary.set_label(&format!("Quest 2: {}", device.id()));
                }
            }
        } else {
            self.cover.set_resource(None);
            self.device_summary.set_label("");
        }
    }
}