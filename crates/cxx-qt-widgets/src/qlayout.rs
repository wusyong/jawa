use std::pin::Pin;

use crate::{Alignment, QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

pub use ffi::QLayout;

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!(<QtWidgets/QWidget>);
        type Alignment = crate::Alignment;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qlayout.h");
        type QWidget = crate::QWidget;

        /// Base class of geometry managers.
        #[qobject]
        type QLayout;

        /// Sets the spacing between items in the layout.
        #[cxx_name = "setSpacing"]
        fn set_spacing(self: Pin<&mut QLayout>, spacing: i32);

        /// Sets the contents margins of the layout.
        #[cxx_name = "setContentsMargins"]
        fn set_contents_margins(
            self: Pin<&mut QLayout>,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );

        /// Sets the alignment for a widget in this layout.
        #[cxx_name = "setAlignment"]
        unsafe fn set_alignment_widget(
            self: Pin<&mut QLayout>,
            widget: *mut QWidget,
            alignment: Alignment,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        #[doc(hidden)]
        #[cxx_name = "new_layout"]
        fn new_layout() -> UniquePtr<QLayout>;

        #[doc(hidden)]
        #[cxx_name = "new_layout_with_parent"]
        unsafe fn new_layout_with_parent(parent: *mut QWidget) -> *mut QLayout;
    }
}

impl ffi::QLayout {
    /// Creates a new layout without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_layout().into()
    }

    /// Creates a new layout with a parent widget.
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe { ffi::new_layout_with_parent(parent.upcast_pin().get_unchecked_mut()).into() }
    }

    /// Sets the alignment for a widget in this layout.
    pub fn set_alignment_for_widget<T: Upcast<QWidget> + UniquePtrTarget>(
        self: Pin<&mut QLayout>,
        widget: &mut WidgetPtr<T>,
        alignment: Alignment,
    ) -> bool {
        unsafe { self.set_alignment_widget(widget.pin_mut().upcast_pin().get_unchecked_mut(), alignment) }
    }
}
