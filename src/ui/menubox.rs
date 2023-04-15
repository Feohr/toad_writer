// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

mod file;
mod license;

use crate::ui::license::TWLicenseWindow;
use file::TWFileButton;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, Box, CompositeTemplate,
};
use license::TWLicenseButton;
use log::*;

mod imp {
    use super::*;

    // Define the TWMenuBox struct
    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/menubox.ui")]
    pub struct TWMenuBox {
        #[template_child]
        pub file: TemplateChild<TWFileButton>,
        #[template_child]
        pub license: TemplateChild<TWLicenseButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWMenuBox {
        const NAME: &'static str = "TWMenuBox";
        type Type = super::TWMenuBox;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl TWMenuBox {
        #[template_callback]
        fn license_button_clicked(button: &TWLicenseButton) {
            debug!("license button clicked");
            let window = button.imp().window.upgrade().unwrap_or_else(|| {
                debug!("License window closed. Creating new window.");
                // Add new window if no window present
                let window = TWLicenseWindow::new();
                button.imp().window.set(Some(&window));
                window
            });

            // Window visibility switch
            let visible = window.is_visible();
            window.set_visible(!visible);
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
