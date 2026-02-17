use std::pin::Pin;

use crate::{QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;
pub use ffi::{QDialog, DialogCode};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qdialog.h");
        type QWidget = crate::QWidget;

        /// Base class for dialog windows.
        #[qobject]
        #[base = QWidget]
        type QDialog;

        /// Executes the dialog modally and returns the result.
        #[cxx_name = "exec"]
        fn exec(self: Pin<&mut QDialog>) -> i32;

        /// Closes the dialog and sets the result code to Accepted.
        fn accept(self: Pin<&mut QDialog>);

        /// Closes the dialog and sets the result code to Rejected.
        fn reject(self: Pin<&mut QDialog>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_dialog() -> UniquePtr<QDialog>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_dialog_with_parent(parent: *mut QWidget) -> *mut QDialog;
    }

    /// Policy for persistent cookies.
    #[repr(u32)]
    #[derive(Debug)]
    enum DialogCode {
        Rejected = 0,
        Accepted = 1,
    }

    unsafe extern "C++" {
        type DialogCode;
    }
}

impl ffi::QDialog {
    /// Creates a new dialog without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_dialog().into()
    }

    /// Creates a new dialog with a parent.
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe { ffi::new_dialog_with_parent(parent.upcast_pin().get_unchecked_mut()).into() }
    }
}
