pub use ffi::{qpainter_begin, qpainter_draw_text, qpainter_end};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qpainter.h");

        include!("cxx-qt-lib/qpainter.h");
        type QPainter = cxx_qt_lib::QPainter;

        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qprinter.h");
        type QPrinter = crate::QPrinter;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[cxx_name = "qpainter_begin"]
        pub fn qpainter_begin(painter: Pin<&mut QPainter>, device: Pin<&mut QPrinter>) -> bool;

        #[cxx_name = "qpainter_end"]
        pub fn qpainter_end(painter: Pin<&mut QPainter>) -> bool;

        #[cxx_name = "qpainter_draw_text"]
        pub fn qpainter_draw_text(painter: Pin<&mut QPainter>, position: &QPointF, text: &QString);
    }
}
