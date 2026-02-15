mod notification_popup;

use std::pin::Pin;

use crate::notification_popup::NotificationPopup;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use cxx_qt_widgets::{
    PermissionType, QApplication, QDesktopServices, QUrl, QWebEnginePage, QWebEngineProfile,
    QWebEngineView, QWidget, WidgetPtr, casting::Upcast,
};

#[cxx_qt::bridge]
pub mod qobject {
    // Define the API from QtQuick that we need
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-widgets/qwebenginepage.h");
        /// Base for Qt type
        type QWebEnginePage = cxx_qt_widgets::QWebEnginePage;
        type NavigationType = cxx_qt_widgets::NavigationType;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = QWebEnginePage]
        type WebEnginePage = super::WebEnginePageRust;

        #[cxx_override]
        #[cxx_name = "acceptNavigationRequest"]
        fn accept_navigation_request(
            self: Pin<&mut Self>,
            url: &QUrl,
            _type: NavigationType,
            is_main_frame: bool,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_web_engine_page() -> UniquePtr<WebEnginePage>;
    }
}

#[derive(Default)]
pub struct WebEnginePageRust;

impl qobject::WebEnginePage {
    /// Creates a new web engine page.
    pub fn new() -> WidgetPtr<Self> {
        qobject::new_web_engine_page().into()
    }

    pub fn accept_navigation_request(
        self: Pin<&mut Self>,
        url: &qobject::QUrl,
        _type: qobject::NavigationType,
        _is_main_frame: bool,
    ) -> bool {
        if url.scheme() != Some(QString::from("https")) {
            return true;
        }
        QDesktopServices::open_url(url);
        false
    }
}

fn main() {
    let mut app = QApplication::new();

    let mut view = QWebEngineView::new();
    let mut page = qobject::WebEnginePage::new();
    view.pin_mut().set_page(&mut page);
    let mut page: Pin<&mut QWebEnginePage> = page.pin_mut().upcast_pin();

    page.as_mut()
        .on_permission_requested(|_page, permission| {
            if permission.permission_type() != PermissionType::Notifications {
                println!("Unsupported permission type requested: {:?}", permission);
                return;
            }
            println!("Permission requested: {:?}", permission);
            permission.grant();
        })
        .release();

    let mut profile = page.profile();
    let mut profile: Pin<&mut QWebEngineProfile> = profile.pin_mut();
    view.pin_mut().load(&QUrl::from("qrc:/index.html"));
    let mut widget: Pin<&mut QWidget> = view.pin_mut().upcast_pin();
    let popup = NotificationPopup::new(widget.as_mut());
    let this = popup.qt_thread();

    profile
        .as_mut()
        .set_notification_presenter(move |notification| {
            let _ = this.queue(|p| p.present(notification));
        });

    widget.as_mut().resize(800, 600);
    widget.as_mut().show();

    app.pin_mut().exec();
    drop(view);
}
