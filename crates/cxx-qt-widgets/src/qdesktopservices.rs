use cxx_qt_lib::{QString, QUrl};
use cxx_qt::QObject;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qdesktopservices.h");
        type QObject = cxx_qt::QObject;
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;

        fn open_url(url: &QUrl) -> bool;

        unsafe fn set_url_handler(scheme: &QString, receiver: *mut QObject, method: &str);

        fn unset_url_handler(scheme: &QString);
    }
}

/// Helpers for launching URLs with the system default handler.
pub struct QDesktopServices;

impl QDesktopServices {
    /// Opens the given URL using the default application.
    pub fn open_url(url: &QUrl) -> bool {
        ffi::open_url(url)
    }

    /// Sets a handler for the given URL scheme.
    ///
    /// `method` should be the Qt slot signature, e.g. "openUrl(QUrl)".
    pub fn set_url_handler(scheme: &QString, receiver: *mut QObject, method: &str) {
        unsafe { ffi::set_url_handler(scheme, receiver, method) }
    }

    /// Clears the handler for the given URL scheme.
    pub fn unset_url_handler(scheme: &QString) {
        ffi::unset_url_handler(scheme)
    }
}
