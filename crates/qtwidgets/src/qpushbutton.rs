use crate::WidgetPtr;
pub use ffi::QPushButton;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!(<QtWidgets/QPushButton>);

        #[qobject]
        type QPushButton;

        // TODO: we should use upcasting methods here and implement QAbstractButton and QWidget
        // so that we don't need to duplicate all of the methods

        /// This signal is emitted when the button is activated (i.e., pressed down then released while the mouse cursor is inside the button)
        #[qsignal]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);

        /// Set the text shown on the button
        #[cxx_name = "setText"]
        fn set_text(self: Pin<&mut QPushButton>, text: &QString);

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QPushButton>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_push_button() -> UniquePtr<QPushButton>;
    }
}

impl ffi::QPushButton {
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_push_button().into()
    }
}
