pub use ffi::QMouseEvent;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtGui/QMouseEvent>);
        type QSinglePointEvent = crate::QSinglePointEvent;

        /// Mouse event with a single pointer.
        #[qobject]
        #[base = QSinglePointEvent]
        type QMouseEvent;
    }
}
