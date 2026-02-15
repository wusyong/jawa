use std::pin::Pin;

use crate::{QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

pub use ffi::QLayout;

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
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
}
