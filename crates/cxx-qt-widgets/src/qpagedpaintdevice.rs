pub use ffi::QPagedPaintDevice;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qpagedpaintdevice.h");
        type QPagedPaintDevice;
    }
}
