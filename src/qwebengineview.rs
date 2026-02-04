use std::pin::Pin;

pub use ffi::QWebEngineView;

use crate::{QWebEnginePage, QWidget, WidgetPtr};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "C++Qt" {
        include!(<QWebEngineView>);
        type QWidget = crate::QWidget;
        type QWebEnginePage = crate::QWebEnginePage;

        /// Widget used to display and interact with web content.
        ///
        /// Wraps Qt's QWebEngineView, a widget for rendering web pages using
        /// the Qt WebEngine backend.
        #[qobject]
        #[base = QWidget]
        type QWebEngineView;

        /// Loads the given URL into the view.
        fn load(self: Pin<&mut QWebEngineView>, text: &QUrl);

        /// Returns the currently loaded URL.
        fn url(self: &QWebEngineView) -> QUrl;

        /// Returns the associated page object.
        #[cxx_name = "page"]
        fn page_raw(self: &QWebEngineView) -> *mut QWebEnginePage;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_web_engine_view() -> UniquePtr<QWebEngineView>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_web_engine_view_with_parent(parent: *mut QWidget) -> *mut QWebEngineView;
    }
}

impl QWebEngineView {
    /// Creates a new web engine view widget.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_view().into()
    }

    /// Creates a new web engine view widget with a parent.
    pub fn new_with_parent(parent: Pin<&mut QWidget>) -> WidgetPtr<Self> {
        unsafe { ffi::new_web_engine_view_with_parent(parent.get_unchecked_mut()).into() }
    }

    /// Returns the associated page object.
    ///
    /// The returned pointer is non-owning; the view manages its lifetime.
    pub fn page(&self) -> WidgetPtr<QWebEnginePage> {
        self.page_raw().into()
    }
}
