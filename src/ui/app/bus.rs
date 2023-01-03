use gtk::glib;
use gtk::glib::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::client::{AdbClient, get_installed_apps};
use crate::ui::window::VRexWindow;

pub enum Msg {
    DeviceChanged
}

pub fn connect(window: &VRexWindow, client: &AdbClient) {
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    client.connect("devices-changed", false, move |_| {
        tx.send(Msg::DeviceChanged).unwrap();
        None
    });

    window.connect("device-changed", true, move |values| {
        let dev_id = values[1].get::<String>().unwrap();
        if let Some(apps) = get_installed_apps(&dev_id) {
            println!("{:#?}", apps);
        }
        None
    });

    let device = client.imp().get_active_device();
    window.imp().set_device(device.as_ref());
    let window_ref = window.downgrade();
    let client_ref = client.downgrade();

    rx.attach(None, move |msg| {
        let window = &window_ref.upgrade().unwrap();
        let client = &client_ref.upgrade().unwrap();

        match msg {
            Msg::DeviceChanged => {
                let device = client.imp().get_active_device();
                window.imp().set_device(device.as_ref());
            }
        }
        
        glib::Continue(true)
    });
}