use std::pin::Pin;

use cxx_qt_widgets::{
    PermissionType, QApplication, QFlag, QMainWindow, QMessageBox, QUrl, QWebEngineView, QWidget, StandardButton, StandardButtons, casting::Upcast
};

fn main() {
    let mut app = QApplication::new();
    let mut window = QMainWindow::new();

    let widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
    let mut view = QWebEngineView::new_with_parent(widget);

    window.pin_mut().set_central_widget(&mut view);
    let mut page = view.page();
    let _connection = page.pin_mut().on_permission_requested(|_page, permission| {
        if permission.permission_type() != PermissionType::Geolocation {
            println!("Unsupported permission type requested: {:?}", permission);
            return;
        }

        let mut message_box = QMessageBox::new();
        message_box
            .pin_mut()
            .set_text(&format!("{} wants to know your location", permission.origin().host_or_default()).into());
        message_box.pin_mut().set_informative_text(
            &"Do you want to send your current location to this website?".into(),
        );

        message_box
            .pin_mut()
            .set_standard_buttons(StandardButtons::from(
                StandardButton::Yes | StandardButton::No,
            ));
        message_box
            .pin_mut()
            .set_default_button(StandardButton::Yes);

        if message_box.pin_mut().exec() == StandardButton::Yes.to_repr() as i32 {
            println!("User granted permission.");
            permission.grant();
        } else {
            println!("User denied permission.");
            permission.deny();
        }
    });
    view.pin_mut()
        // .load(&QUrl::from("https://html5test.teamdev.com"));
        .load(&QUrl::from("https://www.google.com/maps"));

    let mut widget: Pin<&mut QWidget> = window.pin_mut().upcast_pin();
    widget.as_mut().resize(800, 600);
    widget.as_mut().show();

    app.pin_mut().exec();
}
