use cxx_qt_widgets::{
    QApplication, QUrl, WidgetPtr,
};

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++Qt" {
        include!("cookiebrowser/mainwindow.h");
        type QUrl = cxx_qt_lib::QUrl;
        #[qobject]
        type MainWindow;

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut MainWindow>);

        /// Resizes the widget to the given size in pixels.
        #[cxx_name = "resize"]
        fn resize(self: Pin<&mut MainWindow>, width: i32, height: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_main_window(url: &QUrl) -> UniquePtr<MainWindow>;
    }
}

impl ffi::MainWindow {
    pub fn new(url: &QUrl) -> WidgetPtr<Self> {
        ffi::new_main_window(url).into()
    }
}

fn main() {
    let mut app = QApplication::new();

    let mut window = ffi::MainWindow::new(&QUrl::from("https://qt.io"));
    window.pin_mut().resize(1024, 768);
    window.pin_mut().show();

    app.pin_mut().exec();
}
