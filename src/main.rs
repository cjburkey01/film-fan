mod app;

use app::*;
use gtk::prelude::*;

fn main() {
    let app = FilmFanApp::new();
    std::process::exit(app.run());
}
