use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::WidgetPtr;
use cxx::memory::UniquePtrTarget;
use cxx_qt::casting::Upcast;

use cxx_qt::QObject;
pub use ffi::QTimer;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qtimer.h");

        /// Timer that emits `timeout()` at a specified interval.
        #[qobject]
        type QTimer;

        /// Starts the timer with the given interval in milliseconds.
        #[cxx_name = "start"]
        fn start_with_interval(self: Pin<&mut QTimer>, msec: i32);

        /// Starts the timer using the current interval.
        fn start(self: Pin<&mut QTimer>);

        /// Stops the timer.
        fn stop(self: Pin<&mut QTimer>);

        /// Returns whether the timer is active.
        #[cxx_name = "isActive"]
        fn is_active(self: &QTimer) -> bool;

        /// Sets the interval in milliseconds.
        #[cxx_name = "setInterval"]
        fn set_interval(self: Pin<&mut QTimer>, msec: i32);

        /// Returns the interval in milliseconds.
        fn interval(self: &QTimer) -> i32;

        /// Sets whether the timer is single-shot.
        #[cxx_name = "setSingleShot"]
        fn set_single_shot(self: Pin<&mut QTimer>, single_shot: bool);

        /// Returns whether the timer is single-shot.
        #[cxx_name = "isSingleShot"]
        fn is_single_shot(self: &QTimer) -> bool;

        #[cxx_name = "singleShot"]
        unsafe fn single_shot_raw(msec: i32, receiver: *const QObject, callback_id: u64);

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_timer() -> UniquePtr<QTimer>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_timer_with_parent(parent: *mut QObject) -> *mut QTimer;
    }

    #[namespace = "rust::cxxqtlib1"]
    extern "Rust" {
        fn single_shot_trampoline(callback_id: u64);
    }
}

impl ffi::QTimer {
    /// Creates a new timer without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_timer().into()
    }

    /// Creates a new timer with a parent object.
    pub fn new_with_parent<T: Upcast<QObject> + UniquePtrTarget>(
        parent: Pin<&mut T>,
    ) -> WidgetPtr<Self> {
        unsafe { ffi::new_timer_with_parent(parent.upcast_pin().get_unchecked_mut()).into() }
    }

    pub fn single_shot<F, T>(msec: i32, receiver: &T, functor: F)
    where
        F: FnOnce() + Send + 'static,
        T: Upcast<QObject> + UniquePtrTarget,
    {
        let callback_id = NEXT_SINGLE_SHOT_ID.fetch_add(1, Ordering::Relaxed);
        let map = SINGLE_SHOTS.get_or_init(|| Mutex::new(HashMap::new()));
        map.lock().unwrap().insert(callback_id, Box::new(functor));
        unsafe { ffi::single_shot_raw(msec, receiver.upcast(), callback_id) };
    }
}

static NEXT_SINGLE_SHOT_ID: AtomicU64 = AtomicU64::new(1);
static SINGLE_SHOTS: OnceLock<Mutex<HashMap<u64, Box<dyn FnOnce() + Send>>>> = OnceLock::new();

fn single_shot_trampoline(callback_id: u64) {
    if let Some(lock) = SINGLE_SHOTS.get() {
        let cb = lock.lock().unwrap().remove(&callback_id);
        if let Some(cb) = cb {
            cb();
        }
    }
}
