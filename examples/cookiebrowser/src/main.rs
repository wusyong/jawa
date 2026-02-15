use cxx_qt_widgets::{
    QApplication, QPushButton, QWebEngineView, QWidget, WidgetPtr,
};

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++Qt" {
        include!("cookiebrowser/mainwindow.h");
        type QUrl = cxx_qt_lib::QUrl;
        type QWidget = cxx_qt_widgets::QWidget;
        type QPushButton = cxx_qt_widgets::QPushButton;
        type QWebEngineView = cxx_qt_widgets::QWebEngineView;
        #[qobject]
        type MainWindow;

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut MainWindow>);

        /// Resizes the widget to the given size in pixels.
        #[cxx_name = "resize"]
        fn resize(self: Pin<&mut MainWindow>, width: i32, height: i32);

        #[cxx_name = "urlLineEditWidgetRaw"]
        fn url_line_edit_widget_raw(self: &MainWindow) -> *mut QWidget;

        #[cxx_name = "scrollAreaWidgetRaw"]
        fn scroll_area_widget_raw(self: &MainWindow) -> *mut QWidget;

        #[cxx_name = "urlButton"]
        fn url_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "deleteAllButton"]
        fn delete_all_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "newButton"]
        fn new_button_raw(self: &MainWindow) -> *mut QPushButton;

        #[cxx_name = "webview"]
        fn webview_raw(self: &MainWindow) -> *mut QWebEngineView;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_main_window() -> UniquePtr<MainWindow>;
    }
}

impl ffi::MainWindow {
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_main_window().into()
    }

    pub fn url_line_edit_widget(&self) -> WidgetPtr<QWidget> {
        self.url_line_edit_widget_raw().into()
    }

    pub fn scroll_area_widget(&self) -> WidgetPtr<QWidget> {
        self.scroll_area_widget_raw().into()
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
}

fn main() {
    let mut app = QApplication::new();

    let mut window = ffi::MainWindow::new();
    window.pin_mut().resize(1024, 768);
    window.pin_mut().show();

    app.pin_mut().exec();
}
