use std::pin::Pin;

use crate::{QWidget, WidgetPtr};
use cxx::memory::UniquePtrTarget;

use cxx_qt::{casting::Upcast, impl_transitive_cast};
pub use ffi::QVBoxLayout;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtWidgets/QVBoxLayout>);
        type QWidget = crate::QWidget;
        type QBoxLayout = crate::QBoxLayout;
        type QLayout = crate::QLayout;

        /// Vertical box layout for arranging child widgets in a column.
        #[qobject]
        #[base = QBoxLayout]
        type QVBoxLayout;

        /// Sets the spacing between items in the layout.
        #[cxx_name = "setSpacing"]
        fn set_spacing(self: Pin<&mut QVBoxLayout>, spacing: i32);

        /// Sets the contents margins of the layout.
        #[cxx_name = "setContentsMargins"]
        fn set_contents_margins(
            self: Pin<&mut QVBoxLayout>,
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_vbox_layout() -> UniquePtr<QVBoxLayout>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_vbox_layout_with_parent(parent: *mut QWidget) -> *mut QVBoxLayout;
    }
}

impl ffi::QVBoxLayout {
    /// Creates a new layout without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_vbox_layout().into()
    }

    /// Creates a new layout with a parent widget.
    pub fn new_with_parent<T: Upcast<QWidget> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe { ffi::new_vbox_layout_with_parent(parent.upcast_pin().get_unchecked_mut()).into() }
    }
}

impl_transitive_cast!(ffi::QVBoxLayout, ffi::QBoxLayout, ffi::QLayout);
