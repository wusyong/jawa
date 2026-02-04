use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("jawa/qwebenginepermission.h");
        /// Represents a permission request for web content.
        type QWebEnginePermission = super::QWebEnginePermission;

        /// Returns whether this permission object is valid.
        #[cxx_name = "isValid"]
        fn is_valid(self: &QWebEnginePermission) -> bool;

        /// Returns the origin associated with the permission request.
        fn origin(self: &QWebEnginePermission) -> QUrl;

        /// Grants the permission request.
        fn grant(self: &QWebEnginePermission);

        /// Denies the permission request.
        fn deny(self: &QWebEnginePermission);

        /// Resets the permission decision.
        fn reset(self: &QWebEnginePermission);

        // TODO: permissionType, state
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct QWebEnginePermission {
    _cspec: MaybeUninit<usize>, // TODO: check actual size
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QWebEnginePermission {
    type Id = type_id!("QWebEnginePermission");
    type Kind = cxx::kind::Trivial;
}
