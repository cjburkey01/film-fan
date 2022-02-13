mod imp;

use gtk::glib;
use gtk::glib::Object;

glib::wrapper! {
    pub struct TimelineEntry(ObjectSubclass<imp::TimelineEntry>);
}

impl TimelineEntry {
    pub fn new(track_id: u32) -> Self {
        Object::new(&[("track_id", &track_id)]).expect("failed to create `TimelineEntry`")
    }
}
