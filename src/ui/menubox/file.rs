// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! File button module.
//!
//! [`Button`] to handle file IO functions.
//! [`Button`] : [`gtk::Button`]

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::*, subclass::prelude::*,
    Box, CompositeTemplate, MenuButton,
};

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/menu/file.ui")]
    /// Struct to handle [`TWFileButton`].
    pub struct TWFileButton {
        /// To handle the actual button underneath.
        #[template_child]
        pub button: TemplateChild<MenuButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWFileButton {
        const NAME: &'static str = "TWFileButton";
        type Type = super::TWFileButton;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWFileButton {}

    impl WidgetImpl for TWFileButton {}

    impl BoxImpl for TWFileButton {}
}

glib::wrapper! {
    pub struct TWFileButton(ObjectSubclass<imp::TWFileButton>)
        @extends gtk::Widget;
}

impl Default for TWFileButton {
    fn default() -> Self {
        Object::new::<Self>()
    }
}
