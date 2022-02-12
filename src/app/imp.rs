use gtk::builders::MenuButtonBuilder;
use gtk::gio::{Menu, SimpleAction};
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::*;

#[derive(Debug, Default)]
pub struct FilmFanApp {}

#[glib::object_subclass]
impl ObjectSubclass for FilmFanApp {
    const NAME: &'static str = "FilmFanApp";
    type Type = super::FilmFanApp;
    type ParentType = gtk::Application;
}

impl ObjectImpl for FilmFanApp {}
impl ApplicationImpl for FilmFanApp {
    fn activate(&self, application: &Self::Type) {
        let builder = gtk::Builder::from_string(include_str!("main.ui"));

        // Get the window template
        let window: ApplicationWindow = builder
            .object("window")
            .expect("could not get object `window` from builder.");
        window.set_application(Some(application));

        // Link callback to new project button
        let action_new = SimpleAction::new("project.new", None);
        action_new.connect_activate(clone!(@weak window => move |_, _| {
            println!("new project");
        }));
        window.add_action(&action_new);

        // Show the window
        window.present();
    }
}
impl GtkApplicationImpl for FilmFanApp {}
