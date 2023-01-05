use gtk::glib;
use gtk::prelude::BoxExt;
use gtk::subclass::prelude::*;

#[derive(Debug, Default)]
pub struct AppRowImp {
    pub(super) name: gtk::Label,
    pub(super) version: gtk::Label,
}

#[glib::object_subclass]
impl ObjectSubclass for AppRowImp {
    const NAME: &'static str = "AppRow";
    type Type = super::AppRow;
    type ParentType = gtk::Box;
}

impl ObjectImpl for AppRowImp {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.append(&self.name);
        obj.append(&self.version);
    }
}
impl WidgetImpl for AppRowImp {}
impl BoxImpl for AppRowImp {}