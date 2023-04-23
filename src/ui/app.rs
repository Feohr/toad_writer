// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use crate::{config, ui::window::TWApplicationWindow};
use gtk::{
    gdk::Display,
    gio::Application,
    glib,
    glib::{subclass::object::ObjectImpl, Object},
    prelude::*,
    subclass::prelude::*,
    CssProvider, style_context_add_provider_for_display, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use log::*;
use once_cell::sync::OnceCell;
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    pub struct TWApplication {
        window: OnceCell<super::TWApplicationWindow>,
    }

    #[gtk::glib::object_subclass]
    impl ObjectSubclass for TWApplication {
        const NAME: &'static str = "TWApplication";
        type Type = super::TWApplication;
        type ParentType = gtk::Application;

        fn new() -> Self {
            Self {
                window: OnceCell::new(),
            }
        }
    }

    impl ObjectImpl for TWApplication {}

    impl WidgetImpl for TWApplication {}

    impl ApplicationImpl for TWApplication {
        fn activate(&self) {
            // Initialize CSS
            super::TWApplication::init_css("/com/github/feohr/ToadWriter/style.css");

            // Main window
            let window = self.obj().create_window();
            info!(
                "Main Window: {{ id: {:?}, title: {:?} }}",
                window.id(),
                window.title()
            );

            // Set window as the main app window
            let Ok(_) = self.window.set(window) else {
                error!("Error while setting the application window");
                return
            };
        }
    }

    impl GtkApplicationImpl for TWApplication {}
}

glib::wrapper! {
        pub struct TWApplication(ObjectSubclass<imp::TWApplication>)
        @extends gtk::Application, gtk::gio::Application,
        @implements gtk::gio::ActionMap, gtk::gio::ActionGroup;
}

impl TWApplication {
    pub fn run() -> glib::ExitCode {
        // Debug information
        debug!(
            "\n\tApplication\x20Name:\t{},\n\
            \tApplication\x20ID:\t\t{:?},\n\
            \tBase\x20ID:\t\t{:?},\n\
            \tPath\x20ID:\t\t{:?},",
            config::APP_NAME,
            config::APP_ID,
            config::BASE_ID,
            config::PATH_ID,
        );

        // Main application
        let app = Object::builder::<TWApplication>()
            .property("application-id", Some(config::APP_ID))
            .build();
        // Application info log
        info!("Application built: {:?}", app.type_());
        info!("Resolution {}", gtk::PrintSettings::new().resolution());

        // Run the main application
        app.run()
    }

    fn create_window(&self) -> TWApplicationWindow {
        // Main window
        let window = TWApplicationWindow::new(self.clone());

        // Default window attributes
        window.maximize();
        window.present();

        window
    }

    fn init_css(path: &str) {
        // Get the CssProvider
        let provider = CssProvider::new();
        provider.load_from_resource(path);

        // Get default display
        let display = Display::default().expect("Cannot get default display");

        // Setting StyleContext
        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

impl Default for TWApplication {
    fn default() -> Self {
        Application::default()
            .expect("Error while creating default application")
            .downcast()
            .expect("Error while downcasting application")
    }
}
