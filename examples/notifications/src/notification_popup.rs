use std::{pin::Pin, ptr::null_mut};

use cxx_qt::impl_transitive_cast;
use cxx_qt_lib::QString;
use cxx_qt_widgets::{
    QBoxLayout, QHBoxLayout, QLabel, QLayout, QMouseEvent, QPushButton, QVBoxLayout, QWidget, RustQWidget, WidgetPtr, WindowFlags, WindowType, casting::Upcast
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
    }
}

impl_transitive_cast!(ffi::NotificationPopup, ffi::RustQWidget, ffi::QWidget);

#[derive(Default)]
pub struct NotificationPopupRust;

impl ffi::NotificationPopup {
    fn new() -> WidgetPtr<Self> {
        ffi::new_popup().into()
    }

    pub fn mouse_release_event(self: Pin<&mut Self>, event: *mut QMouseEvent) {
        println!("Mouse released on notification popup!");
    }
}

pub struct NotificationPopup {
    this: WidgetPtr<ffi::NotificationPopup>,
    icon: WidgetPtr<QLabel>,
    title: WidgetPtr<QLabel>,
    message: WidgetPtr<QLabel>,
}

impl NotificationPopup {
    pub fn new() -> Self {
        let mut this = ffi::NotificationPopup::new();
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

        // titleLayout->addItem(new QSpacerItem(0, 0, QSizePolicy::Expanding));

        let mut close = QPushButton::new();
        close.pin_mut().set_text(&QString::from("Close"));
        title_layout.as_mut().add_widget(&mut close);
        // connect(close, &QPushButton::clicked, this, &NotificationPopup::onClosed);

        let mut message = QLabel::new();
        body_layout.as_mut().add_widget(&mut message);
        widget.as_mut().adjust_size();
        // widget.as_mut().show();

        Self {
            this,
            icon,
            title,
            message,
        }
    }
}
