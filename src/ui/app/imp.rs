use crate::client::AdbClient;
use crate::ui::window::VRexWindow;
use adw::subclass::prelude::AdwApplicationImpl;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::app::bus;

pub struct VRexAppImp {
    pub window: RefCell<Option<Rc<VRexWindow>>>,
    pub client: AdbClient,
}

impl Default for VRexAppImp {
    fn default() -> Self {
        let client = AdbClient::default();
        client.imp().watch();

        Self {
            window: RefCell::new(None),
            client,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for VRexAppImp {
    const NAME: &'static str = "VRexApp";
    type Type = super::VRexApp;
    type ParentType = adw::Application;
}

impl ObjectImpl for VRexAppImp {}

impl ApplicationImpl for VRexAppImp {
    fn activate(&self) {
        self.parent_activate();

        let obj = self.obj();
        let window = VRexWindow::new(&*obj);
        window.set_title(Some(&gettext("VRex")));
        obj.connect_shutdown(glib::clone!(@weak window =>
            move |_| {
                window.destroy();
            }
        ));

        bus::connect(&window, &self.client);

        let window = Rc::new(window);


        self.window.replace(Some(window.clone()));
        window.present();
    }
}

impl GtkApplicationImpl for VRexAppImp {}

impl AdwApplicationImpl for VRexAppImp {}
