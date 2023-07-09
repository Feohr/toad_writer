// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Page module.
//!
//! To handle the [`TextView`] object of the application.

mod buffer;
mod dimensions;

use crate::ui::statusbar::TWWordCount;
pub use buffer::TWBuffer;
use dimensions::*;
use gtk::{
    glib, glib::subclass::object::ObjectImpl, glib::subclass::*, glib::Object, glib::*, prelude::*,
    subclass::prelude::*, CompositeTemplate, TextBuffer, TextView,
};
#[allow(unused_imports)]
use log::*;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/feohr/ToadWriter/page.ui")]
    /// Struct that holds the main [`TextView`] data of the application.
    pub struct TWPage {
        /// To keep track of number of pages that will be produced in the final compilation.
        pub count: usize,
        /// To keep track of page dimensions.
        pub size: Pixels,
        /// To get a reference to [`TWWordCount`] label and update it accordingly.
        pub count_label: WeakRef<TWWordCount>,
        /// The main [`TextView`] buffer.
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
        /// To move the view to follow the cursor in [`TWPage`].
        #[template_callback]
        fn scroll_to_cursor(&self, page: &TextBuffer) {
            let mark = page.get_insert();
            self.obj()
                .scroll_to_mark(&mark, 0_f64, true, 0.5_f64, 0.5_f64);
        }

        #[template_callback]
        fn update_count_label(&self, buffer: &TextBuffer) {
            let Some(wordcount) = self.count_label.upgrade() else {
                warn!("TWWordCount reference not set. Trying to set it again.");
                self.obj().set_count_label_reference();
                return
            };

            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, true);

            wordcount
                .imp()
                .label
                .set_label(format!("{}", self.obj().parse_word_count(text.as_str())).as_str());
        }
    }

    impl ObjectImpl for TWPage {}

    impl WidgetImpl for TWPage {
        /// Ran when the textview is mapped to the screen.
        fn map(&self) {
            self.parent_map();
            self.obj().set_count_label_reference();
        }
    }

    impl TextViewImpl for TWPage {}
}

glib::wrapper! {
    pub struct TWPage(ObjectSubclass<imp::TWPage>)
        @extends gtk::Widget, gtk::TextView;
}

impl TWPage {
    /// A simple set function to fetch and set the reference to [`TWWordCount`].
    fn set_count_label_reference(&self) {
        let Some(wordcount) = self.count_label_reference() else {
            error!("Error while getting TWWordCount reference from TWPage");
            return
        };
        self.imp().count_label.set(Some(&wordcount));
    }

    /// A simple function to get the reference to [`TWWordCount`].
    fn count_label_reference(&self) -> Option<TWWordCount> {
        self.parent()
            .and_then(|scrolled| scrolled.parent())
            .and_then(|gtkbox| gtkbox.last_child())
            .and_then(|statbar| statbar.first_child())
            .and_then(|wordcount| {
                if let Ok(wordcount) = wordcount.downcast::<TWWordCount>() {
                    return Some(wordcount);
                }
                None
            })
    }

    /// Takes a buffer and calculates the word count.
    fn parse_word_count(&self, text: &str) -> usize {
        let mut word_count = 0_usize;
        let mut is_white = true;

        text
            .chars()
            .for_each(|ch| {
                if ch.is_whitespace() || ch.is_ascii_punctuation() {
                    is_white = true;
                } else if is_white {
                    is_white = false;
                    word_count += 1_usize;
                }
            });

        word_count
    }
}

impl Default for TWPage {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWPage object")
    }
}
