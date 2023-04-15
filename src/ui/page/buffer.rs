// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::Object, prelude::*, subclass::prelude::*,
    TextBuffer,
};
#[allow(unused_imports)]
use log::*;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TWBuffer;

    #[glib::object_subclass]
    impl ObjectSubclass for TWBuffer {
        const NAME: &'static str = "TWBuffer";
        type ParentType = TextBuffer;
        type Type = super::TWBuffer;
    }

    impl ObjectImpl for TWBuffer {}

    impl WidgetImpl for TWBuffer {}

    impl TextBufferImpl for TWBuffer {}
}

glib::wrapper! {
    pub struct TWBuffer(ObjectSubclass<imp::TWBuffer>)
        @extends gtk::Widget, gtk::TextBuffer;
}

impl Default for TWBuffer {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWBuffer object")
    }
}
