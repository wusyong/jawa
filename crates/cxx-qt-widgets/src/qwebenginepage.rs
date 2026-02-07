use crate::WidgetPtr;
pub use ffi::{QWebEnginePage, NavigationType};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qwebenginepermission.h");
        type QWebEnginePermission = crate::QWebEnginePermission;
        include!("cxx-qt-widgets/qwebengineprofile.h");
        type QWebEngineProfile = crate::QWebEngineProfile;
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;


        include!("cxx-qt-widgets/qwebenginepage.h");
        /// Represents the contents of a web page without a visual widget.
        #[qobject]
        type QWebEnginePage;

        /// Loads the given URL into the page.
        fn load(self: Pin<&mut QWebEnginePage>, url: &QUrl);

        /// Returns the currently loaded URL.
        fn url(self: &QWebEnginePage) -> QUrl;

        /// Returns the profile associated with this page.
        #[cxx_name = "profile"]
        fn profile_raw(self: &QWebEnginePage) -> *mut QWebEngineProfile;

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

    #[repr(u32)]
    #[derive(Debug)]
    enum NavigationType {
        NavigationTypeLinkClicked,
        NavigationTypeTyped,
        NavigationTypeFormSubmitted,
        NavigationTypeBackForward,
        NavigationTypeReload,
        NavigationTypeOther,
        NavigationTypeRedirect,
    }

    unsafe extern "C++" {
        type NavigationType;
    }
}

impl QWebEnginePage {
    /// Creates a new web engine page.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_page().into()
    }

    pub fn profile(&self) -> WidgetPtr<crate::QWebEngineProfile> {
        self.profile_raw().into()
    }
}
