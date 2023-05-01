// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Scale module.
//!
//! To handle the scaling of the textview.

use crate::ui::page::TWBuffer;
use crate::ui::page::TWPage;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::Object, glib::WeakRef, prelude::*,
    subclass::prelude::*, ScrollType, TextTag,
};
#[allow(unused_imports)]
use log::*;
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default)]
    /// Struct to handle the Scaler.
    pub struct TWScale {
        /// Zoom tag that applies to the [`TWBuffer`] iter.
        pub zoom_tag: WeakRef<TextTag>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWScale {
        const NAME: &'static str = "TWScale";
        type ParentType = gtk::Scale;
        type Type = super::TWScale;
    }

    impl ObjectImpl for TWScale {}

    impl WidgetImpl for TWScale {
        fn map(&self) {
            self.parent_map();
            self.obj().set_tag_reference();
        }
    }

    impl ScaleImpl for TWScale {}

    impl RangeImpl for TWScale {
        /// Called every time scroll bar value is changed. Zoom tag is updated to scale.
        fn change_value(&self, scroll_type: ScrollType, new_value: f64) -> bool {
            // Set scale factor to new_value.
            match self.zoom_tag.upgrade() {
                Some(tag) => tag.set_scale(new_value / 50_f64),
                None => {
                    warn!("Cannot get zoom tag reference as it is not set");
                    self.obj().set_tag_reference();
                }
            }

            // Return bool from parent.
            self.parent_change_value(scroll_type, new_value)
        }
    }
}

glib::wrapper! {
    pub struct TWScale(ObjectSubclass<imp::TWScale>)
        @extends gtk::Widget, gtk::Scale;
}

impl TWScale {
    /// To set the tag reference.
    fn set_tag_reference(&self) {
        // Getting the zoom and buffer references
        let Some(buffer) = self.textview_buffer() else {
            error!("Couldn't get buffer reference");
            return
        };
        let Some(tag) = buffer.create_tag(Some("zoom"), &[]) else {
            error!("Couldn't get zoom tag reference");
            return
        };

        // Set a local reference for tag
        self.imp().zoom_tag.set(Some(&tag));
        // Set the buffer to apply tag when changed
        buffer.connect_changed(move |buffer| {
            buffer.apply_tag(&tag, &buffer.end_iter(), &buffer.start_iter());
        });
    }

    /// Returns [`TWBuffer`] reference.
    fn textview_buffer(&self) -> Option<TWBuffer> {
        self.parent()
            .and_then(|statbar| statbar.parent())
            .and_then(|gtkbox| gtkbox.first_child())
            .and_then(|scrolled| scrolled.first_child())
            .and_then(|textview| {
                if let Ok(page) = textview.downcast::<TWPage>() {
                    return Some(page);
                }
                None
            })
            .and_then(|textview| Some(textview.buffer()))
            .and_then(|buffer| {
                if let Ok(buffer) = buffer.downcast::<TWBuffer>() {
                    return Some(buffer);
                }
                None
            })
    }
}

impl Default for TWScale {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWScale object")
    }
}
