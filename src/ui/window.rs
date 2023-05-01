// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Window module.
//!
//! The main application window.

use crate::ui::menubox::TWMenuBox;
use crate::ui::page::TWPage;
use crate::ui::statusbar::TWStatusBar;
use crate::ui::toolbar::TWToolBar;
use crate::{config, ui::app::TWApplication};
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, ApplicationWindow, CompositeTemplate,
};
#[allow(unused_imports)]
use log::*;
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/window.ui")]
    /// An [`ApplicationWindow`] type struct to handle the main application window.
    pub struct TWApplicationWindow {
        /// For main headerbar buttons.
        #[template_child]
        pub header: TemplateChild<TWMenuBox>,
        /// Toolbar object.
        #[template_child(id = "main_toolbar")]
        pub toolbar: TemplateChild<TWToolBar>,
        /// Main [`TextView`] object.
        /// [`TextView`] : [`gtk::TextView`]
        #[template_child(id = "main_page")]
        pub page: TemplateChild<TWPage>,
        /// Main Statusbar object.
        #[template_child]
        pub statusbar: TemplateChild<TWStatusBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWApplicationWindow {
        const NAME: &'static str = "TWApplicationWindow";
        type ParentType = ApplicationWindow;
        type Type = super::TWApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWApplicationWindow {}

    impl WidgetImpl for TWApplicationWindow {}

    impl WindowImpl for TWApplicationWindow {}

    impl ApplicationWindowImpl for TWApplicationWindow {}
}

glib::wrapper! {
    pub struct TWApplicationWindow(ObjectSubclass<imp::TWApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::gio::ActionMap, gtk::gio::Menu, gtk::Buildable, gtk::gio::ActionGroup;
}

impl TWApplicationWindow {
    /// Takes an [`TWApplication`] adds a new window to it and returns the [`TWApplicationWindow`].
    pub fn new(app: TWApplication) -> Self {
        let window = Object::builder()
            .property("title", Some(config::APP_NAME))
            .build();
        app.add_window(&window);
        window
    }
}

impl Default for TWApplicationWindow {
    fn default() -> Self {
        TWApplication::default()
            .active_window()
            .expect("There is no active window in application")
            .downcast()
            .expect("Error while downcasting default application window")
    }
}
