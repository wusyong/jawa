use std::pin::Pin;

use crate::{Alignment, QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

pub use ffi::{Direction, QBoxLayout};

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<QtWidgets/QWidget>);
        type Alignment = crate::Alignment;
    }
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qboxlayout.h");
        type QWidget = crate::QWidget;

        /// Box layout for arranging child widgets in a single row or column.
        #[qobject]
        type QBoxLayout;

        /// Adds a widget to the layout.
        #[cxx_name = "addWidget"]
        unsafe fn add_widget_raw(
            self: Pin<&mut QBoxLayout>,
            widget: *mut QWidget,
            strech: i32,
            alignment: Alignment,
        );

        /// Sets the spacing between items in the layout.
        #[cxx_name = "setSpacing"]
        fn set_spacing(self: Pin<&mut QBoxLayout>, spacing: i32);

        /// Sets the contents margins of the layout.
        #[cxx_name = "setContentsMargins"]
        fn set_contents_margins(
            self: Pin<&mut QBoxLayout>,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );

        /// Sets the layout direction.
        #[cxx_name = "setDirection"]
        fn set_direction(self: Pin<&mut QBoxLayout>, direction: Direction);
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Direction {
        LeftToRight = 0,
        RightToLeft = 1,
        TopToBottom = 2,
        BottomToTop = 3,
    }

    extern "C++" {
        type Direction;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_box_layout(direction: Direction) -> UniquePtr<QBoxLayout>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_box_layout_with_parent(
            direction: Direction,
            parent: *mut QWidget,
        ) -> *mut QBoxLayout;
    }
}

impl ffi::QBoxLayout {
    /// Creates a new layout without a parent.
    pub fn new(direction: Direction) -> WidgetPtr<Self> {
        ffi::new_box_layout(direction).into()
    }

    /// Creates a new layout with a parent widget.
    pub fn new_with_parent(direction: Direction, parent: Pin<&mut QWidget>) -> WidgetPtr<Self> {
        unsafe { ffi::new_box_layout_with_parent(direction, parent.get_unchecked_mut()).into() }
    }

    /// Adds a widget to the layout, transferring ownership to the layout.
    pub fn add_widget<T: Upcast<QWidget> + UniquePtrTarget>(
        self: Pin<&mut QBoxLayout>,
        widget: &mut WidgetPtr<T>,
    ) {
        widget.release();
        unsafe {
            self.add_widget_raw(
                (&mut *widget.as_mut_ptr()).upcast_mut(),
                0,
                Alignment::new(),
            );
        }
    }
}
