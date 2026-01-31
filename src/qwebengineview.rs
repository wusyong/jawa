use crate::WidgetPtr;
pub use ffi::QWebEngineView;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "C++Qt" {
        include!(<QWebEngineView>);

        #[qobject]
        type QWebEngineView;

        fn load(self: Pin<&mut QWebEngineView>, text: &QUrl);

        fn show(self: Pin<&mut QWebEngineView>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_web_engine_view() -> UniquePtr<QWebEngineView>;
    }
}

impl QWebEngineView {
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_view().into()
    }
}
