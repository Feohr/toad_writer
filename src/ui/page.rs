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

mod dimensions;

use dimensions::*;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::Object, glib::WeakRef, prelude::*,
    subclass::prelude::*, ScrolledWindow, TextView,
};
use log::*;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct TWPage {
        pub count: usize,
        pub scroll_parent: WeakRef<ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWPage {
        const NAME: &'static str = "TWPage";
        type ParentType = TextView;
        type Type = super::TWPage;

        fn new() -> Self {
            Self {
                count: 0_usize,
                scroll_parent: WeakRef::new(),
            }
        }
    }

    impl ObjectImpl for TWPage {
        fn constructed(&self) {
            let obj = self.obj();

            // Get default height and width
            let resolution = gtk::PrintSettings::new().resolution();
            let dimensions = ISODimensions::default()
                .get()
                .iter()
                .map(|dim| (dim * resolution as f64) as i32)
                .collect::<Vec<i32>>();

            // Extracting values
            let (height, width) = (dimensions[0], dimensions[1]);
            info!(
                "Untitled file hence default page size set {} X {}",
                height, width
            );

            // Set page size
            obj.set_size_request(height, width);
        }
    }

    impl WidgetImpl for TWPage {
        fn map(&self) {
            self.parent_map();

            // Add a reference of the scroll window
            let obj = self.obj();
            let Some(scroll) = obj.parent()
                .and_then(|widget| widget.parent())
                .and_then(|widget| widget.parent()) else { return };
            debug!("TWPage parent: {:?}", scroll);
            // Downcasting widget
            let Ok(scroll) = scroll.downcast::<ScrolledWindow>() else { return };
            self.scroll_parent.set(Some(&scroll));
        }
    }

    impl TextViewImpl for TWPage {}
}

glib::wrapper! {
    pub struct TWPage(ObjectSubclass<imp::TWPage>)
        @extends gtk::Widget, gtk::TextView;
}

impl Default for TWPage {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWPage object")
    }
}
