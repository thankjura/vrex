use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/ru/slie/vrex/ui/window.ui")]
pub struct VRexWindowImp {
    #[template_child(id = "body")]
    pub body: TemplateChild<gtk::Box>,
    // #[template_child(id = "footer")]
    // pub footer: TemplateChild<gtk::Box>,
    //
    // #[template_child(id = "cover")]
    // pub cover: TemplateChild<gtk::Picture>,
}

#[glib::object_subclass]
impl ObjectSubclass for VRexWindowImp {
    const NAME: &'static str = "VRexWindow";
    type Type = super::VRexWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for VRexWindowImp {
    // fn constructed(&self) {
    //     self.parent_constructed();
    //     self.set_cover(None);
    // }
}

impl WidgetImpl for VRexWindowImp {}

impl WindowImpl for VRexWindowImp {}

impl ApplicationWindowImpl for VRexWindowImp {}
