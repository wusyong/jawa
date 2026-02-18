use std::pin::Pin;

pub use ffi::QWebEngineView;

use crate::{QWebEnginePage, QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

#[cxx_qt::bridge]
mod ffi {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qwebengineview.h");
        type QWidget = crate::QWidget;
        type QWebEnginePage = crate::QWebEnginePage;
        type QPrinter = crate::QPrinter;

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

        /// Sets the page object to be used by this view.
        #[cxx_name = "setPage"]
        unsafe fn set_page_raw(self: Pin<&mut QWebEngineView>, page: *mut QWebEnginePage);

        /// Returns the associated page object.
        #[cxx_name = "page"]
        fn page_raw(self: &QWebEngineView) -> *mut QWebEnginePage;

        #[qsignal]
        #[cxx_name = "printRequested"]
        fn print_requested(self: Pin<&mut QWebEngineView>);

        #[qsignal]
        #[cxx_name = "printFinished"]
        fn print_finished(self: Pin<&mut QWebEngineView>, success: bool);

        /// Renders the current content of the page into a temporary PDF document, then prints it using printer.
        unsafe fn print(self: Pin<&mut QWebEngineView>, printer: *mut QPrinter);
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

unsafe impl Send for ffi::QWebEngineView {}
unsafe impl Sync for ffi::QWebEngineView {}

impl QWebEngineView {
    /// Creates a new web engine view widget.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_view().into()
    }

    /// Creates a new web engine view widget with a parent.
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe {
            ffi::new_web_engine_view_with_parent(parent.upcast_pin().get_unchecked_mut()).into()
        }
    }

    /// Returns the associated page object.
    ///
    /// The returned pointer is non-owning; the view manages its lifetime.
    pub fn page(&self) -> WidgetPtr<QWebEnginePage> {
        self.page_raw().into()
    }

    /// Sets the page object to be used by this view.
    pub fn set_page<T: Upcast<QWebEnginePage> + UniquePtrTarget>(
        self: Pin<&mut QWebEngineView>,
        page: &mut WidgetPtr<T>,
    ) {
        page.release();
        unsafe { self.set_page_raw(page.pin_mut().upcast_pin().get_unchecked_mut()) }
    }
}
