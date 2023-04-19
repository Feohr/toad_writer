// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::Object, prelude::*, subclass::prelude::*,
    TextBuffer, TextIter,
};
#[allow(unused_imports)]
use log::*;

const TAB: &'static str = "\x09";
const SPACE: &'static str = "\x20";

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct TWBuffer {
        pub tab_size: usize,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWBuffer {
        const NAME: &'static str = "TWBuffer";
        type ParentType = TextBuffer;
        type Type = super::TWBuffer;

        fn new() -> Self {
            Self { tab_size: 4_usize }
        }
    }

    impl ObjectImpl for TWBuffer {}

    impl WidgetImpl for TWBuffer {}

    impl TextBufferImpl for TWBuffer {
        fn insert_text(&self, iter: &mut TextIter, new_text: &str) {
            self.parent_insert_text(iter, &new_text.replace(TAB, &SPACE.repeat(self.tab_size)));
        }
    }
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
