use std::{
    pin::Pin,
    sync::{Mutex, OnceLock},
};

use crate::{QWebEngineNotification, WidgetPtr};
use cxx::UniquePtr;

pub use ffi::{PersistentCookiesPolicy, QWebEngineProfile};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qwebengineprofile.h");
        type QWebEngineNotification = crate::QWebEngineNotification;
        /// Configuration and persistent storage for web engine components.
        #[qobject]
        type QWebEngineProfile;

        /// Returns the storage name for this profile.
        #[cxx_name = "storageName"]
        fn storage_name(self: &QWebEngineProfile) -> QString;

        /// Returns the HTTP user agent used by this profile.
        #[cxx_name = "httpUserAgent"]
        fn http_user_agent(self: &QWebEngineProfile) -> QString;

        /// Sets the HTTP user agent for this profile.
        #[cxx_name = "setHttpUserAgent"]
        fn set_http_user_agent(self: Pin<&mut QWebEngineProfile>, user_agent: &QString);

        /// Returns the HTTP Accept-Language header for this profile.
        #[cxx_name = "httpAcceptLanguage"]
        fn http_accept_language(self: &QWebEngineProfile) -> QString;

        /// Sets the HTTP Accept-Language header for this profile.
        #[cxx_name = "setHttpAcceptLanguage"]
        fn set_http_accept_language(self: Pin<&mut QWebEngineProfile>, language: &QString);

        /// Returns the persistent storage path.
        #[cxx_name = "persistentStoragePath"]
        fn persistent_storage_path(self: &QWebEngineProfile) -> QString;

        /// Sets the persistent storage path.
        #[cxx_name = "setPersistentStoragePath"]
        fn set_persistent_storage_path(self: Pin<&mut QWebEngineProfile>, path: &QString);

        /// Returns the cache path.
        #[cxx_name = "cachePath"]
        fn cache_path(self: &QWebEngineProfile) -> QString;

        /// Sets the cache path.
        #[cxx_name = "setCachePath"]
        fn set_cache_path(self: Pin<&mut QWebEngineProfile>, path: &QString);

        /// Returns the persistent cookies policy.
        #[cxx_name = "persistentCookiesPolicy"]
        fn persistent_cookies_policy(self: &QWebEngineProfile) -> PersistentCookiesPolicy;

        /// Sets the persistent cookies policy.
        #[cxx_name = "setPersistentCookiesPolicy"]
        fn set_persistent_cookies_policy(
            self: Pin<&mut QWebEngineProfile>,
            policy: PersistentCookiesPolicy,
        );

        /// Returns true if the profile is off-the-record.
        #[cxx_name = "isOffTheRecord"]
        fn is_off_the_record(self: &QWebEngineProfile) -> bool;

        /// Sets the notification presenter callback.
        #[cxx_name = "setNotificationPresenter"]
        fn set_notification_presenter_raw(
            profile: Pin<&mut QWebEngineProfile>,
            presenter: fn(UniquePtr<QWebEngineNotification>),
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_web_engine_profile() -> UniquePtr<QWebEngineProfile>;
    }

    /// Policy for persistent cookies.
    #[repr(u32)]
    #[derive(Debug)]
    enum PersistentCookiesPolicy {
        NoPersistentCookies,
        AllowPersistentCookies,
        ForcePersistentCookies,
    }

    unsafe extern "C++" {
        type PersistentCookiesPolicy;
    }
}

impl QWebEngineProfile {
    /// Creates a new web engine profile.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_web_engine_profile().into()
    }

    pub fn set_notification_presenter<F>(self: Pin<&mut Self>, presenter: F)
    where
        F: FnMut(UniquePtr<QWebEngineNotification>) + Send + 'static,
    {
        let _ = PRESENTER.set(Mutex::new(Box::new(presenter)));
        ffi::set_notification_presenter_raw(self, presenter_trampoline);
    }
}

static PRESENTER: OnceLock<Mutex<Box<dyn FnMut(UniquePtr<QWebEngineNotification>) + Send>>> =
    OnceLock::new();

fn presenter_trampoline(notification: UniquePtr<QWebEngineNotification>) {
    if let Some(lock) = PRESENTER.get() {
        let mut cb = lock.lock().unwrap();
        (cb)(notification);
    }
}
