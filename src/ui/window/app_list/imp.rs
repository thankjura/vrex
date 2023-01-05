use adw::gio;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use crate::ui::window::app_list::app_row::AppRow;

pub struct AppListImp {
    pub(super) store: gio::ListStore,
    selector: gtk::SingleSelection,
    factory: gtk::SignalListItemFactory,
    list_view: gtk::ListView,
    container: gtk::ScrolledWindow,
}

impl Default for AppListImp {
    fn default() -> Self {
        let store = gio::ListStore::new(AppRow::static_type());
        let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
            let app_info1 = obj1.downcast_ref::<AppRow>().unwrap();
            let app_info2 = obj2.downcast_ref::<AppRow>().unwrap();
            app_info1.name().to_lowercase().cmp(&app_info2.name().to_lowercase()).into()
        });
        let sorted_model = gtk::SortListModel::new(Some(&store), Some(&sorter));
        let selector = gtk::SingleSelection::new(Some(&sorted_model));
        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = AppRow::new();
            item.set_child(Some(&row));
        });

        factory.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let app_row = item.item().unwrap().downcast::<AppRow>().unwrap();
            let child = item.child().unwrap().downcast::<AppRow>().unwrap();
            child.set_app_info(&app_row.name(), &app_row.version());
        });

        let list_view = gtk::ListView::new(Some(&selector), Some(&factory));
        let container = gtk::ScrolledWindow::new();
        container.set_child(Some(&list_view));
        container.set_vexpand(true);

        Self {
            store,
            selector,
            factory,
            list_view,
            container,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AppListImp {
    const NAME: &'static str = "AppList";
    type Type = super::AppList;
    type ParentType = gtk::Box;
}


impl ObjectImpl for AppListImp {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().append(&self.container);
    }

    // fn signals() -> &'static [Signal] {
    //     static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![
    //         Signal::builder("device-changed")
    //             .param_types([Type::STRING]).build()
    //     ]);
    //
    //     SIGNALS.as_ref()
    // }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for AppListImp {}

impl BoxImpl for AppListImp {}