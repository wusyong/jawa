use std::pin::Pin;

use cxx_qt_lib::{QColor, QString, QUrl};
use cxx_qt_widgets::{
    DialogCode, Policy, QApplication, QBoxLayout, QLineEdit, QList_QNetworkCookie, QPalette, QPushButton, QScrollArea, QSpacerItem, QWebEngineView, QWidget, ScrollBarPolicy, WidgetPtr
};

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++Qt" {
        include!("cookiebrowser/mainwindow.h");
        type QUrl = cxx_qt_lib::QUrl;
        type QWidget = cxx_qt_widgets::QWidget;
        type QPushButton = cxx_qt_widgets::QPushButton;
        type QWebEngineView = cxx_qt_widgets::QWebEngineView;
        type QLineEdit = cxx_qt_widgets::QLineEdit;
        type QScrollArea = cxx_qt_widgets::QScrollArea;
        type QList_QNetworkCookie = cxx_qt_widgets::QList_QNetworkCookie;
        type QBoxLayout = cxx_qt_widgets::QBoxLayout;
        type QNetworkCookie = cxx_qt_widgets::QNetworkCookie;
        #[qobject]
        type MainWindow;

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut MainWindow>);

        /// Resizes the widget to the given size in pixels.
        #[cxx_name = "resize"]
        fn resize(self: Pin<&mut MainWindow>, width: i32, height: i32);

        #[cxx_name = "urlLineEdit"]
        fn url_line_edit_raw(self: &MainWindow) -> *mut QLineEdit;

        #[cxx_name = "scrollArea"]
        fn scroll_area_raw(self: &MainWindow) -> *mut QScrollArea;

        #[cxx_name = "urlButton"]
        fn url_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "deleteAllButton"]
        fn delete_all_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "newButton"]
        fn new_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "webview"]
        fn webview_raw(self: &MainWindow) -> *mut QWebEngineView;

        #[cxx_name = "cookies"]
        fn cookies_raw(self: &MainWindow) -> *const QList_QNetworkCookie;

        #[cxx_name = "layout"]
        fn layout_raw(self: &MainWindow) -> *mut QBoxLayout;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_main_window() -> UniquePtr<MainWindow>;
    }

    unsafe extern "C++Qt" {
        #[qobject]
        type CookieDialog;

        fn cookie(self: Pin<&mut CookieDialog>) -> QNetworkCookie;

        fn exec(self: Pin<&mut CookieDialog>) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_cookie_dialog() -> UniquePtr<CookieDialog>;
    }
}

unsafe impl Send for ffi::MainWindow {}
unsafe impl Sync for ffi::MainWindow {}

impl ffi::MainWindow {
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_main_window().into()
    }

    pub fn url_line_edit(&self) -> WidgetPtr<QLineEdit> {
        self.url_line_edit_raw().into()
    }

    pub fn scroll_area(&self) -> WidgetPtr<QScrollArea> {
        self.scroll_area_raw().into()
    }

    pub fn url_button(&self) -> WidgetPtr<QPushButton> {
        self.url_button_raw().into()
    }

    pub fn delete_all_button(&self) -> WidgetPtr<QPushButton> {
        self.delete_all_button_raw().into()
    }

    pub fn new_button(&self) -> WidgetPtr<QPushButton> {
        self.new_button_raw().into()
    }

    pub fn webview(&self) -> WidgetPtr<QWebEngineView> {
        self.webview_raw().into()
    }

    pub fn cookies(&self) -> WidgetPtr<QList_QNetworkCookie> {
        (self.cookies_raw() as *mut QList_QNetworkCookie).into()
    }

    pub fn layout(&self) -> WidgetPtr<QBoxLayout> {
        self.layout_raw().into()
    }
}

impl ffi::CookieDialog {
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_cookie_dialog().into()
    }
}

fn main() {
    let mut app = QApplication::new();

    let mut window = ffi::MainWindow::new();
    let url = QUrl::from("https://www.qt.io");
    window.url_line_edit().pin_mut().set_text(&url.to_qstring());

    let mut vbox_layout = window.layout();
    let mut layout: Pin<&mut QBoxLayout> = vbox_layout.pin_mut();
    layout.as_mut().add_item(&mut QSpacerItem::new(
        0,
        0,
        Policy::Minimum,
        Policy::Expanding,
    ));
    layout.as_mut().set_contents_margins(0, 0, 0, 0);
    layout.as_mut().set_spacing(0);

    let mut w = QWidget::new();
    let mut p = QPalette::new();
    p.pin_mut()
        .set_color(w.background_role(), &QColor::from_rgb(0, 0, 0));
    w.pin_mut().set_palette(&p);
    w.pin_mut().set_layout(&mut vbox_layout);

    let mut scroll_area = window.scroll_area();
    scroll_area.pin_mut().set_widget(&mut w);
    scroll_area
        .pin_mut()
        .set_horizontal_scroll_bar_policy(ScrollBarPolicy::ScrollBarAlwaysOff);
    scroll_area
        .pin_mut()
        .set_vertical_scroll_bar_policy(ScrollBarPolicy::ScrollBarAlwaysOn);

    let win: WidgetPtr<ffi::MainWindow> = window.as_mut_ptr().into();
    window
        .url_button()
        .pin_mut()
        .on_clicked(move |_, _| {
            let url = QUrl::from_user_input(&win.url_line_edit().text(), &QString::from(""));
            win.webview().pin_mut().load(&url);
        })
        .release();

    let win: WidgetPtr<ffi::MainWindow> = window.as_mut_ptr().into();
    window
        .delete_all_button()
        .pin_mut()
        .on_clicked(move |_, _| {
            let mut store = win.webview().page().profile().pin_mut().cookie_store();
            store.pin_mut().delete_all_cookies();
            let index = win.layout().count() - 1;
            for i in index..=0 {
                win.layout().item_at(i).widget().as_mut().map(|w|w.delete());
            }
            win.cookies().pin_mut().clear();
        })
        .release();

    let win: WidgetPtr<ffi::MainWindow> = window.as_mut_ptr().into();
    window.new_button().pin_mut().on_clicked(move |_, _| {
        let mut dialog = ffi::CookieDialog::new();
        if dialog.pin_mut().exec() == DialogCode::Accepted.repr as i32 {
            let cookie = dialog.pin_mut().cookie();
            win.cookies().pin_mut().append(&cookie);
        }
    }).release();

    // connect(m_store, &QWebEngineCookieStore::cookieAdded, this, &MainWindow::handleCookieAdded);
    let mut store = window.webview().page().profile().pin_mut().cookie_store();
    store.pin_mut().load_all_cookies();
    window.webview().pin_mut().load(&url);

    window.pin_mut().resize(1024, 768);
    window.pin_mut().show();

    app.pin_mut().exec();
}
