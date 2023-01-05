use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ContentFit};
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::glib::Type;
use crate::ui::window::app_list::AppList;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/ru/slie/vrex/ui/window.ui")]
pub struct VRexWindowImp {
    #[template_child(id = "stack")]
    pub stack: TemplateChild<gtk::Stack>,

    #[template_child(id = "cover")]
    pub cover: TemplateChild<gtk::Picture>,

    #[template_child(id = "device_summary")]
    pub device_summary: TemplateChild<gtk::Label>,

    #[template_child(id = "app_list_box")]
    pub app_list_box: TemplateChild<AppList>,
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
    fn constructed(&self) {
        self.parent_constructed();
        self.cover.set_content_fit(ContentFit::Cover);
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![
            Signal::builder("device-changed")
                .param_types([Type::STRING]).build()
        ]);

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for VRexWindowImp {}

impl WindowImpl for VRexWindowImp {}

impl ApplicationWindowImpl for VRexWindowImp {}
