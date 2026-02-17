pub use ffi::QLayoutItem;

use crate::WidgetPtr;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qlayoutitem.h");
        type QWidget = crate::QWidget;

        /// Base class for layout items.
        type QLayoutItem;

        #[cxx_name = "widget"]
        fn widget_raw(self: &QLayoutItem) -> *mut QWidget;
    }

    impl UniquePtr<QLayoutItem> {}
}

impl ffi::QLayoutItem {
    pub fn widget(&self) -> Option<WidgetPtr<crate::QWidget>> {
        let widget = self.widget_raw();
        if widget.is_null() {
            None
        } else {
            Some(widget.into())
        }
    }
}
