// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Word count module.
//!
//! To handle the word count label.

use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, prelude::*,
    subclass::prelude::*, CompositeTemplate, Label,
};
#[allow(unused_imports)]
use log::*;
use std::default::Default;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/wordcount.ui")]
    /// Struct to handle word count label.
    pub struct TWWordCount {
        /// Inner label that holds the [`Label`].
        #[template_child]
        pub label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TWWordCount {
        const NAME: &'static str = "TWWordCount";
        type ParentType = gtk::Box;
        type Type = super::TWWordCount;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TWWordCount {}

    impl WidgetImpl for TWWordCount {}

    impl BoxImpl for TWWordCount {}
}

glib::wrapper! {
    pub struct TWWordCount(ObjectSubclass<imp::TWWordCount>)
        @extends gtk::Widget, gtk::Label;
}

impl Default for TWWordCount {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWWordCount object")
    }
}
