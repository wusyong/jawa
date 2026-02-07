use crate::WidgetPtr;
pub use ffi::QWidget;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!(<QtWidgets/QWidget>);

        /// Base class of all user interface objects.
        #[qobject]
        type QWidget;

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QWidget>);

        /// Sets the window title for this widget.
        #[cxx_name = "setWindowTitle"]
        fn set_window_title(self: Pin<&mut QWidget>, title: &QString);

        /// Resizes the widget to the given size in pixels.
        #[cxx_name = "resize"]
        fn resize(self: Pin<&mut QWidget>, width: i32, height: i32);

        #[cxx_name = "setParent"]
        unsafe fn set_parent(self: Pin<&mut QWidget>, parent: *mut QWidget);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_widget() -> UniquePtr<QWidget>;
    }
}

impl ffi::QWidget {
    /// Creates a new widget without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_widget().into()
    }
}
