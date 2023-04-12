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
