use cxx::UniquePtr;

pub use ffi::{QSizePolicy, Policy};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qsizepolicy.h");

        /// Describes the size policy of a widget.
        type QSizePolicy;

        /// Sets the horizontal size policy.
        #[cxx_name = "setHorizontalPolicy"]
        fn set_horizontal_policy(self: Pin<&mut QSizePolicy>, policy: Policy);

        /// Sets the vertical size policy.
        #[cxx_name = "setVerticalPolicy"]
        fn set_vertical_policy(self: Pin<&mut QSizePolicy>, policy: Policy);

        /// Returns the horizontal size policy.
        #[cxx_name = "horizontalPolicy"]
        fn horizontal_policy(self: &QSizePolicy) -> Policy;

        /// Returns the vertical size policy.
        #[cxx_name = "verticalPolicy"]
        fn vertical_policy(self: &QSizePolicy) -> Policy;

        /// Sets the horizontal stretch factor.
        #[cxx_name = "setHorizontalStretch"]
        fn set_horizontal_stretch(self: Pin<&mut QSizePolicy>, stretch: i32);

        /// Sets the vertical stretch factor.
        #[cxx_name = "setVerticalStretch"]
        fn set_vertical_stretch(self: Pin<&mut QSizePolicy>, stretch: i32);

        /// Returns the horizontal stretch factor.
        #[cxx_name = "horizontalStretch"]
        fn horizontal_stretch(self: &QSizePolicy) -> i32;

        /// Returns the vertical stretch factor.
        #[cxx_name = "verticalStretch"]
        fn vertical_stretch(self: &QSizePolicy) -> i32;
    }

    /// Size policy for a widget in a layout.
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Policy {
        Fixed = 0,
        Minimum = 1,
        Maximum = 4,
        Preferred = 5,
        MinimumExpanding = 3,
        Expanding = 7,
        Ignored = 13,
    }

    extern "C++" {
        type Policy;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_size_policy() -> UniquePtr<QSizePolicy>;
    }
}

impl ffi::QSizePolicy {
    /// Creates a new size policy with default values.
    pub fn new() -> UniquePtr<Self> {
        ffi::new_size_policy()
    }
}
