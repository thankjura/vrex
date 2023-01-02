use crate::structs::device::{DevType, PICO4, QUEST2};
use crate::structs::Device;
use gettextrs::gettext;
use std::process;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

fn get_connected_devices() -> Option<Vec<Device>> {
    if let Ok(value) = process::Command::new("adb")
        .args(["devices", "-l"])
        .output()
    {
        if value.status.success() {
            if let Ok(device_lines) = from_utf8(&value.stdout) {
                let mut out = vec![];

                for line in device_lines.split("\n") {
                    let dev_type = if line.contains(PICO4) {
                        Some(DevType::Pico4)
                    } else if line.contains(QUEST2) {
                        Some(DevType::Quest2)
                    } else {
                        None
                    };

                    if let Some(dev_type) = dev_type {
                        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
                        if parts.len() != 7 {
                            continue;
                        }

                        out.push(Device::new(parts.get(0).unwrap(), dev_type));
                    }
                }

                return Some(out);
            }
        } else {
            println!(
                "{}",
                gettext("Can't receive list of device. Are adb installed?")
            )
        }
    }

    None
}

pub(super) fn update_devices_list(devices: &Arc<Mutex<Vec<Device>>>) -> bool {
    let mut out = false;

    if let Some(mut new_devices) = get_connected_devices() {
        let mut devices = devices.lock().unwrap();

        for dev in devices.iter_mut() {
            if let Some(index) = new_devices.iter().position(|d| d.eq(&dev)) {
                if !dev.is_online() {
                    dev.set_online(true);
                    out = true;
                }

                new_devices.remove(index);
            } else {
                if dev.is_online() {
                    dev.set_online(false);
                    out = true;
                }
            }
        }

        if new_devices.len() > 0 {
            out = true;

            for dev in new_devices {
                devices.push(dev);
            }
        }
    }

    out
}
