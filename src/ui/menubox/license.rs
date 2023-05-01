// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! license window module.
//!
//! To handle [`TWLicenseWindow`] functions.

use crate::ui::license::TWLicenseWindow;
use gtk::{
    glib, glib::object::*, glib::subclass::object::ObjectImpl, glib::Object, subclass::prelude::*,
    Button,
};

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    /// Struct to handle [`TWLicenseButton`].
    pub struct TWLicenseButton {
        /// [`WeakRef`] to [`TWLicenseWindow`] to create and open window.
        pub window: WeakRef<TWLicenseWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWLicenseButton {
        const NAME: &'static str = "TWLicenseButton";
        type Type = super::TWLicenseButton;
        type ParentType = Button;

        fn new() -> Self {
            let window = WeakRef::new();
            Self { window }
        }
    }

    impl ObjectImpl for TWLicenseButton {
        fn constructed(&self) {
            let window = TWLicenseWindow::new();
            self.window.set(Some(&window));
        }
    }

    impl WidgetImpl for TWLicenseButton {}

    impl ButtonImpl for TWLicenseButton {}
}

glib::wrapper! {
    pub struct TWLicenseButton(ObjectSubclass<imp::TWLicenseButton>)
        @extends gtk::Widget, gtk::Button,
        @implements gtk::gio::ActionMap, gtk::Buildable, gtk::gio::ActionGroup;
}

impl Default for TWLicenseButton {
    fn default() -> Self {
        Object::new::<Self>()
    }
}
