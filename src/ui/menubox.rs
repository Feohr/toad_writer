// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! MenuBox module.
//!
//! To handle the menu buttons in the Toolbar.
//! Buttons:
//!     1. [`TWFileButton`]
// //!     2. [`TWLicenseButton`]

mod file;
// mod license;

// use crate::ui::license::TWLicenseWindow;
use file::TWFileButton;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, // prelude::*,
    subclass::prelude::*, Box, CompositeTemplate,
};
// use license::TWLicenseButton;
// use log::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/menubox.ui")]
    /// Struct to handle the toolabar menu [`Button`]s.
    ///
    /// [`Button`] : [`gtk::Button`]
    pub struct TWMenuBox {
        /// A [`Button`] type to handle file IO operations.
        #[template_child]
        pub file: TemplateChild<TWFileButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWMenuBox {
        const NAME: &'static str = "TWMenuBox";
        type Type = super::TWMenuBox;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWMenuBox {}

    impl WidgetImpl for TWMenuBox {}

    impl BoxImpl for TWMenuBox {}
}

glib::wrapper! {
    pub struct TWMenuBox(ObjectSubclass<imp::TWMenuBox>)
        @extends gtk::Widget, gtk::HeaderBar,
        @implements gtk::gio::ActionMap, gtk::Buildable, gtk::gio::ActionGroup;
}

impl Default for TWMenuBox {
    fn default() -> Self {
        Object::new::<Self>()
    }
}
