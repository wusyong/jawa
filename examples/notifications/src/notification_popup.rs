use std::{pin::Pin, ptr::null_mut};

use cxx_qt_widgets::{
    QBoxLayout, QHBoxLayout, QLabel, QLayout, QVBoxLayout, QWidget, WidgetPtr, WindowFlags,
    WindowType, casting::Upcast,
};

#[cxx_qt::bridge]
pub mod qobject {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<QWidget>);
        type WindowFlags = cxx_qt_widgets::WindowFlags;

    }
    // Define the API from QtQuick that we need
    unsafe extern "C++" {
        include!(<memory>);
        /// Base for Qt type
        type QWidget = cxx_qt_widgets::QWidget;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = QWidget]
        type NotificationPopup = super::NotificationPopupRust;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        // #[doc(hidden)]
        // #[cxx_name = "make_unique"]
        // unsafe fn new_popup(
        //     // parent: *mut QWidget,
        //     // window_flags: WindowFlags,
        // ) -> UniquePtr<NotificationPopup>;
    }
}


#[derive(Default)]
pub struct NotificationPopupRust;

// impl qobject::NotificationPopup {
//     /// Creates a new notification popup.
//     // pub fn new() -> WidgetPtr<Self> {
//     //     unsafe { qobject::new_popup().into() }
//     // }
// }

pub struct NotificationPopup {
    this: WidgetPtr<QHBoxLayout>,
    icon: WidgetPtr<QLabel>,
    title: WidgetPtr<QLabel>,
    message: WidgetPtr<QLabel>,
}

impl NotificationPopup {
    pub fn new() -> Self {
        // let mut this = QWidget::new();// TODO: Fix Qwidget constructor to accept parent and window flags
        // let mut widget: Pin<&mut QWidget> = this.pin_mut().upcast_pin();
        // widget.as_mut().set_window_flags(WindowType::ToolTip.into());

        let mut this = QHBoxLayout::new();
        let mut root_layout: Pin<&mut QBoxLayout> = this.pin_mut().upcast_pin();

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
