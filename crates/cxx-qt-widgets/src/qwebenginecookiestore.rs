pub use ffi::QWebEngineCookieStore;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qnetworkcookie.h");
        type QNetworkCookie = crate::QNetworkCookie;
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-widgets/qwebenginecookiestore.h");

        /// Access to the Chromium cookie store used by a web engine profile.
        #[qobject]
        type QWebEngineCookieStore;

        /// Loads all cookies from persistent storage.
        #[cxx_name = "loadAllCookies"]
        fn load_all_cookies(self: Pin<&mut QWebEngineCookieStore>);

        /// Deletes all cookies.
        #[cxx_name = "deleteAllCookies"]
        fn delete_all_cookies(self: Pin<&mut QWebEngineCookieStore>);

        /// Sets (adds or updates) a cookie.
        #[cxx_name = "setCookie"]
        fn set_cookie(self: Pin<&mut QWebEngineCookieStore>, cookie: &QNetworkCookie, url: &QUrl);

        /// Deletes a cookie.
        #[cxx_name = "deleteCookie"]
        fn delete_cookie(self: Pin<&mut QWebEngineCookieStore>, cookie: &QNetworkCookie, url: &QUrl);

        /// Emitted when a cookie is added.
        #[qsignal]
        #[cxx_name = "cookieAdded"]
        fn cookie_added(self: Pin<&mut QWebEngineCookieStore>, cookie: &QNetworkCookie);

        /// Emitted when a cookie is removed.
        #[qsignal]
        #[cxx_name = "cookieRemoved"]
        fn cookie_removed(self: Pin<&mut QWebEngineCookieStore>, cookie: &QNetworkCookie);
    }

    impl UniquePtr<QWebEngineCookieStore> {}
}
