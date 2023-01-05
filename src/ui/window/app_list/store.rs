use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::structs::App;
use crate::ui::window::app_list::app_row::AppRow;
use crate::ui::window::app_list::AppList;

impl AppList {
    pub fn clear(&self) {
        self.imp().store.remove_all();
    }

    pub fn add(&self, app: &App) {
        let row = AppRow::new();
        row.set_app_info(app.name(), app.version());
        self.imp().store.append(&row);
    }
}