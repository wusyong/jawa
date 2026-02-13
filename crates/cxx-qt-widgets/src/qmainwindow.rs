use std::pin::Pin;

use crate::{QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;
pub use ffi::QMainWindow;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!(<QtWidgets/QMainWindow>);
        type QWidget = crate::QWidget;

        /// Main application window, providing a menu bar, toolbars, and a central widget.
        #[qobject]
        #[base = QWidget]
        type QMainWindow;

        /// Sets the central widget for the main window.
        #[cxx_name = "setCentralWidget"]
        unsafe fn set_central_widget_raw(self: Pin<&mut QMainWindow>, widget: *mut QWidget);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_main_window() -> UniquePtr<QMainWindow>;
    }
}

impl ffi::QMainWindow {
    /// Creates a new main window without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_main_window().into()
    }

    pub fn set_central_widget<T: Upcast<QWidget> + UniquePtrTarget>(
        self: Pin<&mut QMainWindow>,
        widget: &mut WidgetPtr<T>,
    ) {
        widget.release();
        unsafe {
            self.set_central_widget_raw(widget.pin_mut().upcast_pin().get_unchecked_mut());
        }
    }
}
