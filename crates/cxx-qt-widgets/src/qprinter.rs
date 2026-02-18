pub use ffi::QPrinter;

use cxx::UniquePtr;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qprinter.h");

        type QPagedPaintDevice = crate::QPagedPaintDevice;

        #[qobject]
        #[base = QPagedPaintDevice]
        type QPrinter;

        #[cxx_name = "setResolution"]
        fn set_resolution(self: Pin<&mut QPrinter>, resolution: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_printer() -> UniquePtr<QPrinter>;
    }
}

unsafe impl Send for ffi::QPrinter {}
unsafe impl Sync for ffi::QPrinter {}

impl ffi::QPrinter {
    /// Creates a new printer without a parent.
    pub fn new() -> UniquePtr<Self> {
        ffi::new_printer()
    }
}
