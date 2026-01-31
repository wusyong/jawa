use cxx_qt_lib::QUrl;
use cxx_qt_lib_extras::QApplication;

use jawa::QWebEngineView;

fn main() {
    // Create the application and engine
    let mut app = QApplication::new();
    // let mut engine = QQmlApplicationEngine::new();

    let mut view = QWebEngineView::new();
    if let Some(mut view) = view.as_mut() {
        view.as_mut()
            .load(&QUrl::from("https://html5test.teamdev.com"));
        view.as_mut().show();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
