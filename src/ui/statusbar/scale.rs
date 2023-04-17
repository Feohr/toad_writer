// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::Object, prelude::*, subclass::prelude::*,
};
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct TWScale;

    #[glib::object_subclass]
    impl ObjectSubclass for TWScale {
        const NAME: &'static str = "TWScale";
        type ParentType = gtk::Scale;
        type Type = super::TWScale;
    }

    impl ObjectImpl for TWScale {}

    impl WidgetImpl for TWScale {}

    impl ScaleImpl for TWScale {}

    impl RangeImpl for TWScale {}
}

glib::wrapper! {
    pub struct TWScale(ObjectSubclass<imp::TWScale>)
        @extends gtk::Widget, gtk::Scale;
}

impl Default for TWScale {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWScale object")
    }
}
