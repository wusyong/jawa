use crate::WidgetPtr;

pub use ffi::QLineEdit;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qlineedit.h");
        type QWidget = crate::QWidget;

        /// A one-line text editor.
        #[qobject]
        #[base = QWidget]
        type QLineEdit;

        /// Set the current text.
        #[cxx_name = "setText"]
        fn set_text(self: Pin<&mut QLineEdit>, text: &QString);

        /// Returns the line edit's text.
        fn text(self: &QLineEdit) -> QString;

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QLineEdit>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_line_edit() -> UniquePtr<QLineEdit>;
    }
}

impl ffi::QLineEdit {
    /// Creates a new line edit without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_line_edit().into()
    }
}
