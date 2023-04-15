// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, CompositeTemplate, TextView, Window,
};
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/license.ui")]
    pub struct TWLicenseWindow {
        #[template_child]
        pub text: TemplateChild<TextView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWLicenseWindow {
        const NAME: &'static str = "TWLicenseWindow";
        type ParentType = Window;
        type Type = super::TWLicenseWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWLicenseWindow {}

    impl WidgetImpl for TWLicenseWindow {}

    impl WindowImpl for TWLicenseWindow {}

    impl DialogImpl for TWLicenseWindow {}
}

glib::wrapper! {
    pub struct TWLicenseWindow(ObjectSubclass<imp::TWLicenseWindow>)
        @extends gtk::Widget, gtk::Window,
        @implements gtk::gio::ActionMap, gtk::Buildable, gtk::gio::ActionGroup;
}

impl TWLicenseWindow {
    pub fn new() -> Self {
        Object::new::<Self>()
    }
}

impl Default for TWLicenseWindow {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWLicenseWindow object")
    }
}
