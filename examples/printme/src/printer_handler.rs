use cxx::UniquePtr;
use cxx_qt::{QObject, Threading};
use cxx_qt_widgets::{
    QEventLoop, QEventLoopProcessEventsFlag, QEventLoopProcessEventsFlags, QPainter,
    QPrintPreviewDialog, QPrinter, QString, QWebEngineView, qpainter_begin, qpainter_draw_text,
    qpainter_end,
};
use std::cell::{Cell, RefCell};
use std::pin::Pin;

pub use ffi::PrinterHandler;

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "RustQt" {
        #[qobject]
        type PrinterHandler = super::PrinterHandlerRust;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[cxx_name = "make_unique"]
        #[doc(hidden)]
        fn new_printer_handler() -> UniquePtr<PrinterHandler>;

        #[cxx_name = "make_unique"]
        #[doc(hidden)]
        unsafe fn new_printer_handler_with_parent(
            parent: *mut QObject,
        ) -> UniquePtr<PrinterHandler>;
    }

    impl cxx_qt::Threading for PrinterHandler {}
}

unsafe impl Send for ffi::PrinterHandler {}
unsafe impl Sync for ffi::PrinterHandler {}

pub struct PrinterHandlerRust {
    printer: RefCell<UniquePtr<QPrinter>>,
    webengine_view: Cell<*mut QWebEngineView>,
    in_print_preview: Cell<bool>,
    wait_for_result: RefCell<UniquePtr<QEventLoop>>,
}

impl Default for PrinterHandlerRust {
    fn default() -> Self {
        let mut printer = QPrinter::new();
        printer.pin_mut().set_resolution(300);
        Self {
            printer: RefCell::new(printer),
            webengine_view: Cell::new(std::ptr::null_mut()),
            in_print_preview: Cell::new(false),
            wait_for_result: RefCell::new(QEventLoop::new()),
        }
    }
}

impl ffi::PrinterHandler {
    pub fn new(parent: Option<Pin<&mut QObject>>) -> UniquePtr<Self> {
        match parent {
            Some(mut parent) => unsafe {
                ffi::new_printer_handler_with_parent(parent.as_mut().get_unchecked_mut() as *mut _)
            },
            None => ffi::new_printer_handler(),
        }
    }
    pub fn set_view(self: Pin<&mut Self>, mut view: Pin<&mut QWebEngineView>) {
        let view_ptr = unsafe { view.as_mut().get_unchecked_mut() as *mut QWebEngineView };
        self.webengine_view.set(view_ptr);

        let qt_thread = self.qt_thread();

        let qt_thread_clone = qt_thread.clone();
        let _ = view
            .as_mut()
            .on_print_requested(move |_view| {
                let _ = qt_thread_clone.queue(move |handler| {
                    let view = unsafe { &mut *(handler.webengine_view.get()) };
                    handler.on_print_requested(unsafe { Pin::new_unchecked(view) });
                });
            })
            .release();

        let qt_thread_clone = qt_thread.clone();
        let _ = view
            .as_mut()
            .on_print_finished(move |_view, success| {
                let _ = qt_thread_clone.queue(move |handler| {
                    let view = unsafe { &mut *(handler.webengine_view.get()) };
                    handler.on_print_finished(unsafe { Pin::new_unchecked(view) }, success);
                });
            })
            .release();
    }
    pub fn on_print_requested(self: Pin<&mut Self>, _view: Pin<&mut QWebEngineView>) {
        let view = unsafe {
            if self.webengine_view.get().is_null() {
                return;
            }
            &mut *self.webengine_view.get()
        };

        if !self.in_print_preview.get() {
            self.in_print_preview.set(true);
        } else {
            return;
        }

        unsafe {
            let mut preview = QPrintPreviewDialog::new(
                self.printer.borrow_mut().pin_mut().get_unchecked_mut(),
                Pin::new_unchecked(view),
            );
            let qt_thread = self.qt_thread();
            let _ = preview
                .pin_mut()
                .on_paint_requested(move |_dialog, _printer| {
                    let _ = qt_thread.queue(move |handler| {
                        handler.on_print_document();
                    });
                })
                .release();
            preview.pin_mut().exec();
        }

        self.in_print_preview.set(false);
    }
    pub fn on_print_finished(self: Pin<&mut Self>, _view: Pin<&mut QWebEngineView>, success: bool) {
        if !success {
            let mut painter = QPainter::new();
            let mut printer = self.printer.borrow_mut();
            if qpainter_begin(painter.pin_mut(), printer.pin_mut()) {
                let mut font = painter.font().clone();
                font.set_pixel_size(20);
                painter.pin_mut().set_font(&font);

                let position = cxx_qt_lib::QPointF::new(10.0, 25.0);
                let text = QString::from("Could not generate print preview.");
                qpainter_draw_text(painter.pin_mut(), &position, &text);

                qpainter_end(painter.pin_mut());
            }
        }
        let mut wait_for_result = self.wait_for_result.borrow_mut();
        wait_for_result.pin_mut().quit();
    }
    fn on_print_document(self: Pin<&mut Self>) {
        let printer_ptr: *mut QPrinter =
            unsafe { self.printer.borrow_mut().pin_mut().get_unchecked_mut() };

        let view = unsafe { &mut *(self.webengine_view.get()) };
        let pinned_view = unsafe { Pin::new_unchecked(view) };

        // print() is comment because it will really print the document..
        unsafe { pinned_view.print(printer_ptr) };

        let mut flags = QEventLoopProcessEventsFlags::new();
        let _ = flags.set_flag(QEventLoopProcessEventsFlag::ExcludeUserInputEvents, true);

        let wait_ptr: *mut QEventLoop = unsafe {
            self.wait_for_result
                .borrow_mut()
                .pin_mut()
                .get_unchecked_mut()
        };
        unsafe { Pin::new_unchecked(&mut *wait_ptr).exec(flags) };
    }
}
