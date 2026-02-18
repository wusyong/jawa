use std::pin::Pin;

pub use ffi::QPrintPreviewDialog;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;

use crate::{QPrinter, QWidget};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qprintpreviewdialog.h");

        type QDialog = crate::QDialog;
        type QPrinter = crate::QPrinter;
        type QWidget = crate::QWidget;

        #[qobject]
        #[base = QDialog]
        type QPrintPreviewDialog;

        fn exec(self: Pin<&mut QPrintPreviewDialog>) -> i32;

        #[qsignal]
        #[cxx_name = "paintRequested"]
        unsafe fn paint_requested(self: Pin<&mut QPrintPreviewDialog>, printer: *mut QPrinter);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        unsafe fn new_print_preview_dialog(
            printer: *mut QPrinter,
            parent: *mut QWidget,
        ) -> UniquePtr<QPrintPreviewDialog>;
    }
}

impl ffi::QPrintPreviewDialog {
    /// Creates a new print preview dialog with the specified printer and parent.
    pub fn new<T: Upcast<QWidget>>(printer: *mut QPrinter, parent: Pin<&mut T>) -> UniquePtr<Self> {
        unsafe { ffi::new_print_preview_dialog(printer, parent.upcast_pin().get_unchecked_mut()) }
    }
}
