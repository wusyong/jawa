use std::pin::Pin;

use crate::{Alignment, QLayout, QLayoutItem, QWidget, WidgetPtr};
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
        type QLayout = crate::QLayout;
        type QLayoutItem = crate::QLayoutItem;

        /// Box layout for arranging child widgets in a single row or column.
        #[qobject]
        #[base = QLayout]
        type QBoxLayout;

        /// Adds a widget to the layout.
        #[cxx_name = "addWidget"]
        unsafe fn add_widget_raw(
            self: Pin<&mut QBoxLayout>,
            widget: *mut QWidget,
            strech: i32,
            alignment: Alignment,
        );

        /// Adds a layout to the layout.
        #[cxx_name = "addLayout"]
        unsafe fn add_layout_raw(self: Pin<&mut QBoxLayout>, layout: *mut QLayout, strech: i32);

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

        /// Adds a layout item to the layout.
        #[cxx_name = "addItem"]
        unsafe fn add_item_raw(self: Pin<&mut QBoxLayout>, item: *mut QLayoutItem);
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
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        direction: Direction,
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe {
            ffi::new_box_layout_with_parent(direction, parent.upcast_pin().get_unchecked_mut())
                .into()
        }
    }

    /// Adds a widget to the layout, transferring ownership to the layout.
    pub fn add_widget<T: Upcast<QWidget> + UniquePtrTarget>(
        self: Pin<&mut QBoxLayout>,
        widget: &mut WidgetPtr<T>,
    ) {
        widget.release();
        unsafe {
            self.add_widget_raw(
                widget.pin_mut().upcast_pin().get_unchecked_mut(),
                0,
                Alignment::new(),
            );
        }
    }

    /// Adds a layout to this layout, transferring ownership to the parent layout.
    pub fn add_layout<T: Upcast<QLayout> + UniquePtrTarget>(
        self: Pin<&mut QBoxLayout>,
        layout: &mut WidgetPtr<T>,
    ) {
        self.add_layout_with_stretch(layout, 0);
    }

    /// Adds a layout with the given stretch factor.
    pub fn add_layout_with_stretch<T: Upcast<QLayout> + UniquePtrTarget>(
        self: Pin<&mut QBoxLayout>,
        layout: &mut WidgetPtr<T>,
        strech: i32,
    ) {
        layout.release();
        unsafe {
            self.add_layout_raw(layout.pin_mut().upcast_pin().get_unchecked_mut(), strech);
        }
    }

    /// Adds a layout item to this layout, transferring ownership to the parent layout.
    pub fn add_item<T: Upcast<QLayoutItem> + UniquePtrTarget>(
        self: Pin<&mut QBoxLayout>,
        item: &mut WidgetPtr<T>,
    ) {
        item.release();
        unsafe {
            self.add_item_raw(item.pin_mut().upcast_pin().get_unchecked_mut());
        }
    }
}
