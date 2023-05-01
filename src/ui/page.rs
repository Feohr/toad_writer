// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

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
    pub struct TWPage {
        pub count: usize,
        pub size: Pixels,
        pub count_label: WeakRef<TWWordCount>,
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
        fn scroll_to_cursor(&self, page: &TextBuffer) {
            let mark = page.get_insert();
            self.obj()
                .scroll_to_mark(&mark, 0_f64, true, 0.5_f64, 0.5_f64);
        }

        #[template_callback]
        fn count_words(&self, page: &TextBuffer) {
            let Some(wordcount) = self.count_label.upgrade() else {
                warn!("TWWordCount reference not set. Trying to set it again.");
                self.obj().set_count_label_reference();
                return
            };

            // Count the number of words.
            let startiter = page.start_iter();
            let enditer = page.end_iter();
            let count = page
                .text(&startiter, &enditer, false)
                .split_whitespace()
                .filter_map(|tokens| {
                    let trimmed = tokens
                        .chars()
                        .filter(|ch| !ch.is_ascii_punctuation())
                        .collect::<String>();
                    if trimmed.is_empty() {
                        return None;
                    }
                    Some(trimmed)
                })
                .count();

            wordcount
                .imp()
                .label
                .set_label(format!("{}", count).as_str());
        }
    }

    impl ObjectImpl for TWPage {}

    impl WidgetImpl for TWPage {
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
    fn set_count_label_reference(&self) {
        let Some(wordcount) = self.count_label_reference() else {
            error!("Error while getting TWWordCount reference from TWPage");
            return
        };
        self.imp().count_label.set(Some(&wordcount));
    }

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
}

impl Default for TWPage {
    fn default() -> Self {
        Object::new::<Self>()
            .downcast()
            .expect("Error while downcasting TWPage object")
    }
}
