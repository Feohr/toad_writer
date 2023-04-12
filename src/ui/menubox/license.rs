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

use crate::ui::license::TWLicenseWindow;
use gtk::{
    glib, glib::object::*, glib::subclass::object::ObjectImpl, glib::Object, subclass::prelude::*,
    Button,
};

mod imp {
    use super::*;

    // Define the TWLicenseButton struct
    #[derive(Debug, Default)]
    pub struct TWLicenseButton {
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
