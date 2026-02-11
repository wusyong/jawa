use std::{pin::Pin, ptr::null_mut};

use cxx_qt::impl_transitive_cast;
use cxx_qt_widgets::{
    QBoxLayout, QHBoxLayout, QLabel, QLayout, QMouseEvent, QVBoxLayout, QWidget, RustQWidget, WidgetPtr, WindowFlags, WindowType, casting::Upcast
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
        widget.as_mut().show();

        // let mut root_layout: Pin<&mut QBoxLayout> = this.pin_mut().upcast_pin();

        let mut icon = QLabel::new();
        // root_layout.as_mut().add_widget(&mut icon);

        // let mut body_layout = QVBoxLayout::new();
        // root_layout.as_mut().add_layout(&mut body_layout);

        let mut title = QLabel::new();
        let mut message = QLabel::new();

        Self {
            this,
            icon,
            title,
            message,
        }
    }
}
