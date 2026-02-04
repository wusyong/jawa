use crate::WidgetPtr;
pub use ffi::QWebEnginePage;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "C++Qt" {
        include!("jawa/qwebenginepermission.h");
        type QWebEnginePermission = crate::QWebEnginePermission;

        include!(<QWebEngineView>);
        /// Represents the contents of a web page without a visual widget.
        #[qobject]
        type QWebEnginePage;

        /// Loads the given URL into the page.
        fn load(self: Pin<&mut QWebEnginePage>, url: &QUrl);

        /// Returns the currently loaded URL.
        fn url(self: &QWebEnginePage) -> QUrl;

        /// Sets the HTML content of the page with an optional base URL.
        #[cxx_name = "setHtml"]
        fn set_html(self: Pin<&mut QWebEnginePage>, html: &QString, base_url: &QUrl);

        #[qsignal]
        #[cxx_name = "permissionRequested"]
        unsafe fn permission_requested(
            self: Pin<&mut QWebEnginePage>,
            permission: QWebEnginePermission,
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_web_engine_page() -> UniquePtr<QWebEnginePage>;
    }
}

impl QWebEnginePage {
    /// Creates a new web engine page.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_page().into()
    }
}
