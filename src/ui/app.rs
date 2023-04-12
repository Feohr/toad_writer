// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.
// SPDX-License-Identifier: BSD-3-Clause
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
// conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of
// conditions and the following disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to
// endorse or promote products derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use crate::{config, ui::window::TWApplicationWindow};
use gtk::{
    gdk::Display,
    gio::Application,
    glib,
    glib::{subclass::object::ObjectImpl, Object, WeakRef},
    prelude::*,
    subclass::prelude::*,
    CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use log::*;
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    pub struct TWApplication {
        pub window: WeakRef<super::TWApplicationWindow>,
    }

    #[gtk::glib::object_subclass]
    impl ObjectSubclass for TWApplication {
        const NAME: &'static str = "TWApplication";
        type Type = super::TWApplication;
        type ParentType = gtk::Application;

        fn new() -> Self {
            Self {
                window: WeakRef::default(),
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
            let window = {
                let data = None;
                self.obj().create_window(data)
            };
            info!(
                "Main Window: {{ id: {:?}, title: {:?} }}",
                window.id(),
                window.title()
            );

            // Set window as the main app window
            self.window.set(Some(&window));
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

    fn create_window(&self, _data: Option<i32>) -> TWApplicationWindow {
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
        StyleContext::add_provider_for_display(
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
