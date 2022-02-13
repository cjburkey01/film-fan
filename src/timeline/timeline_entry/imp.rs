use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Value},
    prelude::*,
};
use std::cell::RefCell;

#[derive(Default)]
pub struct TimelineEntry {
    track_id: RefCell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for TimelineEntry {
    const NAME: &'static str = "TimelineEntry";
    type Type = super::TimelineEntry;
}

impl ObjectImpl for TimelineEntry {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecUInt::new(
                "track_id",
                "TrackID",
                "Track ID",
                0,
                128,
                0,
                glib::ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "track_id" => {
                let track_id = value.get().unwrap();
                self.track_id.replace(track_id);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.track_id.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
