use crate::config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR};
use crate::ui::VRexApp;
use gettextrs::{
    bind_textdomain_codeset, bindtextdomain, gettext, setlocale, textdomain, LocaleCategory,
};
use gtk::gio;
use gtk::prelude::ApplicationExtManual;

mod config;
mod core;
mod ui;

#[cfg(not(debug_assertions))]
fn load_resources() {
    use config::LOCALEDIR;
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/vrex.gresource")
        .expect(&gettext("Could not load resources"));
    gio::resources_register(&resources);
}

#[cfg(debug_assertions)]
fn load_resources() {
    Result::expect(
        gio::resources_register_include!("vrex.gresource"),
        &gettext("Could not load resources"),
    );
}

fn main() {
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect(&gettext("Unable to bind the text domain"));
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect(&gettext("Unable to set the text domain encoding"));
    textdomain(GETTEXT_PACKAGE).expect(&gettext("Unable to switch to the text domain"));

    load_resources();

    let app = VRexApp::new(APP_ID);
    std::process::exit(app.run());
}
