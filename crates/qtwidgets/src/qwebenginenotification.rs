pub use ffi::QWebEngineNotification;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qurl.h");
        type QString = cxx_qt_lib::QString;
        type QUrl = cxx_qt_lib::QUrl;
    }

    unsafe extern "C++Qt" {
        include!(<QWebEngineNotification>);

        /// Notification object emitted by the web engine.
        #[qobject]
        type QWebEngineNotification;

        /// Shows the notification.
        fn show(self: &QWebEngineNotification);

        /// Closes the notification.
        fn close(self: &QWebEngineNotification);

        /// Returns the notification title.
        fn title(self: &QWebEngineNotification) -> QString;

        /// Returns the notification message.
        fn message(self: &QWebEngineNotification) -> QString;

        /// Returns the origin of the notification.
        fn origin(self: &QWebEngineNotification) -> QUrl;
    }
}
