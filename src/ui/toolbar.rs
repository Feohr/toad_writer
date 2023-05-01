// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Toolbar module.
//!
//! Handles the toolbar UI. Main interface to handle all the toolbar button signals.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, CompositeTemplate,
};
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/toolbar.ui")]
    /// Main struct for toolbar handling.
    pub struct TWToolBar;

    #[glib::object_subclass]
    impl ObjectSubclass for TWToolBar {
        const NAME: &'static str = "TWToolBar";
        type ParentType = gtk::Box;
        type Type = super::TWToolBar;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWToolBar {}

    impl WidgetImpl for TWToolBar {}

    impl BoxImpl for TWToolBar {}
}

glib::wrapper! {
    pub struct TWToolBar(ObjectSubclass<imp::TWToolBar>)
        @extends gtk::Widget,
        @implements gtk::gio::ActionMap, gtk::gio::Menu, gtk::Buildable, gtk::gio::ActionGroup;
}

impl Default for TWToolBar {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWToolBar object")
    }
}
