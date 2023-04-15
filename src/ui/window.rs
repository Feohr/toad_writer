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

use crate::ui::menubox::TWMenuBox;
use crate::ui::page::TWPage;
use crate::ui::toolbar::TWToolBar;
use crate::{config, ui::app::TWApplication};
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, ApplicationWindow, CompositeTemplate, TextBuffer,
};
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/window.ui")]
    pub struct TWApplicationWindow {
        #[template_child(id = "main_toolbar")]
        pub toolbar: TemplateChild<TWToolBar>,
        #[template_child]
        pub header: TemplateChild<TWMenuBox>,
        #[template_child(id = "main_page")]
        pub page: TemplateChild<TWPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWApplicationWindow {
        const NAME: &'static str = "TWApplicationWindow";
        type ParentType = ApplicationWindow;
        type Type = super::TWApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl TWApplicationWindow {
        #[template_callback]
        fn text_changed(&self, page: &TextBuffer) {
            let textview = self.page.imp().obj();
            let mark = page.get_insert();
            textview.scroll_to_mark(&mark, 0_f64, true, 0.5_f64, 0.5_f64);
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
