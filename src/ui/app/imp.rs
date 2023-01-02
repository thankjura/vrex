use crate::ui::window::VRexWindow;
use adw::subclass::prelude::AdwApplicationImpl;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct VRexAppImp {
    pub window: RefCell<Option<Rc<VRexWindow>>>,
}

impl Default for VRexAppImp {
    fn default() -> Self {
        Self {
            window: RefCell::new(None),
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

        let window = Rc::new(window);

        self.window.replace(Some(window.clone()));
        window.present();
    }
}

impl GtkApplicationImpl for VRexAppImp {}

impl AdwApplicationImpl for VRexAppImp {}
