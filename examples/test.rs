use std::pin::Pin;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::QUrl;
use cxx_qt_lib_extras::QApplication;

use jawa::{PermissionType, QMainWindow, QWebEngineView, QWidget};

fn main() {
    let mut app = QApplication::new();
    let mut window = QMainWindow::new();

    let widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
    let mut view = QWebEngineView::new_with_parent(widget);

    window.pin_mut().set_central_widget(&mut view);
    let mut page = view.page();
    let connection = page.pin_mut().on_permission_requested(|page, permission| {
        if permission.permission_type() != PermissionType::Geolocation {
            println!("Unsupported permission type requested: {:?}", permission);
            return;
        }
        // TODO: message box
        println!(
            "Permission requested from origin: {}",
            permission.origin().to_string()
        );
        permission.grant();
        println!("Permission granted.");
    });
    view.pin_mut()
        // .load(&QUrl::from("https://html5test.teamdev.com"));
        .load(&QUrl::from("https://www.google.com/maps"));

    let mut widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
    widget.as_mut().resize(800, 600);
    widget.as_mut().show();

    app.pin_mut().exec();
}
