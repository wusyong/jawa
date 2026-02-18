mod printer_handler;

use std::pin::Pin;

use cxx_qt_widgets::{QApplication, QUrl, QWebEngineView, QWidget, casting::Upcast};
use printer_handler::PrinterHandler;

fn main() {
    let mut app = QApplication::new();

    let mut view = QWebEngineView::new();
    view.pin_mut().load(&QUrl::from("qrc:/index.html"));

    let mut widget: Pin<&mut QWidget> = view.pin_mut().upcast_pin();
    widget.as_mut().resize(1024, 750);
    widget.as_mut().show();

    let mut handler = PrinterHandler::new(None);
    handler.pin_mut().set_view(view.pin_mut());

    // TODO: Shortcuts

    app.pin_mut().exec();
}
