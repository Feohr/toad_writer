// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Statusbar module.
//!
//! To handle status bar to display [`TWPage`] status.
//! ['TWPage'] : [`crate::ui::page::TWPage`]

mod wordcount;

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, CompositeTemplate,
};
use std::default::Default;
pub use wordcount::TWWordCount;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/statusbar.ui")]
    /// Struct to handle [`TWPage`] status bar.
    /// ['TWPage'] : [`crate::ui::page::TWPage`]
    pub struct TWStatusBar {
        /// To keep track word count in [`TWPage`].
        #[template_child]
        pub word_count: TemplateChild<TWWordCount>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWStatusBar {
        const NAME: &'static str = "TWStatusBar";
        type ParentType = gtk::Box;
        type Type = super::TWStatusBar;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWStatusBar {}

    impl WidgetImpl for TWStatusBar {}

    impl BoxImpl for TWStatusBar {}
}

glib::wrapper! {
    pub struct TWStatusBar(ObjectSubclass<imp::TWStatusBar>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for TWStatusBar {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWStatusBar object")
    }
}
