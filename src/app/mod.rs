mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct FilmFanApp(ObjectSubclass<imp::FilmFanApp>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for FilmFanApp {
    fn default() -> Self {
        Self::new()
    }
}

impl FilmFanApp {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &"com.cjburkey.film-fan"),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
        .expect("failed to create film-fan application")
    }
}
