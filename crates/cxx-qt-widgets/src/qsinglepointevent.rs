pub use ffi::QSinglePointEvent;

#[cxx_qt::bridge]
mod ffi {
    #[namespace = "Qt"]
    extern "C++" {
        include!(<QtGui/QSinglePointEvent>);
        type MouseButton = crate::MouseButton;
        type MouseButtons = crate::MouseButtons;
    }

    unsafe extern "C++" {
        include!(<QtGui/QSinglePointEvent>);
        /// Base class for pointer events with a single point (mouse, tablet).
        type QSinglePointEvent;

        /// Returns the button that caused the event.
        fn button(self: &QSinglePointEvent) -> MouseButton;

        /// Returns the state of all mouse buttons during the event.
        fn buttons(self: &QSinglePointEvent) -> MouseButtons;

        /// Returns the timestamp of the event in milliseconds.
        fn timestamp(self: &QSinglePointEvent) -> u64;
    }
}
