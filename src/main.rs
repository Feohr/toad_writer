// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

mod logger;
mod ui;

use anyhow::Result;
use gtk::glib;
use logger::init_logger;
use ui::app::TWApplication;

pub(crate) mod config {
    include!(concat!(env!("OUT_DIR"), "/tw_config.rs"));
}

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

fn main() -> Result<glib::ExitCode> {
    // Initialize logger
    init_logger()?;

    // Initialize GTK
    gtk::init()?;

    // Set default application name
    gtk::glib::set_application_name(config::APP_NAME);

    // Default icon name
    // gtk::Window::set_default_icon_name(config::APP_ID);
    // log::debug!("Default icon name: {:?}", gtk::Window::default_icon_name());

    // Default Theme
    if let Some(settings) = gtk::Settings::default() {
        settings.set_gtk_theme_name(Some("Adwaita"));
    }

    // Include GTK Composite Templates
    gtk::gio::resources_register_include!("compiled.gresource")?;

    // Run the main GTK application
    Ok(TWApplication::run())
}
