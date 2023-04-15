// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

mod buffer;
mod dimensions;

use buffer::TWBuffer;
use dimensions::*;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, CompositeTemplate, TextBuffer, TextView,
};
#[allow(unused_imports)]
use log::*;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/page.ui")]
    pub struct TWPage {
        pub count: usize,
        pub size: Pixels,
        #[template_child]
        pub buffer: TemplateChild<TWBuffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWPage {
        const NAME: &'static str = "TWPage";
        type ParentType = TextView;
        type Type = super::TWPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl TWPage {
        #[template_callback]
        fn text_changed(&self, page: &TextBuffer) {
            let textview = self.obj();
            let mark = page.get_insert();
            textview.scroll_to_mark(&mark, 0_f64, true, 0.5_f64, 0.5_f64);
        }
    }

    impl ObjectImpl for TWPage {}

    impl WidgetImpl for TWPage {}

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
