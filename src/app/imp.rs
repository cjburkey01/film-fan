use crate::render::GliumGLArea;
use gtk::ffi::GtkBox;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::gio::SimpleAction;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::*;

#[derive(Debug, Default)]
pub struct FilmFanApp;

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
            .object("main_window")
            .expect("could not get object `window` from builder");
        window.set_application(Some(application));
        window.set_default_size(960, 540);
        window.set_title(Some(&format!(
            "{} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )));

        // Link callback to new project button
        let action_new = SimpleAction::new("project_new", None);
        action_new.connect_activate(clone!(@weak window => move |_, _| {
            println!("new project");
        }));
        window.add_action(&action_new);

        // Link callback to new project button
        let action_open = SimpleAction::new("project_open", None);
        action_open.connect_activate(clone!(@weak window => move |_, _| {
            println!("open project");
        }));
        window.add_action(&action_open);

        // Get the GL area container box
        let gl_area_box: Box = builder
            .object("gl_area_box")
            .expect("failed to get object `gl_area_box` from builder");

        // Add the GL area into the container
        let gl_area = GliumGLArea::new();
        gl_area.set_width_request(256);
        gl_area.set_height_request(256);
        gl_area.set_hexpand(true);
        gl_area.set_vexpand(true);
        gl_area_box.insert_child_after(&gl_area, Option::<&Box>::None);

        // Show the window
        window.present();
        window.grab_focus();
    }
}
impl GtkApplicationImpl for FilmFanApp {}
