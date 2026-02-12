use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use cxx::UniquePtr;
use cxx_qt::{Threading, impl_transitive_cast};
use cxx_qt_lib::{QPoint, QString};
use cxx_qt_widgets::{
    Policy, QBoxLayout, QHBoxLayout, QLabel, QMouseEvent, QPushButton, QSpacerItem, QVBoxLayout,
    QWebEngineNotification, QWidget, WidgetPtr, WindowType, casting::Upcast,
};

#[cxx_qt::bridge]
pub mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qwidget.h");
        type WindowFlags = cxx_qt_widgets::WindowFlags;

    }

    unsafe extern "C++" {
        type RustQWidget = cxx_qt_widgets::RustQWidget;
        type QMouseEvent = cxx_qt_widgets::QMouseEvent;
        type QWidget = cxx_qt_widgets::QWidget;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = RustQWidget]
        type NotificationPopup = super::NotificationPopupRust;

        #[cxx_override]
        #[cxx_name = "mouseReleaseEvent"]
        fn mouse_release_event(self: Pin<&mut Self>, event: *mut QMouseEvent);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_popup() -> UniquePtr<NotificationPopup>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_popup_with_parent(parent: *mut QWidget) -> *mut NotificationPopup;
    }

    impl cxx_qt::Threading for NotificationPopup {}
}

impl_transitive_cast!(ffi::NotificationPopup, ffi::RustQWidget, ffi::QWidget);

#[derive(Default)]
pub struct NotificationPopupRust;

impl ffi::NotificationPopup {
    fn new() -> WidgetPtr<Self> {
        ffi::new_popup().into()
    }

    pub fn new_with_parent(parent: Pin<&mut QWidget>) -> WidgetPtr<Self> {
        unsafe { ffi::new_popup_with_parent(parent.get_unchecked_mut()).into() }
    }

    pub fn mouse_release_event(self: Pin<&mut Self>, event: *mut QMouseEvent) {
        // TODO:
        println!("Mouse released on notification popup!");
    }
}

pub struct NotificationPopup {
    this: WidgetPtr<ffi::NotificationPopup>,
    icon: WidgetPtr<QLabel>,
    title: WidgetPtr<QLabel>,
    message: WidgetPtr<QLabel>,
    notification: Arc<Mutex<UniquePtr<QWebEngineNotification>>>,
}

unsafe impl Send for NotificationPopup {}
unsafe impl Sync for NotificationPopup {}

impl NotificationPopup {
    pub fn new(parent: Pin<&mut QWidget>) -> Self {
        // TODO: new_shared
        let mut this = ffi::NotificationPopup::new_with_parent(parent);
        let this_clone = this.qt_thread();
        let mut widget: Pin<&mut QWidget> = this.pin_mut().upcast_pin();
        widget.as_mut().set_window_flags(WindowType::ToolTip.into());

        let mut root_layout = QHBoxLayout::new_with_parent(widget.as_mut());
        let mut root_layout: Pin<&mut QBoxLayout> = root_layout.pin_mut().upcast_pin();

        let mut icon = QLabel::new();
        root_layout.as_mut().add_widget(&mut icon);

        let mut body_layout = QVBoxLayout::new();
        root_layout.as_mut().add_layout(&mut body_layout);
        let mut body_layout: Pin<&mut QBoxLayout> = body_layout.pin_mut().upcast_pin();

        let mut title_layout = QHBoxLayout::new();
        body_layout.as_mut().add_layout(&mut title_layout);
        let mut title_layout: Pin<&mut QBoxLayout> = title_layout.pin_mut().upcast_pin();

        let mut title = QLabel::new();
        title_layout.as_mut().add_widget(&mut title);

        let mut spacer_item = QSpacerItem::new(0, 0, Policy::Expanding, Policy::Minimum);
        title_layout.as_mut().add_item(&mut spacer_item);

        let mut close = QPushButton::new();
        close.pin_mut().set_text(&QString::from("Close"));
        title_layout.as_mut().add_widget(&mut close);

        let notification = Arc::new(Mutex::new(UniquePtr::null()));
        let notification_clone = notification.clone();
        close
            .pin_mut()
            .on_clicked(move |_, _| {
                let notification = notification_clone.clone();
                let _ = this_clone.queue(move |this| {
                    on_close(this, &notification.lock().unwrap());
                });
            })
            .release();

        let mut message = QLabel::new();
        body_layout.as_mut().add_widget(&mut message);

        widget.as_mut().adjust_size();

        Self {
            this,
            icon,
            title,
            message,
            notification,
        }
    }

    pub fn present(&mut self, new_notification: UniquePtr<QWebEngineNotification>) {
        let popup = self.this.qt_thread();
        let mut notification = self.notification.lock().unwrap();
        if !notification.is_null() {
            on_close(self.this.pin_mut(), &notification);
        }
        *notification = new_notification;

        self.title.pin_mut().set_text(&notification.title());
        self.message.pin_mut().set_text(&notification.message());
        //TODO: icon

        let mut widget: Pin<&mut QWidget> = self.this.pin_mut().upcast_pin();
        widget.as_mut().show();
        notification.show();

        notification
            .pin_mut()
            .on_closed(move |notification| {
                notification.close();
                let _ = popup.queue(move |this| {
                    let mut widget: Pin<&mut QWidget> = this.upcast_pin();
                    widget.as_mut().hide();
                });
            })
            .release();
        // TODO: QTimer::singleShot(10000, notification.get(), [&] () { onClosed(); });

        // position our popup in the right corner of its parent widget
        if let Some(parent) = widget.parent() {
            let width = widget.width();
            let height = widget.height();
            widget.as_mut()._move(&parent.map_to_global(
                &(parent.rect().bottom_right() - QPoint::new(width + 10, height + 10)),
            ));
        }
    }
}

fn on_close(
    this: Pin<&mut ffi::NotificationPopup>,
    notification: &UniquePtr<QWebEngineNotification>,
) {
    let mut widget: Pin<&mut QWidget> = this.upcast_pin();
    widget.as_mut().hide();
    if !notification.is_null() {
        notification.close();
    }
}
